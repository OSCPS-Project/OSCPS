//! # Thermodynamics
//!
//! This module will hold all the functions related to calculating themrodynamic properties for the
//! blocks and chemical species.

// use uom::si::f32::{Pressure, ThermodynamicTemperature};
use uom::si::f64::*; 
use uom::si::mass::kilogram;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::pressure::pascal;
use crate::component::Chemical;

pub enum ThermodynamicConstants {
    UniversalGasConstant, // R
    StandardTemperature,  // T_0
    StandardPressure,     // P_0
    AvogadroNumber,       // N_A
}

/// Enum for representing different types of thermodynamic constant values
pub enum ConstantValue {
    Pressure(Pressure),
    Temperature(ThermodynamicTemperature),
    Dimensionless(f64),
}

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
            ThermodynamicConstants::AvogadroNumber => {
                ConstantValue::Dimensionless(6.02214076e23)
            }
        }
    }
}

pub struct ThermoState {
    pub pressure: Pressure,                // Pressure in Pascals
    pub temperature: ThermodynamicTemperature, // Temperature in Kelvin
    pub mass_list: Vec<SpeciesListPair>, // Mole fractions, typically unitless
}

pub struct SpeciesListPair {
    pub chemical_species : Chemical,
    pub mass_quantity : Mass  
}

impl ThermoState {
    // Constructor for creating a ThermoState
    pub fn new(
        pressure: f64,      // in Pascals
        temperature: f64,   // in Kelvin
        mass_list: Vec<SpeciesListPair>,
    ) -> Self {
        ThermoState {
            pressure: Pressure::new::<pascal>(pressure),
            temperature: ThermodynamicTemperature::new::<kelvin>(temperature),
            mass_list : mass_list
        }
    }

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

    fn ideal_gas_pressure(&self, n: f64, t: f64, v: f64) -> f64 {
        const R: f64 = 8.314; // J/(mol·K)
        (n * R * t) / v
    }
}


#[cfg(test)]
mod thermo_tests {
    use super::*;
    use uom::si::pressure::pascal;
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::mass::kilogram;
    use crate::component::{Chemical, ChemicalProperties};
    
    #[test]
    fn test_create_thermo_state() {
        // Create some test data for ThermoMoleFrac (mole fractions)
        let water = Chemical {
            pubchem_obj: pubchem::Compound::new(962),
            properties: ChemicalProperties {
                molar_mass: 0.01801528, // kg/mol for water
                critical_temp: 647.1,   // K
                critical_pressure: 2206.0, // Pa
                acentric_factor: 0.344, // example
            },
        };
        let water_mass = Mass::new::<kilogram>(2.0);
        let water_species_pair = SpeciesListPair {
            chemical_species: water,
            mass_quantity: water_mass,
        };

        // Create ThermoState
        let thermo_state = ThermoState::new(
            101325.0, // pressure in Pascals (1 atm)
            298.15,   // temperature in Kelvin (25°C)
            vec![water_species_pair], // Example with one chemical
        );

        // Validate ThermoState
        assert_eq!(thermo_state.pressure.get::<pascal>(), 101325.0);
        assert_eq!(thermo_state.temperature.get::<kelvin>(), 298.15);
        assert_eq!(thermo_state.mass_list.len(), 1); // Should contain one mole fraction entry

        // Check that the mole fraction's chemical is correctly set
        assert_eq!(thermo_state.mass_list[0].chemical_species.get_pubchem_obj().cids().unwrap()[0], 962);
    }

    #[test]
    fn test_mass_fraction_calculation() {
        let water = Chemical {
            pubchem_obj: pubchem::Compound::new(962),
            properties: ChemicalProperties {
                molar_mass: 0.01801528, // kg/mol for water
                critical_temp: 647.1,   // K
                critical_pressure: 2206.0, // Pa
                acentric_factor: 0.344, // example
            },
        };

        let Anisdine = Chemical {
            pubchem_obj : pubchem::Compound::new(7732),
            properties : ChemicalProperties {
                molar_mass: 123.155, // g/mol, converting to kg/mol = 123.155 / 1000
                critical_temp: 592.0, // K (approximated)
                critical_pressure: 2.6e6, // Pa (approximated)
                acentric_factor: 0.24,  // (approximated)
            }
        };

        let water_mass = Mass::new::<kilogram>(2.0);
        let water_species_pair = SpeciesListPair {
            chemical_species: water,
            mass_quantity: water_mass,
        };

        let anisidine_mass = Mass::new::<kilogram>(8.0);
        let anisidine_species_pair = SpeciesListPair {
            chemical_species : Anisdine,
            mass_quantity : anisidine_mass
        };

        let therm_obj = ThermoState::new(
            101325.0,
            298.15, 
            vec![water_species_pair, anisidine_species_pair]
        );

        let mass_fraction = therm_obj.mass_frac(&therm_obj.mass_list[0].chemical_species).unwrap();




        assert!((mass_fraction - 0.2).abs() < 1e-6, "Mole fraction calculation failed"); // Should be 0.2
    }
}


