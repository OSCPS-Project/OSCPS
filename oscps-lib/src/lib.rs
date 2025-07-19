//! # oscps-lib
//! 'oscps-lib' is a library which provides the functionality required to build
//! a chemical process simulator. It includes both dynamic and steady-state
//! simulation capabilities.

#![warn(missing_docs)]

extern crate anyhow;
extern crate once_cell;
extern crate serde;
extern crate uom;

pub mod blocks;
// pub mod component;
pub mod simulation;
pub mod stream;
pub mod thermodynamics;
