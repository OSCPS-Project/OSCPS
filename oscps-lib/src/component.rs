//! # Component

extern crate uom;
use uom::si::f64::{ThermodynamicTemperature, Mass, MassDensity};
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::mass::kilogram;
use uom::si::mass_density::kilogram_per_cubic_meter;

extern crate pubchem;
use anyhow::{anyhow, Result};

/// This will hold the list of chemicals used within the simulation
pub struct ChemicalList {
    chemical_list : Vec<pubchem::Compound>
}


impl ChemicalList{
    /// constructor that will hold the list of compounds being used in simulation
    pub fn new() -> ChemicalList {
        return ChemicalList {
            chemical_list : Vec::<pubchem::Compound>::new()
        };
    }
}

/// A struct to store information regarding the chemical properties of a particular substance.
/// The "Chemical" struct is a wrapper for the pubchem::Compound object
pub struct Chemical {
    /// The (PubChem)[https://pubchem.ncbi.nlm.nih.gov/] CID of a compound.
    pubchem_obj : pubchem::Compound,
    properties : ChemicalProperties
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
    pub melting_pt : Option<uom::si::f64::ThermodynamicTemperature>,
    pub boiling_pt : Option<uom::si::f64::ThermodynamicTemperature>,
    pub density : Option<uom::si::f64::MassDensity>,
    pub molec_mass : Option<uom::si::f64::Mass>
}

impl ChemicalProperties {
    pub async fn new(cid : i32) -> Result<Self> {
        let url = format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/{}/property/MolecularWeight,MeltingPoint,BoilingPoint,Density/JSON",cid);
        let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

        // Extract the first (and typically only) set of properties
        let properties_array = response["PropertyTable"]["Properties"].as_array();

        if let Some(properties) = properties_array.and_then(|arr| arr.get(0)) { 
        // Parse and convert the properties
        let molec_mass = properties["MolecularWeight"]
            .as_str()
            .and_then(|w| w.parse::<f64>().ok())
            .map(|w| uom::si::f64::Mass::new::<kilogram>(w / 1000.0));  // Converting molecular weight to grams

        let melting_pt = properties["MeltingPoint"]
            .as_str()
            .and_then(|p| p.parse::<f64>().ok())
            .map(|p| uom::si::f64::ThermodynamicTemperature::new::<kelvin>(p + 273.15));

        let boiling_pt = properties["BoilingPoint"]
            .as_str()
            .and_then(|p| p.parse::<f64>().ok())
            .map(|p| uom::si::f64::ThermodynamicTemperature::new::<kelvin>(p + 273.15));

        let density = properties["Density"]
            .as_str()
            .and_then(|d| d.parse::<f64>().ok())
            .map(|d| uom::si::f64::MassDensity::new::<kilogram_per_cubic_meter>(d));  // Density in kg/m^3

        Ok(ChemicalProperties {
            melting_pt,
            boiling_pt,
            density,
            molec_mass,
            })
        } else {
            Err(anyhow!("Properties not found for CID {}", cid))
        }
    }

    // pub fn convert_units(&self, initial_units : &dyn uom::si::Units<f64>, final_units : &dyn uom::si::Units<f64>) {

    // }
}


#[cfg(test)]
mod component_tests {
    use super::*;
    use std::io;
    use pubchem;
    use uom::si::mass::gram;
    use uom::si::f64::{Mass};
    
    // #[test]
    // fn test_chemical_properties_constructor() -> io::Result<()> {
    //     // Test using water (assuming 1 mole)
    //     let water_mass_one_mole = Mass::new::<gram>(18.02);
    //     let water_chemical_obj = Chemical::new(ChemicalIdentifier::PubchemID(962));
    //     let water_mass_from_obj = (water_chemical_obj.properties.exact_mass.unwrap().parse().expect("Invalid Number"));
    //     let water_mass_from_obj_uom = Mass::new::<gram>(water_mass_from_obj);

    //     let diff = water_mass_from_obj_uom - water_mass_one_mole;
    //     let abs_diff = diff.abs();
    //     
    //     if(abs_diff < Mass::new::<gram>(0.1))
    //     {
    //         return Ok(());
    //     }
    //     else{
    //         Err(io::Error::new(io::ErrorKind::InvalidInput, "Mass difference is too large"))
    //     }
    // }

}
