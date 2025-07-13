///#EOSModel
///
///This struct will represent the different thermodynamic equations of state 


use crate::thermodynamics::*;
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

//Specific EOS Models using this Trait
pub mod base_ideal_eos;

trait BaseEOSModel {
   //Function to calculate the Ideal Helmholtz Energy
   fn ideal_helmholtz() {}
}



