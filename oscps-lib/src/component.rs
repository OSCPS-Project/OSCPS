//! # Component

extern crate uom;

extern crate pubchem;
use anyhow::Result;

/// This will hold the list of chemicals used within the simulation
pub struct ChemicalList {
    chemical_list : Vec<pubchem::Compound>
}



/// A struct to store information regarding the chemical properties of a particular substance.
/// The "Chemical" struct is a wrapper for the pubchem::Compound object
pub struct Chemical {
    /// The (PubChem)[https://pubchem.ncbi.nlm.nih.gov/] CID of a compound.
    pub pubchem_obj : pubchem::Compound,
    pub properties : ChemicalProperties
}

/// This enum will be used by the "Chemical" struct to create the pubchem::Compound obj based on
/// either the chemical name or the pubchem id of the chemical
pub enum ChemicalIdentifier {
    PubchemID(u32),
    CompoundName(String),
}

impl Chemical {

    /// constructor
    pub fn new(identifier : ChemicalIdentifier) -> Result<Self> {
        let pubchem_chemical_object = match identifier {
            ChemicalIdentifier::PubchemID(id) => pubchem::Compound::new(id),
            ChemicalIdentifier::CompoundName(name) => pubchem::Compound::with_name(name.as_str()),
        };

        let cid_vec = pubchem_chemical_object.cids().unwrap();

        let cid : i32 = cid_vec[0];


        //getting the properties of the chemical
        let prop = ChemicalProperties::new(cid).unwrap();

        return Ok(Chemical {
            pubchem_obj : pubchem_chemical_object,
            properties : prop
        });
    }
    /// returns the pubchem object for the compound
    pub fn get_pubchem_obj(&self) -> &pubchem::Compound {
        return &self.pubchem_obj;
    }

    /// returns the "ChemicalProperties" object for the "Chemical" object 
    pub fn get_properties(&self) -> &ChemicalProperties {
        return &self.properties;
    }
}

pub struct ChemicalProperties {
    pub molar_mass: f64,    // kg/mol
    pub critical_temp: f64, // K
    pub critical_pressure: f64, // Pa
    pub acentric_factor: f64,
}

impl ChemicalProperties {
    pub fn new(cid : i32) -> Result<Self> {
        println!("Recieving information for compound/element {cid}");
        return Ok(ChemicalProperties {
                    molar_mass: 0.0,    // kg/mol
                    critical_temp: 0.0, // K
                    critical_pressure: 0.0, // Pa
                    acentric_factor: 0.0,
                    });
    }

}


#[cfg(test)]
mod chemical_species_tests {
    use crate::component::{Chemical,ChemicalIdentifier};
    use std::io;

    #[test]
    fn test_create_chemical_from_pubchem_id() {
        // Using a known PubChem ID, e.g., 7732 (water)
        let identifier = ChemicalIdentifier::PubchemID(7732); 

        let chemical = Chemical::new(identifier);

        assert!(chemical.is_ok(), "Failed to create chemical from PubChem ID");
        let chemical = chemical.unwrap();

        // Verify that the Chemical object contains the expected PubChem object
        assert_eq!(chemical.get_pubchem_obj().cids().unwrap()[0], 7732);

        // Optionally, verify that the ChemicalProperties object has been initialized
        // assert_eq!(chemical.get_properties().molar_mass, 0.0); // Example check for default values
    }


    #[test]
    fn test_create_chemical_from_name() {
        let identifier = ChemicalIdentifier::CompoundName(String::from("Water")); 

        let chemical = Chemical::new(identifier);

        assert!(chemical.is_ok(), "Failed to create chemical from name");
        let chemical = chemical.unwrap();

        // Verify that the Chemical object contains a valid name
        assert_eq!(chemical.get_pubchem_obj().cids().unwrap()[0], 962);
        assert_eq!(chemical.pubchem_obj.title().unwrap(), "Water");
        // assert_eq!(chemical.get_properties().molar_mass, 0.0); // Example check for default values
    }


}
