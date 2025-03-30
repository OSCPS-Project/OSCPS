//! # Properties
//!
//! Contains chemical properties for species in the simulation.

extern crate uom;
extern crate pubchem;
///Importing pure species properties
pub mod pure_species_properties;

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
        let prop = ChemicalProperties::new(cid).unwrap();
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
    /// Molecular weight (kg/mol)
    pub molar_mass: MolarMass,
    
    /// Critical properties (optional since not all compounds have them)
    pub critical: Option<CriticalProperties>,
    
    /// Heat capacity coefficients (optional, stored as an array)
    pub heat_capacity: Option<HeatCapacityCoefficients>,
    
    /// Transport properties (optional, could include viscosity, etc.)
    pub transport: Option<TransportProperties>,
    
    /// Additional chemical property categories
    pub other_properties: Option<Vec<OtherProperty>>,
}

/// Implementation of the ChemicalProperties struct.
impl ChemicalProperties {
    // /// Constructor for the ChemicalProperties struct.
    // pub fn new(cid: i32) -> Result<Self> {
    //     println!("Recieving information for compound/element {cid}");
    //     Ok(ChemicalProperties {
    //         molar_mass: 0.0,        // kg/mol
    //         critical_temp: 0.0,     // K
    //         critical_pressure: 0.0, // Pa
    //         acentric_factor: 0.0,
    //         const_a: 0.0,
    //         const_b: 0.0,
    //         const_c: 0.0,
    //         const_d: 0.0
    //     })
    // }
}

#[cfg(test)]
mod chemical_species_tests {
    // use crate::component::{Chemical, ChemicalIdentifier};
    // use std::{thread,time::Duration};

    // #[test]
    // fn test_create_chemical_from_pubchem_id() {
    //     // Using a known PubChem ID, e.g., 7732 (water)
    //     let identifier = ChemicalIdentifier::PubchemID(7732);

    //     let chemical = Chemical::new(identifier);
    //     thread::sleep(Duration::from_secs(10));
    //     
    //     assert!(
    //         chemical.is_ok(),
    //         "Failed to create chemical from PubChem ID"
    //     );
    //     let chemical = chemical.unwrap();

    //     // Verify that the Chemical object contains the expected PubChem object
    //     assert_eq!(chemical.get_pubchem_obj().cids().unwrap()[0], 7732);

    //     // Optionally, verify that the ChemicalProperties object has been initialized
    //     // assert_eq!(chemical.get_properties().molar_mass, 0.0); // Example check for default values
    // }

    // #[test]
    // fn test_create_chemical_from_name() {
    //     let identifier = ChemicalIdentifier::CompoundName(String::from("Water"));

    //     let chemical = Chemical::new(identifier);
    //     thread::sleep(Duration::from_secs(10));


    //     assert!(chemical.is_ok(), "Failed to create chemical from name");
    //     let chemical = chemical.unwrap();

    //     // Verify that the Chemical object contains a valid name
    //     assert_eq!(chemical.get_pubchem_obj().cids().unwrap()[0], 962);
    //     assert_eq!(chemical.pubchem_obj.title().unwrap(), "Water");
    //     // assert_eq!(chemical.get_properties().molar_mass, 0.0); // Example check for default values
    // }
}
