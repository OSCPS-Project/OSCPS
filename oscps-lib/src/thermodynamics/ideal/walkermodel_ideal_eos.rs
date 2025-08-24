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
use uom::si::molar_mass;

//Internal Imports
use crate::thermodynamics::EOSParams;
use crate::thermodynamics::ReferenceState;
use crate::thermodynamics::ideal::BaseEOSModel;
use crate::thermodynamics::EOSGroupContributionParameters;
use crate::stream::ComponentData;

///# WalkerModel
///
///Another type of ideal equation of state model that will also account for vibrational and
///rotational modes of molecules in thermodynamic calculations.
pub struct WalkerModel {
    ///List of components (coming from 'Stream' struct)
    pub components: Arc<Vec<ComponentData>>,
    /// Overall Molecular weight (SingleParameter)
    pub molecular_weight : Arc<EOSParams>,
    /// Nrot - param for Walker model(SingleParameter)
    pub n_rot : Arc<EOSParams>, 
    /// θ1 - param for Walker model(SingleParameter)
    pub theta_1 : Arc<EOSParams>, 
    /// θ2 - param for Walker model(SingleParameter)
    pub theta_2 : Arc<EOSParams>,
    /// θ3 - param for Walker model(SingleParameter)
    pub theta_3 : Arc<EOSParams>, 
    /// θ4 - param for Walker model(SingleParameter)
    pub theta_4 : Arc<EOSParams>,
    /// deg1 - param for Walker model(SingleParameter)
    pub deg_1 : Arc<EOSParams>,
    /// deg2 - param for Walker model(SingleParameter)
    pub deg_2 : Arc<EOSParams>,
    /// deg3 - param for Walker model(SingleParameter)
    pub deg_3 : Arc<EOSParams>,
    /// deg4 - param for Walker model(SingleParameter)
    pub deg_4 : Arc<EOSParams>,
    /// reference state for the EOS model
    pub reference_state : Arc<ReferenceState>,
    /// group contributions
    pub eos_groups : Arc<EOSGroupContributionParameters>

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
    ///Constructor for the ``WalkerModel`` struct
    ///
    /// # Arguments
    /// * species - The list of species
    /// * molec_weight - The total molecular weight
    /// * The n_rot, theta, and deg are empirical values that are required for the thermodynamic
    /// calculations
    ///
    /// # Returns
    /// Instance of the ``WalkerModel`` struct
    pub fn new(
        species : Arc<Vec<ComponentData>>, 
        molec_weight : Arc<EOSParams>, 
        n_rot : Arc<EOSParams>, 
        theta_values : Arc<Vec<EOSParams>>, 
        deg_values : Arc<Vec<EOSParams>>, 
        reference_state : Arc<ReferenceState>,
        eos_groups : Arc<EOSGroupContributionParameters>)
        -> Self {
            return WalkerModel { 
                components: species, 
                molecular_weight: molec_weight, 
                n_rot: n_rot, 
                theta_1: Arc::new(theta_values[0]), 
                theta_2: Arc::new(theta_values[1]), 
                theta_3: Arc::new(theta_values[2]), 
                theta_4: Arc::new(theta_values[3]), 
                deg_1: Arc::new(deg_values[0]), 
                deg_2: Arc::new(deg_values[1]), 
                deg_3: Arc::new(deg_values[2]), 
                deg_4: Arc::new(deg_values[3]), 
                reference_state: reference_state,
                eos_groups: eos_groups };
    }
}


