//! # Base Ideal Equation of State
//!
//! This will contain the very basic Equation of State Package

use std::sync::Arc;
use crate::thermodynamics::EOSParams;
use crate::thermodynamics::ReferenceState;
use crate::thermodynamics::ideal::BaseEOSModel;
use crate::stream::ComponentData;

///# BaseIdeal
///
///This is the base thermodynamics package.
///
///Will only provide the list of components and it will calculate the 
///ideal helmholtz free energy.
pub struct BaseIdeal {
    ///List of components (coming from 'Stream' struct)
    pub components: Arc<Vec<ComponentData>>,
    pub reference_state : Arc<ReferenceState> 
}

//Implementing the `BaseEOSModel` trait for the BasIdeal EOS Package. This will the default method
//for calculating the ideal helmholtz free energy.
impl BaseEOSModel for BaseIdeal{
    fn components(&self) -> Arc<Vec<ComponentData>> {
        return Arc::clone(&self.components);
    }
}


