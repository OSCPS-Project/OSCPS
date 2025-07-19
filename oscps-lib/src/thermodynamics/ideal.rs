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


///# BaseEOSModel
///
///This will hold the common methods associated with the ideal thermodynamics 
///packages.
pub trait BaseEOSModel {
    /// Computes the total moles for the stream
    ///
    /// # Returns
    /// The total amount of moles
    fn total_moles(&self) -> AmountOfSubstance {
        let mut total_moles = 0;
        let comp_list_ref: &Vec<ComponentData> = &self.components;
        for substance in comp_list_ref {
            total_moles += substance.get::<amount_of_substance::mole>();
        }
        return AmountOfSubstance::new::<amount_of_substance::mole>(total_moles);
    }
    /// Function to calculate the Ideal Helmholtz Energy
    /// 
    /// # Arguments
    /// * `V` - The volume (type 'uom Volume')
    /// * `T` - The temperature (type 'uom ThermodynamicTemperature')
    ///
    /// # Returns
    /// The ideal helmholtz free energy (units of Joules)
    fn ideal_helmholtz(&self, V : Volume, T : ThermodynamicTemperature) -> Energy {
        let k_b = ThermodynamicConstants::BoltzmannConstant.value().get::<heat_capacity::joule_per_kelvin>();
        let N = self.total_moles();
        let T_val = T.get::<thermodynamic_temperature::kelvin>();
        let V_val = V.get::<thermodynamic_temperature::kelvin>();
        let mut a_ideal = N*k_b*T_val;
        let comp_list_ref: &Vec<ComponentData> = &self.components;
        for substance in comp_list_ref {
            let substance_molar_quantity = substance.molar_quantity;
            let x_i = substance_molar_quantity / N;
            let log_frac = x_i / (V_val*T_val.powf(1.5));
            a_ideal += x_i * (log_frac.ln() - 1.0);
        }

        return Energy::new::<energy::joule>(a_ideal);

   }
}



