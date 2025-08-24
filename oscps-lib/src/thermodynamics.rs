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
use std::collections::HashMap;

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

///# EOSGroupContributionTypes
///
/// Enumeration to hold the type of groups used within ``EOSGroupContributionParameters``
pub enum EOSGroupContributionTypes{}

///# EOSGroupContributionParameters
///
/// Contain struct definition for Group Contributions. Specifically, this struct will perform
/// calculations to study properties of groups (such as CH2, CH3, etc within a long-chain
/// hydrocarbon) and the interactions between groups.
///
/// This can be used to estimate thermodynamic properties using a molecule's functional groups.
///
/// Derived from ClapeyronThermo (GroupParams.jl)
///
pub struct EOSGroupContributionParameters {
    pub group_type : Arc<EOSGroupContributionTypes>,
    pub components : Arc<Vec<ComponentData>>,
    pub groups : Arc<Vec<String>>,
    pub n_groups : Arc<Vec<i64>>,
    pub n_intergroups : Arc<Vec<DMatrix<i64>>>,
    pub i_groups : Arc<Vec<Vec<i64>>>,
    pub flattened_groups : Arc<Vec<String>>,
    pub n_flattened_groups : Arc<Vec<Vec<i64>>>,
    pub sourcecsvs : Arc<Vec<String>>
}

impl EOSGroupContributionParameters {
    ///Constructor for ``EOSGroupsContributionParameters``
    pub fn new(
        group_type : Arc<EOSGroupContributionTypes>,
        components : Arc<Vec<ComponentData>>,
        raw_groups : Arc<HashMap<String, i64>>,
        sourcecsvs : Arc<Vec<String>>,
    ) ->Self {
        let str_group = raw_groups.keys().cloned().collect(); // getting the group names
        let group_amts = raw_groups.values().cloned().collect(); // getting the group values
        let flattened_groups : Vec<String> = Vec::new();
        let i_groups : Vec<Vec<i64>> = Vec::new();
        let n_flattened_groups : Vec<Vec<i64>> = Vec::new();
        let empty_intergroup: DMatrix<i64> = DMatrix::zeros(0, 0);
        let n_intergroups: Vec<DMatrix<i64>> = vec![empty_intergroup.clone(); components.len()];

        return EOSGroupContributionParameters { 
            group_type: group_type, 
            components: components, 
            groups: Arc::new(str_group), 
            n_groups: Arc::new(group_amts), 
            n_intergroups: Arc::new(n_intergroups), 
            i_groups: Arc::new(i_groups), 
            flattened_groups: Arc::new(flattened_groups), 
            n_flattened_groups: Arc::new(n_flattened_groups),
            sourcecsvs: sourcecsvs 
        };
    }
    /// Retrieving intramolecular group interactions for thermo property calcs
    fn compute_intramolecular_gc(&mut self) {

    }
}


#[cfg(test)]
mod tests_thermo_module {}
