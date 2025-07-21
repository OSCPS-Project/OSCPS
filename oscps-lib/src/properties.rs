//! # Properties
//!
//! Contains chemical properties for species in the simulation.

//Importing pure species properties
pub mod pure_species_properties;

extern crate uom;
extern crate pubchem;
use anyhow::Result;
use uom::si::f64::*;
use std::{thread,time::Duration};
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
/// Used by the "Chemical" struct to create the pubchem::Compound obj based on
/// either the chemical name or the pubchem id of the chemical
pub enum ChemicalIdentifier {
    /// The PubChem ID of the component.
    PubchemID(u32),
    /// The actual name of the component.
    CompoundName(String),
}


#[allow(dead_code)]
/// A struct to store information regarding the chemical properties of a 
/// particular substance. The "Chemical" struct is a wrapper for the 
/// pubchem::Compound object
pub struct Chemical {
    /// The (PubChem)[<https://pubchem.ncbi.nlm.nih.gov/>] CID of a compound.
    pub pubchem_obj: pubchem::Compound,
    /// Physical properties of a compound.
    pub properties: ChemicalProperties,
}

#[allow(dead_code)]
/// Implementation of the chemical of interest.
impl Chemical {
    /// Constructs a new chemical.
    pub fn new(identifier: ChemicalIdentifier) -> Result<Self> {
        let pubchem_chemical_object = match identifier {
            ChemicalIdentifier::PubchemID(id) => pubchem::Compound::new(id),
            ChemicalIdentifier::CompoundName(name) => pubchem::Compound::with_name(name.as_str()),
        };
        let mut request_counter = 0;
        let mut cid_vec = None;
        while request_counter <= 10 {
            match pubchem_chemical_object.cids(){
                Ok(cid_list) => {
                    cid_vec = Some(cid_list);
                    break;
                },
                _ => {
                    request_counter += 1;
                    thread::sleep(Duration::from_secs(10));
                }
            };
        }

        // let cid_vec = pubchem_chemical_object.cids().unwrap();
        let cid: i32 = cid_vec.unwrap()[0];
        let prop = ChemicalProperties::new(cid);
        Ok(Chemical {
            pubchem_obj: pubchem_chemical_object,
            properties: prop,
        })
    }
    /// Returns the pubchem object for the compound.
    pub fn get_pubchem_obj(&self) -> &pubchem::Compound {
        &self.pubchem_obj
    }

    /// Returns the "ChemicalProperties" object for the "Chemical" object.
    pub fn get_properties(&self) -> &ChemicalProperties {
        &self.properties
    }
}

#[allow(dead_code)]
/// Struct containing properties of a chemical
pub struct ChemicalProperties {
    /// Pure species properties
    pub critical: Option<CriticalProperties>,
    
    /// Heat capacity coefficients (optional, stored as an array)
    pub heat_capacity: Option<HeatCapacityCoefficients>,
    
    /// Transport properties (optional, could include viscosity, etc.)
    pub transport: Option<TransportProperties>,
    
    /// Additional chemical property categories
        // Here we might add properties related to binary interactions, etc...
    pub other_properties: Option<Vec<SpecialProperties>>,
}

impl ChemicalProperties{
    /// constructor for the ``ChemicalProperties`` struct
    pub fn new(_cid: i32) -> ChemicalProperties {
        return ChemicalProperties { critical: None, heat_capacity: None, transport: None, other_properties: None};
    }
}

pub struct CriticalProperties {}

pub struct HeatCapacityCoefficients {}

pub struct TransportProperties {}

pub struct SpecialProperties {}



#[cfg(test)]
mod chemical_species_tests {
}
