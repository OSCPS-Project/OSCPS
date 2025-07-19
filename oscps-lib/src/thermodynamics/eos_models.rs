///#EOSModel
///
///This struct will represent the different thermodynamic equations of state

#[allow(unused_imports)]
use crate::thermodynamics::*;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use uom::si::amount_of_substance;
#[allow(unused_imports)]
use uom::si::energy;
#[allow(unused_imports)]
use uom::si::f64::*;
#[allow(unused_imports)]
use uom::si::molar_energy;
#[allow(unused_imports)]
use uom::si::molar_heat_capacity;
#[allow(unused_imports)]
use uom::si::pressure;
#[allow(unused_imports)]
use uom::si::ratio;
#[allow(unused_imports)]
use uom::si::thermodynamic_temperature;
#[allow(unused_imports)]
use uom::si::volume;

/// This struct will hold the chemical potential equation for each type of equation of state
/// Inspired by: https://github.com/ClapeyronThermo/Clapeyron.jl
pub struct EOSModel {}
