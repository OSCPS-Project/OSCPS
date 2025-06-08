//! # Thermodynamics
//!
//! This module will hold all the functions related to calculating 
//! themrodynamic properties for the blocks and chemical species.

///Importing EOSModels
pub mod eos_models;

/// Importing chemical properties
use crate::properties::Chemical;

///Importing External Packages
use uom::si::f64::*;
use uom::si::mass;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;

#[allow(dead_code)]
/// #ThermodynamicConstants
///
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

///Thermodynamic Packages.
///
///#MaxwellRelations
///
///Will be a common trait for all the thermodynamic packages and will include common functions.
///Will also enable to user to switch between thermodynamic packages within the StreamThermoState struct
///
///The thermodynamic pacakges can be used by the blocks for any relevant calculations
///
///For calculations, the thermodynamic packages will call upon the property struct for relevant
///info.
///
///TODO: Currently the rust std::autodiff is still experimental. Need to wait for this release. In
///the meantime, we will either manually write out the derivatives or use a third party autdiff
///package (the third party is: https://crates.io/crates/autodiff)
pub trait MaxwellRelations{
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
    fn heat_capacity_const_pressure(&self) -> MolarHeatCapacity;
    ///Calculate internal temperature
    fn internal_energy(&self) -> MolarEnergy;
    ///Calculate gibbs free energy
    fn gibbs_free_energy(&self) -> Energy;
}

#[cfg(test)]
mod thermo_tests {}
