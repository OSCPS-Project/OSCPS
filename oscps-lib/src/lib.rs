//! # oscps-lib
//! 'oscps-lib' is a library which provides the functionality required to build
//! a chemical process simulator. It includes both dynamic and steady-state
//! simulation capabilities.

#![warn(missing_docs)]

pub mod blocks;
pub mod properties;
pub mod connector;
pub mod simulation;
pub mod thermodynamics;

