//! # Blocks
//!
//! This file contains traits which describe the traits that will be 
//! implemented by various structs to represent different unit operations.
//!
//! For example, if a block is a simple mixer, then it will implement the
//! MassBalance trait but not the EnergyBalance.
//!

use crate::connector;

/// Trait for ensuring the overall mass balance is maintained in a flowsheet.
///
/// This trait can be implemented by any block that needs to ensure mass conservation.
trait MassBalance {
    fn overall_mass_balance() {}
}

/// # EnergyBalance
///
/// This trait ensures that blocks in the flowsheet adhere to energy conservation principles.
///
/// This is useful for distinguishing between "dynamic" and "steady state" simulations.
trait EnergyBalance {
    fn energy_balance() {}
}

/// # ElementBalance (also known as Atomic Balance)
///
/// This trait ensures atomic conservation, especially relevant in reactor simulations.
///
/// Similar to the EnergyBalance trait, this is useful for determining the nature of the simulation (dynamic or steady state).
trait ElementBalance {
    fn element_balance() {}
}

/// # Mixer Block
/// 
/// A block used for simple stream mixing operations.
/// 
/// This struct requires the implementation of both EnergyBalance and MassBalance traits to ensure proper conservation principles are followed.
pub struct Mixer {
    pub block_id: String,
    pub input_stream: Vec<connector::Mconnector>,
    pub output_stream: connector::Mconnector,
}

impl MassBalance for Mixer {
    // Implement mass balance methods specific to Mixer here.
}

impl EnergyBalance for Mixer {
    // Implement energy balance methods specific to Mixer here.
}
