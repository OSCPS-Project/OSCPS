//! # Connector
//!

use crate::thermodynamics::{ThermodynamicConstants, ThermoPackage}; 

/// # Stream
/// 
/// Struct to hold stream information
pub struct Stream {
    // TODO: IDs must be unique within the flowsheet. Consider using integers
    // as IDs and having a separate field for the name of a connector. Adopt
    // a similar scheme for blocks.
    /// ID of the stream. 
    pub s_id : String,
    /// Instance of ThermoState struct that holds thermodynamic information.
    pub thermo : Option<StreamThermoState>,
    // TODO: Change these from strings to integers, or better yet, 
    // references to the source and destination blocks, to minimize
    // computation time spent on looking for sources and destinations.
    /// ID of source block
    pub from_block : String,
    /// ID of destination block
    pub to_block : String
}


impl Stream {
    /// Constructor for 'Stream' struct
    pub fn new(id: String, from_blk_id : String, to_blk_id : String) -> Stream {
        Stream {
            s_id : id,
            thermo : None,
            from_block : from_blk_id,
            to_block : to_blk_id
        }
    }
}

#[allow(dead_code)]
/// Species list
pub struct ComponentData {
    /// Chemical species
    pub chemical_species: Chemical, // will contain intrinsic properties of species
    /// Mass quantity
    pub mass_quantity: Mass,
    /// Molar quantity
    pub molar_quantity: AmountOfSubstance,
    ///volumetric quantity
    pub vol_quantity: Volume,
    /// partial pressure
    pub partial_pressure : Pressure,
}

#[allow(dead_code)]
/// # StreamThermoState
/// 
/// This struct will be used for performing thermodynamic calculations for the streams in the flow
/// diagram.
pub struct StreamThermoState {
    /// Pressure of the state.
    pub pressure: Option<Pressure>,                    // pressure
    /// Temperature of the state.
    pub temperature: Option<ThermodynamicTemperature>, // temperature
    /// List of mole fractions.
    pub mass_list: Vec<ComponentData>,//Information about each component within stream
    /// Total Mass
    pub total_mass : Option<Mass>, // total mass in stream
    /// Total Moles
    pub total_mol : Option<AmountOfSubstance>, // total moles in stream
    /// Total Volume
    pub total_volume : Option<Volume>, // total volume in stream
    ///Thermo Package
    pub thermodynamic_package : Option<Box<dyn ThermoPackage>> // thermodynamics package 
}


#[allow(dead_code)]
/// Implementation of StreamThermoState
/// This struct holds the functionality to perform thermodynamic calculations for streams
impl StreamThermoState {
    /// Constructor for creating a StreamThermoState
    pub fn new() -> Self {
        StreamThermoState {
            pressure : None,
            temperature : None,
            mass_list : vec![],
            total_mass : None,
            total_mol : None,
            total_volume : None,
            thermodynamic_package : None
        }
    }
    /// Public function to execute the calculations for determining the thermodynamic state for the
    /// stream. Dependence on the Thermodynamic Packages.
    pub fn execute_thermo_calcs(&mut self) -> Self {

    }
    /// this function will return the total mass for an individual stream
    fn calc_total_mass(&mut self) -> Mass {
        let mut mass_sum  = 0.0;
        for chem in &self.mass_list {
            mass_sum += chem.mass_quantity.get::<mass::kilogram>();
        }
        self.total_mass = Some(Mass::new::<mass::kilogram>(mass_sum));
        
        self.total_mass.unwrap()
    }
    /// this function will return the total moles for an individual stream
    fn calc_total_moles(&mut self) -> AmountOfSubstance {
        let mut mole_sum  = 0.0;
        for chem in &self.mass_list {
            mole_sum += chem.molar_quantity.get::<amount_of_substance::mole>();
        }
        self.total_mol = Some(AmountOfSubstance::new::<amount_of_substance::mole>(mole_sum));
        
        self.total_mol.unwrap()
    }
}

