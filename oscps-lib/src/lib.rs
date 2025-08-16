//! # oscps-lib
//! 'oscps-lib' is a library which provides the functionality required to build
//! a chemical process simulator. It includes both dynamic and steady-state
//! simulation capabilities.

#![warn(missing_docs)]

extern crate uom;
extern crate once_cell;
extern crate serde;
extern crate anyhow;

pub mod blocks;
pub mod component;
pub mod simulation;
pub mod stream;
pub mod thermodynamics;
