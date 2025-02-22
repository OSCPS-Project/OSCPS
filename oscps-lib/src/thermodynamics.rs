//! # Thermodynamics
//!
//! This module will hold all the functions related to calculating 
//! themrodynamic properties for the blocks and chemical species.
//!
//! TODO: All public items, including struct members, must be documented. Placeholder
//! documentation is in place, but more descriptive documentation should be 
//! implemented in the future.

use crate::component::Chemical;
use uom::si::{f64::*, Quantity};
use uom::si::mass::kilogram;
use uom::si::pressure::pascal;
// use uom::si::temperature_interval::kelvin;
use uom::si::thermodynamic_temperature::kelvin;

#[allow(dead_code)]
/// Struct for storing physical constants for thermodynamics.
/// TODO: Reimplement the use of uom for dimensional analysis.
pub enum ThermodynamicConstants {
    /// The Universal gas constant in J/(mol*K)
    UniversalGasConstant, // J/(mol*K)
    /// Standard temperature in K
    StandardTemperature,  // T_0
    /// Standard pressure in Pa
    StandardPressure,     // P_0
    /// Avogadro's number in mol^-1
    AvogadroNumber,       // N_A
}

#[allow(dead_code)]
/// Enum for representing different types of thermodynamic constant values
pub enum ConstantValue {
    /// Pressure value
    Pressure(Pressure),
    /// Temperature value
    Temperature(ThermodynamicTemperature),
    /// Dimensionless value
    Dimensionless(f64),
}

#[allow(dead_code)] 
/// Implements values of thermodynamic constants.
impl ThermodynamicConstants {
    /// Returns the value of the thermodynamic constant with its appropriate type.
    pub fn value(&self) -> ConstantValue {
        match self {
            ThermodynamicConstants::UniversalGasConstant => {
                ConstantValue::Pressure(Pressure::new::<pascal>(8.314462618))
            }
            ThermodynamicConstants::StandardTemperature => {
                ConstantValue::Temperature(ThermodynamicTemperature::new::<kelvin>(273.15))
            }
            ThermodynamicConstants::StandardPressure => {
                ConstantValue::Pressure(Pressure::new::<pascal>(101_325.0))
            }
            ThermodynamicConstants::AvogadroNumber => ConstantValue::Dimensionless(6.02214076e23),
        }
    }
}


#[allow(dead_code)]
/// Species list
pub struct SpeciesQuantityPair {
    /// Chemical species
    pub chemical_species: Chemical,
    /// Mass quantity
    pub mass_quantity: Mass,

    ///Heat capacity constants (for enthalpy calculations)
    pub const_a: f64,
    pub const_b: f64,
    pub const_c: f64,
    pub const_d: f64
}

#[allow(dead_code)]
/// # ThermoState
/// Returns a thermodynamic state, including pressure, temperature, and 
/// mole fractions.
/// This struct will be used for streams in the flow diagram
pub struct ThermoState {
    /// Pressure of the state.
    pub pressure: Pressure,                    // Pressure in Pascals
    /// Temperature of the state.
    pub temperature: ThermodynamicTemperature, // Temperature in Kelvin
    /// List of mole fractions.
    pub mass_list: Vec<SpeciesQuantityPair>,       // Mole fractions, typically unitless
}


#[allow(dead_code)]
/// Implementation of ThermoState
/// This struct holds the functionality to perform thermodynamic calculations for streams
impl ThermoState {
    /// Constructor for creating a ThermoState
    pub fn new(
        pressure: f64,    // in Pascals
        temperature: f64, // in Kelvin
        mass_list: Vec<SpeciesQuantityPair>,
    ) -> Self {
        ThermoState {
            pressure: Pressure::new::<pascal>(pressure),
            temperature: ThermodynamicTemperature::new::<kelvin>(temperature),
            mass_list,
        }
    }

    /// Determine mass fraction
    pub fn mass_frac(&self, species: &Chemical) -> Option<f64> {
        let mut total_mass = 0.0;
        let mut component_mass = 0.0;

        for chem in &self.mass_list {
            total_mass += chem.mass_quantity.get::<kilogram>();

            if let Some(cids) = Some(chem.chemical_species.pubchem_obj.cids().unwrap()[0]) {
                if cids == species.pubchem_obj.cids().unwrap_or_default()[0] {
                    component_mass = chem.mass_quantity.get::<kilogram>();
                }
            }
        }

        match component_mass {
            0.0 => None,
            _ => Some(component_mass / total_mass),
        }
    }

    /// Determine ideal gas pressure
    fn ideal_gas_pressure(&self, n: f64, t: f64, v: f64) -> f64 {
        const R: f64 = 8.314; // J/(mol·K)
        (n * R * t) / v
    }
    
    /// this function will return the total mass for an individual stream
    pub fn total_mass(& self) -> f64 {
        let mut mass_sum  = 0.0;
        for chem in &self.mass_list {
            mass_sum += chem.mass_quantity.get::<kilogram>();
        }
        mass_sum
    }

    /// This function will provide the enthalpy of an individual stream
    pub fn enthalpy(&self) -> f64 {
        let mut total_enthalpy = 0.0;
        let t_ref = 298.15; //reference temperature 
        let h_ref = 0.0; //Reference enthalpy
        

        // Need to run a for loop where I calculate the enthalpy of each species and then add it to
        // the variable 'total_enthalpy'
        // ASSUMPTIONS CURRENTLY MADE:
            // No enthalpy from phase change or pressure changes
            // when working with gases, assume that they are ideal gases
            // Tref = 298 K & Pref = 101.325 kPa
            // Href = 0 
        
        for chem in &self.mass_list {
            let mut cp_ref = 0.0;
            let mut cp_t = 0.0;
            if(chem.const_c != 0.0){
                cp_ref = chem.const_a * t_ref + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * t_ref.powi(2);
                cp_t = chem.const_a * self.temperature.get::<kelvin>() + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * self.temperature.get::<kelvin>().powf(2.0) + (1.0 / 3.0) * (chem.const_c / (10.0f64.powf(6.0))) * self.temperature.get::<kelvin>().powf(3.0);
            }
            else{
                cp_ref = chem.const_a * t_ref + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * t_ref.powi(2) + (-1.0) * (chem.const_d / (10.0f64.powf(-5.0))) * t_ref.powi(-1);
                cp_t = chem.const_a * self.temperature.get::<kelvin>() + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * self.temperature.get::<kelvin>().powf(2.0) + (-1.0) * (chem.const_d / (10.0f64.powf(-5.0))) * self.temperature.get::<kelvin>().powf(-1.0);
            }
            let species_enthalpy = h_ref + (cp_t - cp_ref);
            total_enthalpy += species_enthalpy;
        }

        total_enthalpy
    }
}


#[cfg(test)]
mod thermo_tests {
    use super::*;
    use crate::component::{Chemical, ChemicalProperties};
    use uom::si::mass::kilogram;
    use uom::si::pressure::pascal;
    use uom::si::thermodynamic_temperature::kelvin;
    use std::{thread,time::Duration};

    #[test]
    ///Test case generates an instance of the 'ThermoState' struct
    fn test_create_thermo_state() {
        // Create some test data for ThermoMoleFrac (mole fractions)
        let water = Chemical {
            pubchem_obj: pubchem::Compound::new(962),
            properties: ChemicalProperties {
                molar_mass: 0.01801528,    // kg/mol for water
                critical_temp: 647.1,      // K
                critical_pressure: 2206.0, // Pa
                acentric_factor: 0.344,    // example
            },
        };
        thread::sleep(Duration::from_secs(10));
        let water_mass = Mass::new::<kilogram>(2.0);
        let water_species_pair = SpeciesQuantityPair {
            chemical_species: water,
            mass_quantity: water_mass,
            const_a: 1.0,
            const_b: 1.0,
            const_c: 1.0,
            const_d: 0.0
        };

        // Create ThermoState
        let thermo_state = ThermoState::new(
            101325.0,                 // pressure in Pascals (1 atm)
            298.15,                   // temperature in Kelvin (25°C)
            vec![water_species_pair], // Example with one chemical
        );

        // Validate ThermoState
        assert_eq!(thermo_state.pressure.get::<pascal>(), 101325.0);
        assert_eq!(thermo_state.temperature.get::<kelvin>(), 298.15);
        assert_eq!(thermo_state.mass_list.len(), 1); // Should contain one mole fraction entry

        

        // Check that the mole fraction's chemical is correctly set
        assert_eq!(
            thermo_state.mass_list[0]
                .chemical_species
                .get_pubchem_obj()
                .cids()
                .unwrap()[0],
            962
        );
    }

    #[test]
    ///Tests the mass fraction function within the 'ThermoState struct'
    fn test_mass_fraction_calculation() {
        let water = Chemical {
            pubchem_obj: pubchem::Compound::new(962),
            properties: ChemicalProperties {
                molar_mass: 0.01801528,    // kg/mol for water
                critical_temp: 647.1,      // K
                critical_pressure: 2206.0, // Pa
                acentric_factor: 0.344,    // example
            },
        };
        thread::sleep(Duration::from_secs(10));

        let anisdine = Chemical {
            pubchem_obj: pubchem::Compound::new(7732),
            properties: ChemicalProperties {
                molar_mass: 123.155,      // g/mol, converting to kg/mol = 123.155 / 1000
                critical_temp: 592.0,     // K (approximated)
                critical_pressure: 2.6e6, // Pa (approximated)
                acentric_factor: 0.24,    // (approximated)
            },
        };
        thread::sleep(Duration::from_secs(10));
        
        let water_mass = Mass::new::<kilogram>(2.0);
        let water_species_pair = SpeciesQuantityPair {
            chemical_species: water,
            mass_quantity: water_mass,
        };

        let anisidine_mass = Mass::new::<kilogram>(8.0);
        let anisidine_species_pair = SpeciesQuantityPair {
            chemical_species: anisdine,
            mass_quantity: anisidine_mass,
        };

        let therm_obj = ThermoState::new(
            101325.0,
            298.15,
            vec![water_species_pair, anisidine_species_pair],
        );

        let mass_fraction = therm_obj
            .mass_frac(&therm_obj.mass_list[0].chemical_species)
            .unwrap();

        assert!(
            (mass_fraction - 0.2).abs() < 1e-6,
            "Mole fraction calculation failed"
        ); // Should be 0.2
    }
}
