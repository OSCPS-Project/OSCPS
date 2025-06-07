//! # Simulation
//!
//! Allows for the construction of a simulation object. TODO: Implement this.

use crate::blocks::Block;
use crate::stream::Stream;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// fn compute_outlet_phase_fractions(&self) {

// }

// fn compute_outlet_temperature(&self) {

// }

// fn compute_outlet_pressure(&self) {

// }

/// A struct for storing settings of the simulation
#[derive(Debug, Clone, Default)]
pub struct Settings {
    // Add fields as needed
}

/// A struct for storing the current state of the simulation
#[derive(Debug, Clone, Default)]
pub struct SimulationState {
    // Add fields as needed
}

/// An enum used to represent errors.
#[derive(Debug)]
pub enum Err {
    /// Error when a block is not found
    BlockNotFound,
    /// Error when a connector is not found
    ConnectorNotFound,
    /// Error when a block with a matching ID is already in the simulation
    BlockExists,
    /// Error when a connector with a matching ID is already in the simulation
    ConnectorExists,
    /// Any other error
    Other(String),
}

/// The Simulation struct stores information pertaining to blocks and streams
#[derive(Default)]
pub struct Simulation {
    /// Stores all the blocks in the simulation
    blocks: HashMap<i32, Arc<RwLock<Box<dyn Block + Send>>>>,
    /// Stores all the streams in the simulation
    streams: HashMap<i32, Arc<RwLock<Box<Stream>>>>,
    /// Stores simulation settings
    settings: Settings,
    /// Stores the state of the simlation
    state: SimulationState,
}

impl Simulation {
    /// Create a new simultion
    pub fn new(settings: Settings, state: SimulationState) -> Self {
        Self {
            blocks: HashMap::new(),
            streams: HashMap::new(),
            settings,
            state,
        }
    }

    /// Add a block to the simulation.
    pub fn add_block(
        &mut self,
        block_id: i32,
        block: Arc<RwLock<Box<dyn Block + Send>>>,
    ) -> Result<(), Err> {
        if self.blocks.contains_key(&block_id) {
            return Err(Err::BlockExists);
        }
        self.blocks.insert(block_id, block);
        Ok(())
    }

    //     pub fn add_connector(&mut self, connector_id: i32, connector: Connector) -> Result<(), Err> {
    //         if self.connectors.contains_key(&connector_id) {
    //             return Err(Err::ConnectorExists);
    //         }
    //         self.connectors.insert(connector_id, connector);
    //         Ok(())
    //     }

    //     pub fn remove_block(&mut self, block_id: i32) -> Result<(), Err> {
    //         if self.blocks.remove(&block_id).is_none() {
    //             return Err(Err::BlockNotFound);
    //         }
    //         Ok(())
    //     }

    //     pub fn remove_connector(&mut self, connector_id: i32) -> Result<(), Err> {
    //         if self.connectors.remove(&connector_id).is_none() {
    //             return Err(Err::ConnectorNotFound);
    //         }
    //         Ok(())
    //     }
}
