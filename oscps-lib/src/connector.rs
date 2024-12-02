//! # Connector
//!

use crate::thermodynamics::ThermoState; 

/// # Stream
/// 
/// Struct to hold stream information
pub struct Stream {
    /// Stream id
    pub s_id : String,
    /// Instance of ThermoState struct that holds 
    pub thermo : Option<ThermoState>,
    /// block id for where the stream is coming from
    pub from_block : String,
    /// block id for where the stream is going to
    pub to_block : String
}


impl Stream {
    /// Constructor for 'Stream' struct
    pub fn new(id: String, from_blk_id : String, to_blk_id : String) -> Stream {
        Stream {
            s_id : id,
            thermo : None,
            from_block : from_blk_id,
            to_block : to_blk_id
        }
    }
}
