//! # Component

/// A struct to store information regarding the chemical properties of a particular substance.
pub struct Chemical {
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
    /// # Example
    /// ```
    /// // First create a ChemicalProperties struct
    /// using oscps::ChemicalProperties;
    /// using oscps::Chemical;
    /// let water_chemical_propertes = ChemicalProperties::new(273.15, 373.15);
    /// let water = Chemical::new(962, "oxidane", "H2O", water_chemical-properties);
    /// ```
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
    normal_melting_point: f64,
    /// The normal boiling point of a substance at atmospheric pressure in Kelvin
    normal_boiling_point: f64,
}

impl ChemicalProperties {
    /// Create a ChemicalProperties struct
    ///
    /// # Examples
    /// ```
    /// use oscps::ChemicalProperties;
    /// let water_chemical_propertes = ChemicalProperties::new(273.15, 373.15)
    /// ``` 
    pub fn new(normal_melting_point: f64, normal_boiling_point: f64) -> ChemicalProperties {
        ChemicalProperties {
            normal_melting_point,
            normal_boiling_point,
        }
    }
}
