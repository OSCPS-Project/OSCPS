//! # Blocks
//!
//! This file contains traits which describe the traits that will be 
//! implemented by various structs to represent different unit operations.
//!
//! For example, if a block is a simple mixer, then it will implement the
//! MassBalance trait but not th
//!

use crate::component;
use crate::connector;


trait MassBalance {
    fn overall_massive_balance() {}
}

trait EnergyBalance {
    fn energy_balance(){}
}

trait ElementBalance {
    fn element_balance(){}
}

pub struct Mixer{
    pub block_id: String,
    pub input_stream: Vec<connector::Mconnector>,
    pub output_stream: connector::Mconnector
}

impl MassBalance for Mixer{

}

impl EnergyBalance for Mixer{
    
}

// comp a
// comp b
// comp c
// 2a + 4b + c (Reactor) -> 5c 1a

// MassBalance: 2a * massA + 4b* massB + c*massC == 5c*massC + a*massA

// MolesOfElements:


// Output = Input1 + Input2
//
// Mixer :: MassBalance
//
// Output == (Input1 + Input2) // Mass Balance

// Reactor: Time 1:
//
// In : 1 mol H2, 2 mol O2
// Out : 1 mol H2, 2mol O2
//
// Time 2:
// 
// In: 2 mol H2, 1 mol O2
// Out : 2 mol H2, mol O1
//
// Difference (Time 2, Time 1)
//
// H2Diff = (H2(Time2) - H2(Time1))^2
// O2Diff = (O2(Time2) - O2(Time1))^2
//
// TotalDiff = H2oDiff + O2Diff < Tol
