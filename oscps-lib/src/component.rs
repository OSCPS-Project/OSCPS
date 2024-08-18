//! # Component

extern crate uom;
extern crate pubchem;

/// This will hold the list of chemicals used within the simulation
struct ChemicalList {
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
struct Chemical {
    /// The (PubChem)[https://pubchem.ncbi.nlm.nih.gov/] CID of a compound.
    pubchem_obj : pubchem::Compound,
    properties : pubchem::model::rest::Properties
}

/// This enum will be used by the "Chemical" struct to create the pubchem::Compound obj based on
/// either the chemical name or the pubchem id of the chemical
enum ChemicalIdentifier {
    PubchemID(u32),
    CompoundName(String),
}

impl Chemical {
    pub fn new(identifier : ChemicalIdentifier) -> Chemical {
        let pubchem_chemical_object = match identifier {
            ChemicalIdentifier::PubchemID(id) => pubchem::Compound::new(id),
            ChemicalIdentifier::CompoundName(name) => pubchem::Compound::with_name(name.as_str()),
            //_ => panic!("Need to provide a u32 integer or String to identify the compound!!!")
        };

        //getting the properties of the chemical
        let chemical_properties = pubchem_chemical_object
            .properties(&[pubchem::CompoundProperty::Title, pubchem::CompoundProperty::MolecularFormula, pubchem::CompoundProperty::CanonicalSMILES, pubchem::CompoundProperty::ExactMass])
            .unwrap();

        return Chemical {
            pubchem_obj : pubchem_chemical_object,
            properties : chemical_properties
        };
    }
}


#[cfg(test)]
mod component_tests {
    use super::*;
    use std::io;
    use pubchem;
    use uom::si::mass::gram;
    use uom::si::f64::{Mass};
    
    #[test]
    fn test_chemical_properties_constructor() -> io::Result<()> {
        // Test using water (assuming 1 mole)
        let water_mass_one_mole = Mass::new::<gram>(18.02);
        let water_chemical_obj = Chemical::new(ChemicalIdentifier::PubchemID(962));
        let water_mass_from_obj = (water_chemical_obj.properties.exact_mass.unwrap().parse().expect("Invalid Number"));
        let water_mass_from_obj_uom = Mass::new::<gram>(water_mass_from_obj);

        let diff = water_mass_from_obj_uom - water_mass_one_mole;
        let abs_diff = diff.abs();
        
        if(abs_diff < Mass::new::<gram>(0.1))
        {
            return Ok(());
        }
        else{
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Mass difference is too large"))
        }
    }

}
