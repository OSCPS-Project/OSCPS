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
use crate::component::ChemicalIdentifier;
use crate::{component, connector};
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


/// # Mixer Block
/// 
/// A block used for simple stream mixing operations.
/// 
/// This struct requires the implementation of both EnergyBalance and MassBalance traits to ensure proper conservation principles are followed.
pub struct Mixer {
    pub block_id : String,
    pub x_cord : i32,
    pub y_cord : i32,
    pub input_streams_mass : Vec<connector::Mconnector>,
    pub input_streams_energy : Vec<connector::Econnector>,
    pub outlet_stream_energy : Option<connector::Econnector>, 
    pub outlet_stream_mass: Option<connector::Mconnector>,
    
}

// Applying mass balance trait to Mixer Block
impl MassBalance for Mixer {}

// Applying the energy balance trait to the Mixer Block
impl EnergyBalance for Mixer {}

impl Mixer {
    pub fn new(id : String, x_cord : i32, y_cord : i32, in_streams_mass : Vec<connector::Mconnector>, in_streams_energy : Vec<connector::Econnector>) -> Mixer {
        return Mixer {
            block_id : id,
            x_cord : x_cord,
            y_cord : y_cord,
            input_streams_mass : in_streams_mass,
            input_streams_energy : in_streams_energy,
            outlet_stream_mass : None,
            outlet_stream_energy : None
        };
    }

    pub fn execute_block(&self) -> bool {
        return false;
    }

    fn compute_inlet_mass_flows(&self) {
        
    }

    fn compute_outlet_mass_flows(&self) {

    }

    fn compute_inlet_energy_flows(&self) {

    }

    fn compute_outlet_energy_flows(&self) {

    }

    fn compute_outlet_phase_fractions(&self) {

    }

    fn compute_outlet_temperature(&self) {

    }

    fn compute_outlet_pressure(&self) {

    }

}

#[cfg(test)]
mod block_tests {
    use crate::connector::Mconnector;

    use super::*;
    use std::io;
    use uom::si::f64::Energy;
    use uom::si::energy::kilojoule;
    use uom::si::mass::pound;
    
    #[test]
    fn test_mass_balance_check_steady_state_for_mixer() {
        // here you will need to check that the mass into the mixer = mass out of mixer
        
        let mixer_test_obj = Mixer {
            block_id : String::from("Test Mixer"),
            x_cord : 0,
            y_cord : 0,
            input_streams_mass : Vec::new(),
            input_streams_energy : Vec::new(),
            outlet_stream_mass : None,
            outlet_stream_energy : None
        };
        let mass_in = Mass::new::<pound>(100.0);
        let mass_out = Mass::new::<pound>(95.0);
        assert!(mixer_test_obj.mass_balance_check(mass_in, mass_out));
    }

    #[test]
    fn test_energy_balance_check_steady_state_for_mixer() {
        // energy into mixer = energy out of mixer
        let mixer_test_obj = Mixer {
            block_id : String::from("Test Mixer"),
            x_cord : 0,
            y_cord : 0,
            input_streams_mass : Vec::new(),
            input_streams_energy : Vec::new(),
            outlet_stream_mass : None,
            outlet_stream_energy : None
        };
        let energy_in = Energy::new::<kilojoule>(10.0);
        let energy_out = Energy::new::<kilojoule>(95.0);
        assert!(mixer_test_obj.energy_balance_check(energy_in, energy_out));
    }
}
