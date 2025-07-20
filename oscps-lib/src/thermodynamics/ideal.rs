///#EOSModel
///
///This struct will represent the different thermodynamic equations of state 

//Internal Crate Imports
use crate::thermodynamics::*;
use crate::stream::ComponentData;

//External Imports
use std::sync::Arc;
use uom::si::f64::*;
use uom::si::molar_energy;
use uom::si::molar_heat_capacity;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;
use uom::si::volume;
use uom::si::ratio;

//Specific EOS Models using this Trait
pub mod base_ideal_eos;
pub mod walkermodel_ideal_eos;


///# BaseEOSModel
///
///This will hold the common methods associated with the ideal thermodynamics 
///packages.
///
///Note - In general the EOS Models will hold the functions for calculating resid helmholtz free
///energy (as total = resid + ideal). The supporting methods will use these EOS models to then
///calculate the specific properties :)
pub trait BaseEOSModel {
    ///Getting the list of component data
    ///
    /// # Returns
    /// Vector with ``ComponentData`` objects
    fn components (&self) -> Arc<Vec<ComponentData>>;
    /// Default function that computes the total moles for the stream
    ///
    /// # Returns
    /// The total amount of moles
    fn total_moles(&self) -> AmountOfSubstance {
        let mut total_moles = 0.0;
        let comp_list_ref =  self.components();
        for substance in comp_list_ref.as_ref() {
            total_moles += substance.molar_quantity.get::<amount_of_substance::mole>();
        }
        return AmountOfSubstance::new::<amount_of_substance::mole>(total_moles);
    }
    /// Default function to calculate the Ideal Helmholtz Energy
    /// 
    /// # Arguments
    /// * `V` - The volume (type 'uom Volume')
    /// * `T` - The temperature (type 'uom ThermodynamicTemperature')
    ///
    /// # Returns
    /// The ideal helmholtz free energy (units of Joules)
    fn ideal_helmholtz(&self, V : Volume, T : ThermodynamicTemperature) -> Energy {
        let k_b = ThermodynamicConstants::BoltzmannConstant.value()
            .downcast_ref::<HeatCapacity>()  
            .unwrap()
            .get::<heat_capacity::joule_per_kelvin>();
        let N = self.total_moles().get::<amount_of_substance::mole>();
        let t_val = T.get::<thermodynamic_temperature::kelvin>();
        let v_val = V.get::<volume::cubic_meter>();
        let mut a_ideal = N*k_b*t_val;
        let comp_list_ref =  self.components();
        for substance in comp_list_ref.as_ref() {
            let substance_molar_quantity = substance.molar_quantity.get::<amount_of_substance::mole>();
            let x_i = substance_molar_quantity / N;
            let log_frac = x_i / (v_val*t_val.powf(1.5));
            a_ideal += x_i * (log_frac.ln() - 1.0);
        }

        return Energy::new::<energy::joule>(a_ideal);
    }
}



