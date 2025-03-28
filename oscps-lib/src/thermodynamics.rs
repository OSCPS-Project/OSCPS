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

use uom::si::f32::MolarEnergy;
use uom::si::f32::MolarHeatCapacity;
use uom::si::f32::Ratio;
use uom::si::f64::MolarEnergy;
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
pub struct ComponentData {
    /// Chemical species
    pub chemical_species: Chemical, // will contain intrinsic properties of species
    /// Mass quantity
    pub mass_quantity: Mass,
    /// Molar quantity
    pub molar_quantity: AmountOfSubstance,
    ///volumetric quantity
    pub vol_quantity: Volume,
    /// partial pressure
    pub partial_pressure : Pressure,
}

#[allow(dead_code)]
/// # ThermoState
/// Returns a thermodynamic state, including pressure, temperature, and 
/// mole fractions.
/// This struct will be used for streams in the flow diagram
pub struct ThermoState {
    /// Pressure of the state.
    pub pressure: Option<Pressure>,                    // pressure
    /// Temperature of the state.
    pub temperature: Option<ThermodynamicTemperature>, // temperature
    /// List of mole fractions.
    pub mass_list: Vec<ComponentData>,//Information about each component within stream
    // Total Mass
    pub total_mass : Option<Mass>, // total mass in stream
    // Total Moles
    pub total_mol : Option<AmountOfSubstance>, // total moles in stream
    // Total Volume
    pub total_volume : Option<Volume>, // total volume in stream
    ///Thermo Package
    pub thermodynamic_package : Option<Box<dyn ThermoPackage>> // thermodynamics package 
}


#[allow(dead_code)]
/// Implementation of ThermoState
/// This struct holds the functionality to perform thermodynamic calculations for streams
impl ThermoState {
    /// Constructor for creating a ThermoState
    pub fn new() -> Self {
        ThermoState {
            pressure : None,
            temperature : None,
            mass_list : vec![],
            total_mass : None,
            total_mol : None,
            total_volume : None,
            thermodynamic_package : None
        }
    }
    /// this function will return the total mass for an individual stream
    fn calc_total_mass(&mut self) -> Mass {
        let mut mass_sum  = 0.0;
        for chem in &self.mass_list {
            mass_sum += chem.mass_quantity.get::<mass::kilogram>();
        }
        self.total_mass = Some(Mass::new::<mass::kilogram>(mass_sum));
        
        self.total_mass.unwrap()
    }
    /// this function will return the total moles for an individual stream
    fn calc_total_moles(&mut self) -> AmountOfSubstance {
        let mut mole_sum  = 0.0;
        for chem in &self.mass_list {
            mole_sum += chem.molar_quantity.get::<amount_of_substance::mole>();
        }
        self.total_mol = Some(AmountOfSubstance::new::<amount_of_substance::mole>(mole_sum));
        
        self.total_mol.unwrap()
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
    fn enthalpy(&self) -> MolarEnergy;
    ///Calculating the Entropy
    fn entropy(&self) -> MolarHeatCapacity;
    ///Calculate pressure
    fn pressure(&self) -> Pressure;
    ///Calculate volume
    fn volume(&self) -> Volume;
    ///Calculate temperature
    fn temperature(&self) -> ThermodynamicTemperature;
    ///Calculate vapor fractions
    fn vapor_fraction(&self) -> Ratio;
    ///Calculate heat capacity
    fn heat_capacity_const_pressure(&self) -> HeatCapacity;
    ///Calculate internal temperature
    fn internal_energy(&self) -> MolarEnergy;
    ///Calculate gibbs free energy
    fn gibbs_free_energy(&self) -> Energy;
}




#[cfg(test)]
mod thermo_tests {
    // use super::*;
    // use crate::component::{Chemical, ChemicalProperties};
    // use uom::si::mass::kilogram;
    // use uom::si::pressure::pascal;
    // use uom::si::thermodynamic_temperature::kelvin;
    // use std::{thread,time::Duration};

    // #[test]
    // ///Test case generates an instance of the 'ThermoState' struct
    // fn test_create_thermo_state() {
    //     // Create some test data for ThermoMoleFrac (mole fractions)
    //     let water = Chemical {
    //         pubchem_obj: pubchem::Compound::new(962),
    //         properties: ChemicalProperties {
    //             molar_mass: 0.01801528,    // kg/mol for water
    //             critical_temp: 647.1,      // K
    //             critical_pressure: 2206.0, // Pa
    //             acentric_factor: 0.344,    // example
    //         },
    //     };
    //     thread::sleep(Duration::from_secs(10));
    //     let water_mass = Mass::new::<kilogram>(2.0);
    //     let water_species_pair = SpeciesQuantityPair {
    //         chemical_species: water,
    //         mass_quantity: water_mass,
    //         const_a: 1.0,
    //         const_b: 1.0,
    //         const_c: 1.0,
    //         const_d: 0.0
    //     };

    //     // Create ThermoState
    //     let thermo_state = ThermoState::new(
    //         101325.0,                 // pressure in Pascals (1 atm)
    //         298.15,                   // temperature in Kelvin (25°C)
    //         vec![water_species_pair], // Example with one chemical
    //     );

    //     // Validate ThermoState
    //     assert_eq!(thermo_state.pressure.get::<pascal>(), 101325.0);
    //     assert_eq!(thermo_state.temperature.get::<kelvin>(), 298.15);
    //     assert_eq!(thermo_state.mass_list.len(), 1); // Should contain one mole fraction entry

    //     

    //     // Check that the mole fraction's chemical is correctly set
    //     assert_eq!(
    //         thermo_state.mass_list[0]
    //             .chemical_species
    //             .get_pubchem_obj()
    //             .cids()
    //             .unwrap()[0],
    //         962
    //     );
    // }

    // #[test]
    // ///Tests the mass fraction function within the 'ThermoState struct'
    // fn test_mass_fraction_calculation() {
    //     let water = Chemical {
    //         pubchem_obj: pubchem::Compound::new(962),
    //         properties: ChemicalProperties {
    //             molar_mass: 0.01801528,    // kg/mol for water
    //             critical_temp: 647.1,      // K
    //             critical_pressure: 2206.0, // Pa
    //             acentric_factor: 0.344,    // example
    //         },
    //     };
    //     thread::sleep(Duration::from_secs(10));

    //     let anisdine = Chemical {
    //         pubchem_obj: pubchem::Compound::new(7732),
    //         properties: ChemicalProperties {
    //             molar_mass: 123.155,      // g/mol, converting to kg/mol = 123.155 / 1000
    //             critical_temp: 592.0,     // K (approximated)
    //             critical_pressure: 2.6e6, // Pa (approximated)
    //             acentric_factor: 0.24,    // (approximated)
    //         },
    //     };
    //     thread::sleep(Duration::from_secs(10));
    //     
    //     let water_mass = Mass::new::<kilogram>(2.0);
    //     let water_species_pair = SpeciesQuantityPair {
    //         chemical_species: water,
    //         mass_quantity: water_mass,
    //     };

    //     let anisidine_mass = Mass::new::<kilogram>(8.0);
    //     let anisidine_species_pair = SpeciesQuantityPair {
    //         chemical_species: anisdine,
    //         mass_quantity: anisidine_mass,
    //     };

    //     let therm_obj = ThermoState::new(
    //         101325.0,
    //         298.15,
    //         vec![water_species_pair, anisidine_species_pair],
    //     );

    //     let mass_fraction = therm_obj
    //         .mass_frac(&therm_obj.mass_list[0].chemical_species)
    //         .unwrap();

    //     assert!(
    //         (mass_fraction - 0.2).abs() < 1e-6,
    //         "Mole fraction calculation failed"
    //     ); // Should be 0.2
    // }
}
