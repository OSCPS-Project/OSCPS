//! # Blocks
//!
//! This file contains traits implemented by various structs to represent 
//! different unit operations.
//!
//! For example, if a block is a simple mixer, then it will implement the
//! MassBalance trait but not the EnergyBalance.

use crate::connector;
use crate::connector::Stream;
use once_cell::sync::Lazy;
use uom::si::energy::joule;
use uom::si::f64::Energy;
use uom::si::f64::Mass;
use uom::si::mass::kilogram;

#[allow(dead_code)]
/// Minimum error allowed for energy difference. 
/// TODO: Change this to a relative scale instead of an absolute scale.
pub static TOLERENCE_ENERGY: Lazy<Energy> = 
    Lazy::new(|| Energy::new::<joule>(5.0));

#[allow(dead_code)]
/// Minimum error allowed for mass difference. 
/// TODO: Change this to a relative scale instead of an absolute scale.
pub static TOLERENCE_MASS: Lazy<Mass> = 
    Lazy::new(|| Mass::new::<kilogram>(5.0));

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
    fn energy_balance_check(&self, energy_in: Energy, energy_out: Energy) -> 
        bool {
        let energy_in_joules = energy_in.get::<joule>();
        let energy_out_joules = energy_out.get::<joule>();
        let energy_difference = energy_in_joules - energy_out_joules;
        energy_difference <= TOLERENCE_ENERGY.get::<joule>()
    }
}

#[allow(dead_code)]
/// # Mixer
///
/// A block used for simple stream mixing operations.
pub struct Mixer {
    /// The ID of the block.
    pub block_id: String,
    /// The x-coordiante on a flowsheet of the block.
    pub x_cord: i32,
    /// The y-coordinate on a flowsheet of the block.
    pub y_cord: i32,
    /// Set of inlet streams for the mixer
    pub inlet_streams : Vec<Stream>,
    /// outlet stream for the mixer block
    pub outlet_stream : Option<Stream>
}

/// Applying mass balance trait to Mixer Block
impl MassBalance for Mixer {}

/// Applying the energy balance trait to the Mixer Block
impl EnergyBalance for Mixer {}

#[allow(dead_code)]
/// Implementations of the mixer block.
impl Mixer {
    /// Create a new mixer block.
    pub fn new(
        id: String,
        x_cord: i32,
        y_cord: i32,
        in_streams: Vec<Stream>,
    ) -> Mixer {
        Mixer {
            block_id: id,
            x_cord,
            y_cord,
            inlet_streams: in_streams,
            outlet_stream: None

        }
    }

    /// Execute the mixer block (calculate balances, output streams, etc)
    /// This function still needs to be implemented
    pub fn execute_block(&mut self) {
        self.outlet_stream = Some(connector::Stream {
            s_id: String::from("Mass_Outlet"),
            thermo: None,
            from_block: String::from("M1"),
            to_block: String::from("M2")
            // m_flow_total: self.compute_total_outlet_mass_flow().unwrap(),
        });
        // self.outlet_stream_energy = Some(connector::Econnector {
        //     e_conn_id: String::from("Energy Outlet"),
        //     energy_flow_total: self.compute_outlet_energy_flows().unwrap(),
        // });
    }

    /// This private method will compute the outlet mass flows for the mixer block
    ///
    /// # Returns
    ///
    /// A Mass quantity (uom object) that holds the outlet mass flow
    fn compute_total_outlet_mass_flow(&self) -> Option<f64> {
        // TODO: steps to implement function:
        // Need to loop through each of the connector structures and add up the mass flows
        // During this process, need to make sure that all the mass flows are in the same units
        // Use the UOM package to help with this part...
        let mut mass_flow_sum: f64 = 0.0;

        for s in self.inlet_streams.iter() {
            mass_flow_sum += s.thermo.as_ref().unwrap().total_mass();
        }
        Some(mass_flow_sum)
    }

    /// Determines the total energy flowing through the block
    fn compute_outlet_energy_flows(&self) -> Option<f64> {
        let mut energy_flow_sum: f64 = 0.0;

        for s in self.inlet_streams.iter() {
            energy_flow_sum += s.thermo.as_ref().unwrap().enthalpy();
        }
        Some(energy_flow_sum)
    }

    /// Determines the phase fractions of the output using thermodynamics.
    /// TODO: Implement this function
    fn compute_outlet_phase_fractions(&self) {}

    /// Computes the outlet temperature of the mixer (assumes no chemical
    /// reactions) TODO: Implement this function
    fn compute_outlet_temperature(&self) {}
    
    /// Computes the mixer outlet pressure.
    /// TODO: Implement this function
    fn compute_outlet_pressure(&self) {}
}

/// # Block Tests
/// 
/// The following module holds all the unit test cases for the blocks module
#[cfg(test)]
mod block_tests {
    // use crate::connector::Stream;

    use super::*;
    use uom::si::energy::kilojoule;
    use uom::si::f64::Energy;
    use uom::si::mass::pound;

    #[test]
    /// checks whether the mass balance check function was implemented properly
    fn test_mass_balance_check_steady_state_for_mixer() {
        // here you will need to check that the mass into the mixer = mass out of mixer

        let mixer_test_obj = Mixer {
            block_id: String::from("Test Mixer"),
            x_cord: 0,
            y_cord: 0,
            inlet_streams: Vec::new(),
            outlet_stream: None,
        };
        let mass_in = Mass::new::<pound>(100.0);
        let mass_out = Mass::new::<pound>(95.0);
        assert!(mixer_test_obj.mass_balance_check(mass_in, mass_out));
    }

    #[test]
    /// checks if the 'energy_balance_check' function was implemented properly
    fn test_energy_balance_check_steady_state_for_mixer() {
        // energy into mixer = energy out of mixer
        let mixer_test_obj = Mixer {
            block_id: String::from("Test Mixer"),
            x_cord: 0,
            y_cord: 0,
            inlet_streams: Vec::new(),
            outlet_stream: None,
        };
        let energy_in = Energy::new::<kilojoule>(10.0);
        let energy_out = Energy::new::<kilojoule>(95.0);
        assert!(mixer_test_obj.energy_balance_check(energy_in, energy_out));
    }

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
