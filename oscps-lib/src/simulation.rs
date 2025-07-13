//! # Simulation
//!
//! Allows for the construction of a simulation object. TODO: Implement this.

use crate::blocks::{Block, Mixer};
use crate::stream::Stream;
// use std::collections::HashMap;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
/// An Arc, RwLock, Box reference for threadsafe Block interactions.
pub type BlockReference = Arc<RwLock<Box<dyn Block + Send + Sync>>>;
/// An Arc, RwLock, Box reference for threadsafe Stream interactions.
pub type StreamReference = Arc<RwLock<Box<Stream>>>;

/// Used to tell functions what type of block to add.
pub enum BlockType {
    /// Mix multiple streams into a single output stream.
    Mixer,
    /// Source for streams.
    Source,
    /// Sink for streams.
    Sink,
}

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

impl SimulationState {
    /// Create a new SimulationState.
    pub fn new() -> Self {
        return SimulationState {};
    }
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
#[allow(dead_code)]
pub struct Simulation {
    /// Stores all the blocks in the simulation
    blocks: BTreeMap<u64, BlockReference>,
    /// Stores all the streams in the simulation
    streams: BTreeMap<u64, StreamReference>,
    /// Stores simulation settings
    settings: Settings,
    /// Stores the state of the simlation
    state: SimulationState,
}

impl Simulation {
    /// Create a new simulation
    pub fn new(settings: Settings) -> Self {
        Self {
            blocks: BTreeMap::new(),
            streams: BTreeMap::new(),
            settings,
            state: SimulationState::new(),
        }
    }

    /// Adds a block to the simulation and returns the ID of
    /// the block.
    #[allow(dead_code)]
    pub fn add_block(&mut self, block: BlockType) -> u64 {
        // Start with a block id of 1.
        let mut id = 1;
        while self.blocks.contains_key(&id) {
            id += 1;
        }

        match block {
            BlockType::Mixer => {
                self.blocks
                    .insert(id, Arc::new(RwLock::new(Box::new(Mixer::new()))));
            }
            BlockType::Source => {
                todo!()
            }
            BlockType::Sink => {
                todo!()
            }
        }

        return id;
    }

    /// Adds a stream to the simulation and returns the ID of
    /// the stream.
    #[allow(dead_code)]
    pub fn add_stream(&mut self, from: BlockReference, to: BlockReference) -> u64 {
        // Start with a stream ID of 1.
        let mut id = 1;
        while self.streams.contains_key(&id) {
            id += 1;
        }
        self.streams
            .insert(id, Arc::new(RwLock::new(Box::new(Stream::new(from, to)))));
        return id;
    }

    // /// Add a block to the simulation.
    // fn add_block(
    //     &mut self,
    //     block_id: u64,
    //     block: Arc<RwLock<Box<dyn Block + Send>>>,
    // ) -> Result<(), Err> {
    //     if self.blocks.contains_key(&block_id) {
    //         return Err(Err::BlockExists);
    //     }
    //     self.blocks.insert(block_id, block);
    //     Ok(())
    // }

    //     pub fn add_connector(&mut self, connector_id: u64, connector: Connector) -> Result<(), Err> {
    //         if self.connectors.contains_key(&connector_id) {
    //             return Err(Err::ConnectorExists);
    //         }
    //         self.connectors.insert(connector_id, connector);
    //         Ok(())
    //     }

    //     pub fn remove_block(&mut self, block_id: u64) -> Result<(), Err> {
    //         if self.blocks.remove(&block_id).is_none() {
    //             return Err(Err::BlockNotFound);
    //         }
    //         Ok(())
    //     }

    //     pub fn remove_connector(&mut self, connector_id: u64) -> Result<(), Err> {
    //         if self.connectors.remove(&connector_id).is_none() {
    //             return Err(Err::ConnectorNotFound);
    //         }
    //         Ok(())
    //     }
}
