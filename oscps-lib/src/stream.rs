//! # Stream

// NOTE: Temporarily disabled until the thermodynamics crate is thread-safe.
// use crate::thermodynamics::StreamThermoState;
use crate::simulation::BlockReference;
use crate::properties::Chemical;
use crate::thermodynamics::ideal::base_ideal_eos::BaseEOSModel;
///Importing External Packages
use uom::si::f64::*;
use uom::si::mass;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;

/// # Stream
///
/// Struct to hold stream information
pub struct Stream {
    /// Instance of ThermoState struct that holds thermodynamic information.
    // pub thermo: Option<ThermoState>, // HACK: Temporarily disable to enable thread-safety.
    /// ID of source block
    pub from: BlockReference,
    /// ID of destination block
    pub to: BlockReference,
}

impl Stream {
    /// Constructor for 'Stream' struct
    pub fn new(from: BlockReference, to: BlockReference) -> Stream {
        Stream { from, to }
    }
}

#[allow(dead_code)]
/// #ComponentData
///
/// Species list for a stream
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
/// # StreamThermoState
/// 
/// This struct will be used for performing thermodynamic calculations for the streams in the flow
/// diagram.
pub struct StreamThermoState {
    /// Pressure of the state.
    pub pressure: Option<Pressure>,                    // pressure
    /// Temperature of the state.
    pub temperature: Option<ThermodynamicTemperature>, // temperature
    /// List of mole fractions.
    pub mass_list: Vec<ComponentData>,//Information about each component within stream
    /// Total Mass
    pub total_mass : Option<Mass>, // total mass in stream
    /// Total Moles
    pub total_mol : Option<AmountOfSubstance>, // total moles in stream
    /// Total Volume
    pub total_volume : Option<Volume>, // total volume in stream
    ///Thermo Package
    pub thermodynamic_package : Option<Box<dyn BaseEOSModel>> // thermodynamics package 
}
