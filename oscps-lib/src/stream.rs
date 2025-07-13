//! # Stream

// NOTE: Temporarily disabled until the thermodynamics crate is thread-safe.
// use crate::thermodynamics::ThermoState;
use crate::simulation::BlockReference;

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
