//! # Blocks
//!
//! This file contains traits which describe the traits that will be 
//! implemented by various structs to represent different unit operations.
//!
//! For example, if a block is a simple mixer, then it will implement the
//! MassBalance trait but not the EnergyBalance.
//!


use uom::si::f64::{Energy};
use uom::si::f64::{Mass};
use uom::si::mass::kilogram;
use uom::si::energy::joule;
use crate::connector;
use once_cell::sync::Lazy;

//Initiallizing a global variable for the tolerance for the energy balance
static TOLERENCE_ENERGY: Lazy<Energy> = Lazy::new(|| Energy::new::<joule>(5.0));

//Initiallizing a global variable for the tolerance for the mass balance
static TOLERENCE_MASS: Lazy<Mass> = Lazy::new(|| Mass::new::<kilogram>(5.0));

//Initiallizing a global variable for the tolerance for the element balance

/// Trait for ensuring the overall mass balance is maintained in a flowsheet.
///
/// This trait can be implemented by any block that needs to ensure mass conservation.
pub trait MassBalance {
    // total mass in - total mass out < tolerance
    fn mass_balance_check(&self, mass_in : Mass, mass_out : Mass) -> bool {
        let mass_in_kg = mass_in.get::<kilogram>();
        let mass_out_kg = mass_out.get::<kilogram>();

        let mass_difference = mass_in_kg - mass_out_kg;

        return mass_difference <= TOLERENCE_MASS.get::<kilogram>();
    }
}

/// # EnergyBalance
///
/// This trait ensures that blocks in the flowsheet adhere to energy conservation principles.
///
/// This is useful for distinguishing between "dynamic" and "steady state" simulations.
pub trait EnergyBalance {

    // total energy in - total energy out < tolerance
    fn energy_balance_check(&self, energy_in : Energy, energy_out : Energy) -> bool {
        // Convert both energy_in and energy_out to joules
        let energy_in_joules = energy_in.get::<joule>();
        let energy_out_joules = energy_out.get::<joule>();

        // Calculate the difference between energy_in and energy_out in joules
        let energy_difference = energy_in_joules - energy_out_joules;

        // Check if the energy difference is less than the global threshold
        let within_threshold = energy_difference <= TOLERENCE_ENERGY.get::<joule>();

        return within_threshold;

    }
}

/// # ElementBalance (also known as Atomic Balance)
///
/// This trait ensures atomic conservation, especially relevant in reactor simulations.
///
/// Similar to the EnergyBalance trait, this is useful for determining the nature of the simulation (dynamic or steady state).
pub trait ElementBalance {
    fn element_balance_check() {

    }
}

/// # Mixer Block
/// 
/// A block used for simple stream mixing operations.
/// 
/// This struct requires the implementation of both EnergyBalance and MassBalance traits to ensure proper conservation principles are followed.
pub struct Mixer {
    pub block_id: String,
    //pub input_stream: Vec<connector::Mconnector>,
    //pub output_stream: connector::Mconnector,
}

impl MassBalance for Mixer {
    // Implement mass balance methods specific to Mixer here.
}

impl EnergyBalance for Mixer {
    // Implement energy balance methods specific to Mixer here.
}


#[cfg(test)]
mod block_tests {
    use super::*;
    use std::io;
    use uom::si::f64::Energy;
    use uom::si::energy::kilojoule;
    use uom::si::mass::pound;
    
    #[test]
    fn test_mass_balance_check_steady_state_for_mixer() {
        // here you will need to check that the mass into the mixer = mass out of mixer
        
        let mixer_test_obj = Mixer{
            block_id : String::from("Test Mixer")
        };
        let mass_in = Mass::new::<pound>(100.0);
        let mass_out = Mass::new::<pound>(95.0);
        assert!(mixer_test_obj.mass_balance_check(mass_in, mass_out));
    }

    #[test]
    fn test_energy_balance_check_steady_state_for_mixer() {
        // energy into mixer = energy out of mixer
        let mixer_test_obj = Mixer{
            block_id : String::from("Test Mixer")
        };
        let energy_in = Energy::new::<kilojoule>(10.0);
        let energy_out = Energy::new::<kilojoule>(95.0);
        assert!(mixer_test_obj.energy_balance_check(energy_in, energy_out));
    }
}
