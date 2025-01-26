//! # Simulation
//!
//! Allows for the construction of a simulation object. TODO: Implement this.

use crate::blocks;
use crate::blocks::Block;
// use crate::connector::Stream;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Used for initializing blocks
pub enum BlockType {
    /// A Mixer
    Mixer,
    /// A Separator
    Separator,
}

// fn compute_outlet_phase_fractions(&self) {

// }

// fn compute_outlet_temperature(&self) {

// }

// fn compute_outlet_pressure(&self) {

// }

/// A struct for storing simulation settings information.
#[derive(Debug, Clone)]
pub struct Settings {
    simulation_name: String,
}

impl Settings {
    /// Create a Settings struct.
    pub fn new(simulation_name: String) -> Self {
        return Settings { simulation_name };
    }
}

// /// Stores the current state of the simulation.
// #[derive(Clone)]
// struct SimulationState {
//     dynamic_mode: bool, // Stores whether the simulation is running in dynamic mode or not
//     blocks: HashMap<i64, Box<dyn Block>>,
//     settings: Settings,
// }

// impl SimulationState {
//     fn new(dynamic_mode: bool, settings: Settings) -> Self {
//         SimulationState {
//             dynamic_mode,
//             blocks: HashMap::new(),
//             settings,
//         }
//     }
// }

// #[derive(Debug)]
// enum Err {
//     BlockNotFound,
//     ConnectorNotFound,
//     BlockExists,
//     ConnectorExists,
//     Other(String),
// }

/// # Simluation
///
/// A struct which stores and controls the relationship between streams and
/// blocks, as well as the current state of the simulation.
pub struct Simulation {
    blocks: HashMap<u64, Box<dyn Block>>,
    settings: Settings,
    next_id: u64,
}

impl Simulation {
    /// Create a new Simulation
    pub fn new(settings: Settings) -> Self {
        Self {
            next_id: 0,
            blocks: HashMap::new(),
            settings,
        }
    }

    /// Add a block to the simulation, returns a block ID upon success.
    pub fn add_block(&mut self, block_type: BlockType) -> Result<u64, &str> {
        return match block_type {
            BlockType::Mixer => {
                let id = self.gen_id();
                self.blocks
                    .entry(id)
                    .or_insert(Box::new(blocks::Mixer::new(id)));
                Ok(id)
            }
            BlockType::Separator => {
                let id = self.gen_id();
                self.blocks
                    .entry(id)
                    .or_insert(Box::new(blocks::Mixer::new(id)));
                Ok(id)
            }
        };
    }

    /// Fetch immutable reference block with the given id
    pub fn get_block(&self, block_id: u64) -> Result<Option<&Box<dyn Block>>, &str> {
        match self.blocks.get(&block_id) {
            Some(block) => Ok(Some(block)),
            None => Err("Block id {block_id} does not exist."),
        }
    }

    /// Add a connector which connects a given input and output block ids.
    // pub fn add_connector(block_1: u64, block_2: u64) -> Result<Ok(), &str> {
    //     todo!()
    // }

    fn gen_id(&mut self) -> u64 {
        let result = self.next_id;
        self.next_id += 1;
        return result;
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

#[cfg(test)]
mod simulation_tests {
    use super::*;

    #[test]
    fn initialize_simulation() {
        let settings = Settings::new("Test simulation".to_string());
        assert_eq!(settings.simulation_name, "Test simulation");
        let simulation = Simulation::new(settings);
        assert_eq!(simulation.settings.simulation_name, "Test simulation");
        assert_eq!(simulation.blocks.len(), 0);
    }

    #[test]
    fn add_blocks() {
        let mut simulation = Simulation::new(Settings::new("Test simulation".to_string()));
        // Test with mixer
        let _ = simulation.add_block(BlockType::Mixer);

        // Test with separator
        let _ = simulation.add_block(BlockType::Separator);
    }

    #[test]
    fn add_connectors() {
        let mut simulation = Simulation::new(Settings::new("Test simulation".to_string()));
        let _ = simulation.add_block(BlockType::Mixer);
        let _ = simulation.add_block(BlockType::Separator);
        // let _ = simulation.add_connector();
    }
}
