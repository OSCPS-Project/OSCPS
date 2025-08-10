//! # Thermodynamics
//!
//! This module will hold all the functions related to calculating 
//! themrodynamic properties for the blocks and chemical species.
//!
//! Inspired by: https://github.com/ClapeyronThermo/Clapeyron.jl

///Importing EOSModels
pub mod ideal;
pub mod cubic;

///Importing Supporting Thermodynamic Methods

/// Importing Chemical Properties Used by Thermo Packages
use crate::properties::Chemical;
use crate::stream::ComponentData;

///Importing External Packages
use std::sync::Arc;
use uom::si::f64::*;
use uom::si::heat_capacity;
use uom::si::mass;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;
use nalgebra::DMatrix;

#[allow(dead_code)]
///# ThermodynamicConstants
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
    /// Boltzmann Constant
    BoltzmannConstant     // k_B
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
                Box::new(Pressure::new::<pressure::pascal>(101325.0))
            },
            ThermodynamicConstants::AvogadroNumber => Box::new(6.02214076e23), //Units: particles/mole
            ThermodynamicConstants::BoltzmannConstant => Box::new(HeatCapacity::new::<heat_capacity::joule_per_kelvin>(1.380_649e-23))
        }
    }
}

///# EOSParams
///
/// Enumeration that will hold the single, double, and associating parameters for the different
/// equation of state packages
#[derive(Clone, Copy)]
pub enum EOSParams<T> {
    SingleParameter(T),
    DoubleParameter(T),
    AssociatingParameter(T)
}


///# ReferenceState
///
/// Enumeration that will contain the types of reference states that will be used by the different
/// equation of state models.
pub enum ReferenceState{}


///# ReferenceStateParams
/// 
/// Will contain the parameters that will be part of each enum member in the ``ReferenceState``
/// enumeration.
pub struct ReferenceStateParams{}


///# EOSGroupTypes
///
/// Enumeration to hold the type of groups used within ``EOSGroups``
pub enum EOSGroupTypes{}

///# EOSGroups
///
/// Contain struct definition for Group Contributions. Specifically, this struct will perform
/// calculations to study properties of groups (such as CH2, CH3, etc within a long-chain
/// hydrocarbon) and the interactions between groups
///
/// Derived from ClapeyronThermo (GroupParams.jl)
///
pub struct EOSGroups {
    pub components : Arc<ComponentData>,
    pub molecular_weight : Arc<EOSParams<MolarMass>>,
    pub groups : Arc<Vec<Vec<String>>>,
    pub n_groups : Arc<Vec<Vec<i64>>>,
    pub n_intergroups : Arc<Vec<DMatrix<i64>>>,
    pub i_groups : Arc<Vec<Vec<i64>>>,
    pub flattened_groups : Arc<Vec<String>>,
    pub n_flattened_groups : Arc<Vec<Vec<String>>>,
    pub sourcecsvs : Arc<Vec<String>>

}


#[cfg(test)]
mod tests_thermo_module {}
