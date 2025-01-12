//! # Simulation
//!
//! Allows for the construction of a simulation object. TODO: Implement this.

use std::collections::HashMap;
use crate::blocks::Block;
use crate::connector::Stream;

// fn compute_outlet_phase_fractions(&self) {

// }

// fn compute_outlet_temperature(&self) {

// }

// fn compute_outlet_pressure(&self) {

// }
#[derive(Debug, Clone)]
struct Settings {
    simulation_name: String,
}

impl Settings {
    /// Create a Settings struct.
    fn new(simulation_name: String) -> Self {
        return Settings {
           simulation_name, 
        }
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

struct Simulation {
    blocks: HashMap<u64, Box<dyn Block>>,
    connectors: HashMap<u64, Stream>,
    settings: Settings,
}

impl Simulation {
    pub fn new(settings: Settings) -> Self {
        Self {
            blocks: HashMap::new(),
            connectors: HashMap::new(),
            settings,
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

#[cfg(test)]
mod simulation_tests {
    use super::*;

    #[test]
    fn initialize_simulation() {
        let settings = Settings::new("Test simulation".to_string()); 
        
        // Test that settings are properly initialized:
        assert_eq!(settings.simulation_name, "Test simulation");

        loop {
        println!("Settings: {}", settings.simulation_name);
        }

        let simulation = Simulation::new(settings);

        assert_eq!(simulation.blocks.len(), 0);
        assert_eq!(simulation.connectors.len(), 0);
        assert_eq!(simulation.settings.simulation_name, "Test simulation");
    }

}
