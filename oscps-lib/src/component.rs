pub struct Chemical {
    comp_id: String,
    iupac_name: String,
    chemical_formula: String,
    properties: chem_properties
}

struct chem_properties { //will need to use an API to collect chemical properties information
    normal_melting_point: i64,
    normaul_boiling_point: i64,
}

