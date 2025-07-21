//! # Walker Ideal Model
//!
//! Contains implementation of the ``WalkerModel`` a type of ideal Equation of State Model that
//! also account for the vibrational and rotational modes of molecules in thermo calculations.

//External Imports
use std::sync::Arc;
use uom::si::f64::*;
use uom::si::molar_energy;
use uom::si::molar_heat_capacity;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;
use uom::si::volume;
use uom::si::ratio;

//Internal Imports
use crate::thermodynamics::EOSParams;
use crate::thermodynamics::ReferenceState;
use crate::thermodynamics::ideal::BaseEOSModel;
use crate::stream::ComponentData;

///# WalkerModel
///
///Another type of ideal equation of state model that will also account for vibrational and
///rotational modes of molecules in thermodynamic calculations.
pub struct WalkerModel {
    ///List of components (coming from 'Stream' struct)
    pub components: Arc<Vec<ComponentData>>,
    /// Overall Molecular weight (SingleParameter)
    pub molecular_weight : Arc<EOSParams<f64>>,
    /// Nrot - param for Walker model(SingleParameter)
    pub n_rot : Arc<EOSParams<i64>>, 
    /// θ1 - param for Walker model(SingleParameter)
    pub theta_1 : Arc<EOSParams<f64>>, 
    /// θ2 - param for Walker model(SingleParameter)
    pub theta_2 : Arc<EOSParams<f64>>,
    /// θ3 - param for Walker model(SingleParameter)
    pub theta_3 : Arc<EOSParams<f64>>, 
    /// θ4 - param for Walker model(SingleParameter)
    pub theta_4 : Arc<EOSParams<f64>>,
    /// deg1 - param for Walker model(SingleParameter)
    pub deg_1 : Arc<EOSParams<f64>>,
    /// deg2 - param for Walker model(SingleParameter)
    pub deg_2 : Arc<EOSParams<f64>>,
    /// deg3 - param for Walker model(SingleParameter)
    pub deg_3 : Arc<EOSParams<f64>>,
    /// deg4 - param for Walker model(SingleParameter)
    pub deg_4 : Arc<EOSParams<f64>>,
    /// reference state for the EOS model
    pub reference_state : Arc<ReferenceState> 

}

//Implementing the `BaseEOSModel` trait for the BasIdeal EOS Package. This will the default method
//for calculating the ideal helmholtz free energy.
impl BaseEOSModel for WalkerModel {
    fn components(&self) -> Arc<Vec<ComponentData>> {
        return Arc::clone(&self.components);
    }
    // Overriding the default function for the Walker Ideal Model
    fn ideal_helmholtz(&self, V: Volume, T: ThermodynamicTemperature) -> Energy {
        let rotational_modes = vec![self.theta_1.as_ref(), self.theta_2.as_ref(), self.theta_3.as_ref(), self.theta_4.as_ref()];
        let vibrational_modes = vec![self.deg_1.as_ref(), self.deg_2.as_ref(), self.deg_3.as_ref(), self.deg_4.as_ref()];


        let a_ideal = 0.0;


        return Energy::new::<energy::joule>(a_ideal);
    }
}

impl WalkerModel {
    //Constructor for the Walker Model
    pub fn new(species : Arc<Vec<ComponentData>>, ) -> Self {
        return WalkerModel { components: (), molecular_weight: (), n_rot: (), theta_1: (), theta_2: (), theta_3: (), theta_4: (), deg_1: (), deg_2: (), deg_3: (), deg_4: (), reference_state: () };
    }
}


