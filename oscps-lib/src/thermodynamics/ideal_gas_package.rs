///#IdealGasPackage
///
///Will contain equations related to ideal gases
use crate::thermodynamics::*;
use std::sync::Arc;
use uom::si::f64::*;
use uom::si::mass;
use uom::si::molar_heat_capacity;
use uom::si::pressure;
use uom::si::thermodynamic_temperature;
use uom::si::energy;
use uom::si::amount_of_substance;
use uom::si::volume;


pub struct IdealGasPackage {
    pub temperature : Arc<ThermodynamicTemperature>,
    pub pressure : Arc<Pressure>,
    pub species_list : Vec<Arc<ComponentData>>,
    pub total_mass : Arc<Mass>,
    pub total_vol : Arc<Volume>,
    pub total_mol : Arc<AmountOfSubstance>
}
///Implementing functions specific to the IdealGasPackage
impl IdealGasPackage {
    ///Constructor
    pub fn new(
        temperature: Arc<ThermodynamicTemperature>, 
        pressure : Arc<Pressure>, 
        species_list : Vec<Arc<ComponentData>>, 
        total_mass : Arc<Mass>, 
        total_vol : Arc<Volume>, 
        total_mol : Arc<AmountOfSubstance>) -> Self {
        IdealGasPackage {
            temperature,
            pressure,
            species_list,
            total_mass,
            total_vol,
            total_mol
        }
    }
}
/// Implementing the ThermoPackage trait for the IdealGasPackage
impl ThermoPackage for IdealGasPackage {
    ///Calculating enthalpy
        // Need to run a for loop where I calculate the enthalpy of each species and then add it to
        // the variable 'total_enthalpy'
        // ASSUMPTIONS CURRENTLY MADE:
            // No enthalpy from phase change
            // when working with gases, assume that they are ideal gases
            // Tref = 298 K & Pref = 101.325 kPa
            // Href = 0 
    fn enthalpy(&self) -> Energy {
        let mut total_enthalpy = 0.0;
        let t_ref = 298.15; //reference temperature 
        let h_ref = 0.0; //Reference enthalpy
        let mut cp_ref;
        let mut cp_t;
        let r = ThermodynamicConstants::UniversalGasConstant.value().downcast::<MolarHeatCapacity>().unwrap();
        
        for chem_object in &self.species_list {
            let chem = &(*chem_object).chemical_species.properties;
            if chem.const_c != 0.0 {
                cp_ref = chem.const_a * t_ref + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * t_ref.powi(2);
                cp_t = chem.const_a * self.temperature.get::<thermodynamic_temperature::kelvin>() + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>().powf(2.0) + (1.0 / 3.0) * (chem.const_c / (10.0f64.powf(6.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>().powf(3.0);
            }
            else{
                cp_ref = chem.const_a * t_ref + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * t_ref.powi(2) + (-1.0) * (chem.const_d / (10.0f64.powf(-5.0))) * t_ref.powi(-1);
                cp_t = chem.const_a * self.temperature.get::<thermodynamic_temperature::kelvin>() + (1.0 / 2.0) * (chem.const_b / (10.0f64.powf(3.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>().powf(2.0) + (-1.0) * (chem.const_d / (10.0f64.powf(-5.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>().powf(-1.0);
            }
            let species_enthalpy = h_ref + (cp_t - cp_ref)* r.get::<molar_heat_capacity::joule_per_kelvin_mole>();
            total_enthalpy +=  species_enthalpy;
        }

        Energy::new::<energy::joule>(total_enthalpy)
    }

    /// Determine ideal gas pressure
    fn pressure(&self) -> Pressure {
        let r = ThermodynamicConstants::UniversalGasConstant.value().downcast::<MolarHeatCapacity>().unwrap();
        let ideal_pressure = (self.total_mol.get::<amount_of_substance::mole>() * r.get::<molar_heat_capacity::joule_per_kelvin_mole>() * self.temperature.get::<thermodynamic_temperature::kelvin>()) / (self.total_vol.get::<volume::cubic_meter>());
        Pressure::new::<pressure::pascal>(ideal_pressure)
    }

    ///Deterrmine entropy
        // Will need to use equation (5.10) from the 'Introduction to Chemical Engineering
        // Thermodynamics' 
    fn entropy(&self) -> Energy {
        let mut entropy_total = 0.0;
        let t_ref = 298.15_f64; //reference temperature 
        let mut cp_ref;
        let mut cp_t;
        let r = ThermodynamicConstants::UniversalGasConstant.value().downcast::<MolarHeatCapacity>().unwrap();
        let p_o = 1.0_f64; // units atm
        
        for chem_object in &self.species_list {
            let chem = &(*chem_object).chemical_species.properties;
            if chem.const_c != 0.0 {
                cp_ref = chem.const_a * t_ref.ln() + (chem.const_b / (10.0f64.powf(3.0))) * t_ref;
                cp_t = chem.const_a * self.temperature.get::<thermodynamic_temperature::kelvin>().ln() +  (chem.const_b / (10.0f64.powf(3.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>() + (1.0 / 2.0) * (chem.const_c / (10.0f64.powf(6.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>().powf(2.0);
            }
            else{
                cp_ref = chem.const_a * t_ref.ln() + (chem.const_b / (10.0f64.powf(3.0))) * t_ref + (-1.0/2.0) * (chem.const_d / (10.0f64.powf(-5.0))) * t_ref.powi(-2);
                cp_t = chem.const_a * self.temperature.get::<thermodynamic_temperature::kelvin>().ln() +  (chem.const_b / (10.0f64.powf(3.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>() + (-1.0/2.0) * (chem.const_d / (10.0f64.powf(-5.0))) * self.temperature.get::<thermodynamic_temperature::kelvin>().powf(-2.0);
            }
            let integral_solve_species = cp_t - cp_ref;
            let pressure_ratio = (*chem_object).partial_pressure.get::<pressure::atmosphere>() / p_o;

            entropy_total += r.get::<molar_heat_capacity::joule_per_kelvin_mole>()*(integral_solve_species - pressure_ratio);
        }

        Energy::new::<energy::joule>(entropy_total)
    }
}

