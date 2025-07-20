//! # Blocks
//!
//! This file contains traits implemented by various structs to represent
//! different unit operations.
//!
//! For example, if a block is a simple mixer, then it will implement the
//! MassBalance trait but not the EnergyBalance.

// use crate::stream::Stream;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::sync::Arc;
use uom::si::energy::joule;
use uom::si::f64::Energy;
use uom::si::f64::Mass;
use uom::si::mass::kilogram;

use crate::simulation::StreamReference;

/// # Block
///
/// A trait that all blocks must implement.
/// TODO: In ASPEN, streams can be used to specify process inputs and outputs.
/// Instead, have special blocks that are 'source' and 'sink' blocks for
/// material entering and exiting the simulation. To make it more user friendly,
/// if a user attempts to run a simulation with stream that are not connected to
/// inputs or outputs, offer to automatically insert sources/sinks where the loose
/// ends are. While these special blocks will still have to implement this trait
/// (and thus implement unnecessary functions, such as the "connect_input" function
/// for a source block, these functions can simply be dummy functions for this special case.
/// For safety, they can throw errors if called, but they should never be used.
pub trait Block: Debug + Send + Sync {
    /// Connect an input to a block. TODO: Have this function create the input stream and return a
    /// reference to it. Then use that stream reference to connect an output.
    fn connect_input(&mut self, stream: StreamReference) -> Result<(), &str>;
    /// Disconnect an input to a block
    fn disconnect_input(&mut self, stream: StreamReference) -> Result<(), &str>;
    /// Connect an output to a block
    fn connect_output(&mut self, stream: StreamReference) -> Result<(), &str>;
    /// Disconnect an output to a block
    fn disconnect_output(&mut self, stream: StreamReference) -> Result<(), &str>;
    // TODO: Add additional functions that all Blocks should implement
}

/// # Separator
///
/// A Separator block that allows components of a stream to be separated.
/// Allows for a single input and an arbitrary number of outputs.
#[allow(dead_code)]
#[derive(Debug)]
struct Separator {
    id: u64,
    input: Option<StreamReference>, // An option is used in case there is no input stream
    outputs: Option<Vec<StreamReference>>, // An empty vec can represent no outputs, no need for Option<Vec<Stream>>>
                                           // TODO: Add additional fields that controls how components are separated
}

#[allow(dead_code)]
impl Separator {
    fn new(id: u64) -> Self {
        Separator {
            id,
            input: None,
            outputs: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
/// # Mixer
///
/// A block used for simple stream mixing operations. Spacial information
/// is not stored in the case that non-gui applications use this backend.
pub struct Mixer {
    /// Set of inlet streams for the mixer
    pub inputs: Option<Vec<StreamReference>>,
    /// Outlet stream for the mixer block
    pub output: Option<StreamReference>,
}

#[allow(dead_code)]
/// Implementations of the mixer block.
impl Mixer {
    /// Create a new mixer block. TODO: Figure out importance of lifetimes
    pub fn new() -> Mixer {
        Mixer {
            inputs: None,
            output: None,
        }
    }

    // TODO: Uncomment once desired base functionality is achieved
    // /// Execute the mixer block (calculate balances, output streams, etc) /// This function still needs to be implemented pub fn execute_block(&mut self) {
    //     self.outlet_stream = Some(connector::Stream {
    //         s_id: String::from("Mass_Outlet"),
    //         thermo: None,
    //         from_block: String::from("M1"),
    //         to_block: String::from("M2")
    //         // m_flow_total: self.compute_total_outlet_mass_flow().unwrap(),
    //     });
    //     // self.outlet_stream_energy = Some(connector::Econnector {
    //     //     e_conn_id: String::from("Energy Outlet"),
    //     //     energy_flow_total: self.compute_outlet_energy_flows().unwrap(),
    //     // });
    // }

    // /// This private method will compute the outlet mass flows for the mixer block
    // ///
    // /// # Returns
    // ///
    // /// A Mass quantity (uom object) that holds the outlet mass flow
    // fn compute_total_outlet_mass_flow(&self) -> Option<f64> {
    //     // TODO: steps to implement function:
    //     // Need to loop through each of the connector structures and add up the mass flows
    //     // During this process, need to make sure that all the mass flows are in the same units
    //     // Use the UOM package to help with this part...
    //     let mut mass_flow_sum: f64 = 0.0;

    //     for s in self.inlet_streams.iter() {
    //         mass_flow_sum += s.thermo.as_ref().unwrap().total_mass();
    //     }
    //     Some(mass_flow_sum)
    // }

    // /// Determines the total energy flowing through the block
    // fn compute_outlet_energy_flows(&self) -> Option<f64> {
    //     let mut energy_flow_sum: f64 = 0.0;

    //     for s in self.inlet_streams.iter() {
    //         energy_flow_sum += s.thermo.as_ref().unwrap().enthalpy();
    //     }
    //     Some(energy_flow_sum)
    // }

    // /// Determines the phase fractions of the output using thermodynamics.
    // /// TODO: Implement this function
    // fn compute_outlet_phase_fractions(&self) {}

    // /// Computes the outlet temperature of the mixer (assumes no chemical
    // /// reactions) TODO: Implement this function
    // fn compute_outlet_temperature(&self) {}

    // /// Computes the mixer outlet pressure.
    // /// TODO: Implement this function
    // fn compute_outlet_pressure(&self) {}
}

impl Block for Mixer {
    fn connect_input<'a>(&mut self, stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.inputs {
            None => {
                let mut input_vec = Vec::new();
                input_vec.push(stream);
                self.inputs = Some(Vec::new());
            }
            Some(input_vec) => input_vec.push(stream),
        }
        Ok(())
    }

    fn disconnect_input(&mut self, stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.inputs {
            None => {
                return Err("Input disconnect requested for block with no inputs.");
            }
            Some(input_vec) => {
                let prev_len = input_vec.len();
                input_vec.retain(|arc| !Arc::ptr_eq(arc, &stream));
                let curr_len = input_vec.len();
                if prev_len == curr_len {
                    return Err("Attempted disconnect of input not present on Mixer.");
                }
            }
        }
        Ok(())
    }

    fn connect_output(&mut self, stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.output {
            None => {
                self.output = Some(stream);
                Ok(())
            }
            Some(_) => Err("Attempted to connect output on Mixer with existing output."),
        }
    }

    fn disconnect_output(&mut self, _stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.output {
            None => Err("Attempted disconnect on Mixer with no output."),
            Some(_) => {
                self.output = None;
                Ok(())
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
/// # Source
///
/// A block used to introduce materials into the simulation.
pub struct Source {
    /// Outlet stream for the mixer block
    pub output: Option<StreamReference>,
}

#[allow(dead_code)]
/// Implementations of the source
impl Source {
    /// Create a new source block.
    pub fn new() -> Source {
        Source { output: None }
    }

    /// Create a new source block.
    pub fn with_output(output: StreamReference) -> Source {
        Source {
            output: Some(output),
        }
    }
}

impl Block for Source {
    fn connect_input<'a>(&mut self, _stream: StreamReference) -> Result<(), &'static str> {
        Err("Attempted input connection for Source block. Source does not have inputs.")
    }

    fn disconnect_input(&mut self, _stream: StreamReference) -> Result<(), &'static str> {
        Err("Attempted input disconnect for Source block. Source does not have inputs.")
    }

    fn connect_output(&mut self, stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.output {
            None => {
                self.output = Some(stream);
                Ok(())
            }
            Some(_) => Err("Attempted to connect output on Mixer with existing output."),
        }
    }

    fn disconnect_output(&mut self, _stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.output {
            None => Err("Attempted disconnect on Mixer with no output."),
            Some(_) => {
                self.output = None;
                Ok(())
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
/// # Sink
///
/// A block for removing materials from a simulation.
pub struct Sink {
    /// Inlet stream for the mixer block
    pub inputs: Option<Vec<StreamReference>>,
}

#[allow(dead_code)]
/// Implementations of the source
impl Sink {
    /// Create a new source block.
    pub fn new() -> Sink {
        Sink { inputs: None }
    }
}

impl Block for Sink {
    fn connect_input<'a>(&mut self, stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.inputs {
            None => {
                let mut input_vec = Vec::new();
                input_vec.push(stream);
                self.inputs = Some(Vec::new());
            }
            Some(input_vec) => input_vec.push(stream),
        }
        Ok(())
    }

    fn disconnect_input(&mut self, stream: StreamReference) -> Result<(), &'static str> {
        match &mut self.inputs {
            None => {
                return Err("Input disconnect requested for Sink with no inputs.");
            }
            Some(input_vec) => {
                let prev_len = input_vec.len();
                input_vec.retain(|arc| !Arc::ptr_eq(arc, &stream));
                let curr_len = input_vec.len();
                // If the length does not change, nothing was removed.
                if prev_len == curr_len {
                    return Err("Attempted disconnect of input not present on Mixer.");
                }
            }
        }
        Ok(())
    }

    fn connect_output(&mut self, _stream: StreamReference) -> Result<(), &'static str> {
        Err("Attempted output connection for Sink block. Sink does not have outputs.")
    }

    fn disconnect_output(&mut self, _stream: StreamReference) -> Result<(), &'static str> {
        Err("Attempted output disconnect for Sink block. Sink does not have outputs.")
    }
}

#[allow(dead_code)]
/// Minimum error allowed for energy difference.
/// TODO: Change this to a relative scale instead of an absolute scale.
pub static TOLERENCE_ENERGY: Lazy<Energy> = Lazy::new(|| Energy::new::<joule>(5.0));

#[allow(dead_code)]
/// Minimum error allowed for mass difference.
/// TODO: Change this to a relative scale instead of an absolute scale.
pub static TOLERENCE_MASS: Lazy<Mass> = Lazy::new(|| Mass::new::<kilogram>(5.0));

#[allow(dead_code)]
/// # MassBalance
///
/// Trait for ensuring the overall mass balance is maintained in a flowsheet.
///
/// This trait can be implemented by any block that needs to ensure mass
/// conservation.
pub trait MassBalance {
    /// Perform a mass balance check on object by comparing inlet and outlet
    /// mass. TODO: Compare mass flow rates, not mass and check for relative
    /// error instead of absolute, perhaps error should be less than 1e-6
    /// fraction of the total inlet mass. This can be an adjustable parameter.
    /// Smaller takes longer to converge, but is more
    fn mass_balance_check(&self, mass_in: Mass, mass_out: Mass) -> bool {
        let mass_in_kg = mass_in.get::<kilogram>();
        let mass_out_kg = mass_out.get::<kilogram>();
        let mass_difference = mass_in_kg - mass_out_kg;
        mass_difference <= TOLERENCE_MASS.get::<kilogram>()
    }
}

#[allow(dead_code)]
/// # EnergyBalance
///
/// This trait ensures that blocks in the flowsheet adhere to energy
/// conservation principles.
pub trait EnergyBalance {
    /// Perform an energy balance on a block. Checks all input and output
    /// streams and ensures that energy stays the same. TODO: Ensure that
    /// energy loss is accounted for. For example, a mixer may not be entirely
    /// adiabatic, and therefor some energy will be lost to the environment.
    /// Also implement changes in issue #19.
    fn energy_balance_check(&self, energy_in: Energy, energy_out: Energy) -> bool {
        let energy_in_joules = energy_in.get::<joule>();
        let energy_out_joules = energy_out.get::<joule>();
        let energy_difference = energy_in_joules - energy_out_joules;
        energy_difference <= TOLERENCE_ENERGY.get::<joule>()
    }
}

/// Applying mass balance trait to Mixer Block
impl MassBalance for Mixer {}

/// Applying the energy balance trait to the Mixer Block
impl EnergyBalance for Mixer {}

/// Block implements Clone
impl Clone for Box<dyn Block> {
    fn clone(&self) -> Self {
        todo!();
    }
}

/// # Block Tests
///
/// The following module holds all the unit test cases for the blocks module
#[cfg(test)]
mod block_tests {
    // use crate::connector::Stream;

    // use super::*;
    // use uom::si::energy::kilojoule;
    // use uom::si::f64::Energy;
    // use uom::si::mass::pound;

    // #[test]
    // /// checks whether the mass balance check function was implemented properly
    // fn test_mass_balance_check_steady_state_for_mixer() {
    //     // here you will need to check that the mass into the mixer = mass out of mixer

    //     let mixer_test_obj = Mixer {
    //         block_id: String::from("Test Mixer"),
    //         y_cord: 0,
    //         inlet_streams: Vec::new(),
    //         outlet_stream: None,
    //     };
    //     let mass_in = Mass::new::<pound>(100.0);
    //     let mass_out = Mass::new::<pound>(95.0);
    //     assert!(mixer_test_obj.mass_balance_check(mass_in, mass_out));
    // }

    // #[test]
    // /// checks if the 'energy_balance_check' function was implemented properly
    // fn test_energy_balance_check_steady_state_for_mixer() {
    //     // energy into mixer = energy out of mixer
    //     let mixer_test_obj = Mixer {
    //         block_id: String::from("Test Mixer"),
    //         x_cord: 0,
    //         y_cord: 0,
    //         inlet_streams: Vec::new(),
    //         outlet_stream: None,
    //     };
    //     let energy_in = Energy::new::<kilojoule>(10.0);
    //     let energy_out = Energy::new::<kilojoule>(95.0);
    //     assert!(mixer_test_obj.energy_balance_check(energy_in, energy_out));
    // }

    // #[test]
    // checking functionality of 'compute_total_outlet_mass_flow'
    // fn test_compute_total_outlet_mass_flow() {
    //     let in_streams_mass = vec![
    //         Mconnector {
    //             m_conn_id: String::from("Mass1"),
    //             m_flow_total: 3.0,
    //         },
    //         Mconnector {
    //             m_conn_id: String::from("Mass2"),
    //             m_flow_total: 7.0,
    //         },
    //     ];
    //     let mixer = Mixer::new(String::from("Mixer3"), 0, 0, in_streams_mass, vec![]);

    //     assert_eq!(mixer.compute_total_outlet_mass_flow(), Some(10.0));
    // }

    // #[test]
    // /// checking functionality of 'compute_outlet_energy_flows'
    // fn test_compute_outlet_energy_flows() {
    //     let in_streams_energy = vec![
    //         Econnector {
    //             e_conn_id: String::from("Energy1"),
    //             energy_flow_total: 100.0,
    //         },
    //         Econnector {
    //             e_conn_id: String::from("Energy2"),
    //             energy_flow_total: 200.0,
    //         },
    //     ];
    //     let mixer = Mixer::new(String::from("Mixer5"), 0, 0, vec![], in_streams_energy);

    //     assert_eq!(mixer.compute_outlet_energy_flows(), Some(300.0));
    // }
}
