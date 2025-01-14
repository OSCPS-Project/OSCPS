//! # Connector
//!

use crate::thermodynamics::ThermoState; 

/// # Stream
/// 
/// Struct to hold stream information
pub struct Stream {
    // TODO: IDs must be unique within the flowsheet. Consider using integers
    // as IDs and having a separate field for the name of a connector. Adopt
    // a similar scheme for blocks.
    /// ID of the stream. 
    pub s_id : String,
    /// Instance of ThermoState struct that holds thermodynamic information.
    pub thermo : Option<ThermoState>,
    // TODO: Change these from strings to integers, or better yet, 
    // references to the source and destination blocks, to minimize
    // computation time spent on looking for sources and destinations.
    /// ID of source block
    pub from_block : String,
    /// ID of destination block
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
