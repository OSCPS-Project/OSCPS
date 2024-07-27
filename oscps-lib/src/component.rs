//! # Component

extern crate uom;

use uom::si::f64::ThermodynamicTemperature;

/// A struct to store information regarding the chemical properties of a particular substance.
struct Chemical {
    /// The (PubChem)[https://pubchem.ncbi.nlm.nih.gov/] CID of a compound.
    component_id: u64,
    /// The IUPAC name of a compound
    iupac_name: String,
    /// The chemical formula of a compound
    chemical_formula: String,
    /// The chemical properties of a compound
    properties: ChemicalProperties
}

impl Chemical {
    /// Create a Chemical struct
    ///
    /// TODO: Finish this documentation comment
    ///
    pub fn new(component_id: u64, iupac_name: &str, chemical_formula: &str, properties: ChemicalProperties) -> Chemical {
        Chemical {
            component_id,
            iupac_name: iupac_name.to_string(),
            chemical_formula: chemical_formula.to_string(),
            properties: properties,
        }
    }
}

/// A struct for storing chemical properties of a chemical.
///
/// This struct allows OSCPS access to the data needed to predict the various
/// physical properties of a substance using thermodynamic correlations, including melting and
/// boiling point, heat capacity, solubility, and many other properites.
struct ChemicalProperties {
    /// The melting point of a substance at atmospheric pressure in Kelvin
    normal_melting_point: ThermodynamicTemperature,
    /// The normal boiling point of a substance at atmospheric pressure in Kelvin
    normal_boiling_point: ThermodynamicTemperature,
}

impl ChemicalProperties {
    /// Create a ChemicalProperties struct
    ///
    pub fn new(
        normal_melting_point: ThermodynamicTemperature,
        normal_boiling_point: ThermodynamicTemperature
        ) -> ChemicalProperties {
        ChemicalProperties {
            normal_melting_point,
            normal_boiling_point,
        }
    }
}

use std::io;
use uom::si::thermodynamic_temperature::kelvin;

#[test]
fn test_chemical_properties_constructor() -> io::Result<()> {
    // Test using water
    let water_melting_point = ThermodynamicTemperature::new::<kelvin>(273.15);
    let water_boiling_point = ThermodynamicTemperature::new::<kelvin>(373.15);
    let water_properties = ChemicalProperties::new(water_melting_point, water_boiling_point);
    assert_eq!(water_properties.normal_melting_point, water_melting_point);
    assert_eq!(water_properties.normal_boiling_point, water_boiling_point);
    Ok(()) 
}
