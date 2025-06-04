///# Pure Species Properties
/// This sub-module will contain the information regarding the cheracteristic properties of pure
/// species. This would include the molar mass (molecular weight), accentric factor, critical
/// temperature, critical pressure, critical compressibility, critical molar volume, and normal
/// boiling point.

#[warn(unused_imports)]
use crate::properties::*;
use uom::si::f64;
use std::sync::Arc;
// use oscps_db;

///#PureSpeciesProperties
///
///This will contain all the important properties for pure species
pub struct PureSpeciesProperties {
    pub species_obj_id: Arc<f64>,
    pub antoine_equation_constants: Vec<f64::Ratio>,
    pub critical_temperature: f64::ThermodynamicTemperature,
    pub molar_mass: f64::MolarMass,
    pub normal_boiling_point: f64::ThermodynamicTemperature,
    pub critical_molar_volume: f64::Volume,
    pub accentric_factor: f64::Ratio,
    pub compressibility_factor: f64::Ratio
}

///Functions to pull pure species properties from the database
    // Database will need to handle API calls to external sources for information currently in the
    // database.
impl PureSpeciesProperties {
    
} 
