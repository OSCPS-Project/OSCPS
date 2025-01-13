//! # Simulation
//!
//! Allows for the construction of a simulation object. TODO: Implement this.

use crate::blocks;
use crate::blocks::Block;
use crate::connector::Stream;
use std::collections::HashMap;

/// Used for initializing blocks
enum BlockType {
    Mixer,
    Separator,
}

// fn compute_outlet_phase_fractions(&self) {

// }

// fn compute_outlet_temperature(&self) {

// }

// fn compute_outlet_pressure(&self) {

// }
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

// #[derive(Debug, Clone)]
// struct SimulationState {
//     // Add fields as needed
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
    connectors: HashMap<u64, Stream>,
    settings: Settings,
    next_id: u64,
}

impl Simulation {
    pub fn new(settings: Settings) -> Self {
        Self {
            next_id: 0,
            blocks: HashMap::new(),
            connectors: HashMap::new(),
            settings,
        }
    }

    pub fn add_block(&mut self, block_type: BlockType) -> Result<(), &str> {
        return match block_type {
            BlockType::Mixer => {
                let id = self.gen_id();
                self.blocks
                    .entry(id)
                    .or_insert(Box::new(blocks::Mixer::new(id)));
                Ok(())
            }
            BlockType::Separator => {
                let id = self.gen_id();
                self.blocks
                    .entry(id)
                    .or_insert(Box::new(blocks::Mixer::new(id)));
                Ok(())
            }
        };
    }

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
        assert_eq!(simulation.connectors.len(), 0);
    }

    #[test]
    fn add_blocks() {
        let mut simulation = Simulation::new(Settings::new("Test simulation".to_string()));
        // Test with mixer
        let _ = simulation.add_block(BlockType::Mixer);

        // Test with separator
        let _ = simulation.add_block(BlockType::Mixer);
    }
}
