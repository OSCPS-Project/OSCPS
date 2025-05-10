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

#[derive(Debug, Clone)]
struct Settings {
    // Add fields as needed
}

#[derive(Debug, Clone)]
struct SimulationState {
    // Add fields as needed
}

// #[derive(Debug)]
// enum Err {
//     BlockNotFound,
//     ConnectorNotFound,
//     BlockExists,
//     ConnectorExists,
//     Other(String),
// }

pub struct Simulation {
    blocks: HashMap<i32, Arc<RwLock<Box<dyn Block + Send>>>>,
    streams: HashMap<i32, Arc<RwLock<Box<Stream>>>>,
    settings: Settings,
    state: SimulationState,
}

impl Simulation {
    pub fn new(settings: Settings, state: SimulationState) -> Self {
        Self {
            blocks: HashMap::new(),
            streams: HashMap::new(),
            settings,
            state,
        }
    }

    //     pub fn add_block(&mut self, block_id: i32, block: Block) -> Result<(), Err> {
    //         if self.blocks.contains_key(&block_id) {
    //             return Err(Err::BlockExists);
    //         }
    //         self.blocks.insert(block_id, block);
    //         Ok(())
    //     }

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
