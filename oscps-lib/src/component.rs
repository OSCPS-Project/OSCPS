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
    pub async fn new(identifier : ChemicalIdentifier) -> Result<Self> {
        let pubchem_chemical_object = match identifier {
            ChemicalIdentifier::PubchemID(id) => pubchem::Compound::new(id),
            ChemicalIdentifier::CompoundName(name) => pubchem::Compound::with_name(name.as_str()),
        };

        let cid_vec = pubchem_chemical_object.cids().unwrap();

        let cid : i32 = cid_vec[0];

        //getting the properties of the chemical
        let prop = ChemicalProperties::new(cid).await?;

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

struct ChemicalProperties {
    molar_mass: f64,    // kg/mol
    critical_temp: f64, // K
    critical_pressure: f64, // Pa
    acentric_factor: f64,
}

impl ChemicalProperties {
    pub async fn new(cid : i32) -> Result<Self> {
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
mod component_tests {
    
}
