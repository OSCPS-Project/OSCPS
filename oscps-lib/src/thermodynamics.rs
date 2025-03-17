//! # Thermodynamics
//!
//! This module will hold all the functions related to calculating 
//! themrodynamic properties for the blocks and chemical species.
//!
//! TODO: All public items, including struct members, must be documented. Placeholder
//! documentation is in place, but more descriptive documentation should be 
//! implemented in the future.

pub mod ideal_gas_package;
use crate::component::Chemical;

use uom::si::f64::*;
use uom::si::mass;
use uom::si::molar_heat_capacity;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;
use uom::si::volume;


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
/// Implements values of thermodynamic constants.
impl ThermodynamicConstants {
    /// Returns the value of the thermodynamic constant with its appropriate type.
    pub fn value(&self) -> Box<dyn std::any::Any> {
        match self {
            ThermodynamicConstants::UniversalGasConstant => {
                let r = 8.314462618;
                let constant = Energy::new::<energy::joule>(r) / (ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(1.0)* AmountOfSubstance::new::<amount_of_substance::mole>(1.0));
                Box::new(constant)
            },
            ThermodynamicConstants::StandardTemperature => {
                Box::new(ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(273.15))
            }
            ThermodynamicConstants::StandardPressure => {
                Box::new(Pressure::new::<pressure::pascal>(101_325.0))
            },
            ThermodynamicConstants::AvogadroNumber => Box::new(6.02214076e23), //Units: particles/mole
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
    /// Molar quantity
    pub molar_quantity: AmountOfSubstance,
    ///volumetric quantity
    pub vol_quantity: Volume,
    ///Heat capacity Coefficient A
    pub const_a: f64,
    ///Heat capacity Coefficient B
    pub const_b: f64,
    ///Heat capacity Coefficient C
    pub const_c: f64,
    ///Heat capacity Coefficient D
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
    pub mass_list: Vec<SpeciesQuantityPair>,    // Mole fractions, typically unitless
    // Total Mass
    pub total_mass : Mass,
    // Total Moles
    pub total_mol : AmountOfSubstance,
    // Total Volume
    pub total_volume : Volume,
    ///Thermo Package
    pub thermodynamic_package : Box<dyn ThermoPackage> 
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
        thermo_package : Box<dyn ThermoPackage>
    ) -> Self {
        ThermoState {
            pressure: Pressure::new::<pressure::pascal>(pressure),
            temperature: ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(temperature),
            mass_list,
            thermodynamic_package : thermo_package
        }
    }

    /// Determine mass fraction
    pub fn mass_frac(&self, species: &Chemical) -> Option<f64> {
        let mut total_mass = 0.0;
        let mut component_mass = 0.0;

        for chem in &self.mass_list {
            total_mass += chem.mass_quantity.get::<mass::kilogram>();

            if let Some(cids) = Some(chem.chemical_species.pubchem_obj.cids().unwrap()[0]) {
                if cids == species.pubchem_obj.cids().unwrap_or_default()[0] {
                    component_mass = chem.mass_quantity.get::<mass::kilogram>();
                }
            }
        }

        match component_mass {
            0.0 => None,
            _ => Some(component_mass / total_mass),
        }
    }
    /// this function will return the total mass for an individual stream
    pub fn total_mass(& self) -> f64 {
        let mut mass_sum  = 0.0;
        for chem in &self.mass_list {
            mass_sum += chem.mass_quantity.get::<mass::kilogram>();
        }
        mass_sum
    }
}

///Thermodynamic Packages.
///
///#ThermoPackage
///Will be a common trait for all the thermodynamic packages
///Will include functions common to thermodynamic packages
///Will also enable to user to switch between thermodynamic packages within the ThermoState struct
///(the thermodynamic packages will be structs)

pub trait ThermoPackage{
    ///Calculating the Enthalpy
    fn enthalpy(&self) -> Energy;
    ///Calculating the Entropy
    fn entropy(&self) -> Energy;
    /// Calculate amount of moles
    fn calculate_moles(&self) -> AmountOfSubstance;
    ///Calculate pressure
    fn pressure(&self) -> Pressure;
    ///Calculate temperature
    fn temperature(&self) -> ThermodynamicTemperature;
    ///Calculate volume
    fn volume(&self) -> Volume;
    ///Calculate heat capacity
    fn heat_capacity(&self) -> HeatCapacity;
    ///Calculate internal temperature
    fn internal_energy(&self) -> Energy;
    ///Calculate gibbs free energy
    fn gibbs_free_energy(&self) -> Energy;
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
