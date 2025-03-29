///#IdealGasPackage
///
///Will contain equations related to ideal gases


use crate::thermodynamics::*;
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

///Creating the ideal gas thermodynamics package
pub struct IdealGasPackage {
    ///Temperature
    pub temperature : Arc<ThermodynamicTemperature>,
    /// Pressure
    pub pressure : Arc<Pressure>,
    ///List of Species
    pub species_list : Vec<Arc<ComponentData>>,
    /// Mass
    pub total_mass : Arc<Mass>,
    /// Volume
    pub total_vol : Arc<Volume>,
    /// Moles
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
    fn enthalpy(&self) -> MolarEnergy {
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
            let species_enthalpy = (chem_object.molar_quantity.get::<amount_of_substance::mole>()/self.total_mol.get::<amount_of_substance::mole>())*(h_ref + (cp_t - cp_ref)* r.get::<molar_heat_capacity::joule_per_kelvin_mole>());
            total_enthalpy +=  species_enthalpy;
        }

        MolarEnergy::new::<molar_energy::joule_per_mole>(total_enthalpy)
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
    fn entropy(&self) -> MolarHeatCapacity {
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

            entropy_total += (chem_object.molar_quantity.get::<amount_of_substance::mole>()/self.total_mol.get::<amount_of_substance::mole>())*r.get::<molar_heat_capacity::joule_per_kelvin_mole>()*(integral_solve_species - pressure_ratio);
        }

        MolarHeatCapacity::new::<molar_heat_capacity::joule_per_kelvin_mole>(entropy_total)
    }
    /// Determining vapor fraction
        // In Ideal gas package, only will be used when components are all in gaseous state so
        // vapor fraction will always be equal to 1
    fn vapor_fraction(&self) -> Ratio {
        Ratio::new::<ratio::ratio>(1.0)
    }
    /// Determining Cp (Heat capacity under constant pressure conditions)
    fn heat_capacity_const_pressure(&self) -> MolarHeatCapacity {
        let r = ThermodynamicConstants::UniversalGasConstant.value().downcast::<MolarHeatCapacity>().unwrap();
        let mut total_heat_capacity_const_pressure : f64 = 0.0;
        let mut cp_t;
        let t = self.temperature.get::<thermodynamic_temperature::kelvin>();
        for chem_object in &self.species_list {
            let chem = &(*chem_object).chemical_species.properties;
            if chem.const_c != 0.0 {
                cp_t = chem.const_a + (chem.const_b / (10.0f64.powf(3.0)))*t + (chem.const_c / (10.0f64.powf(6.0)))*t.powf(2.0);            
            }
            else {
                cp_t = chem.const_a + (chem.const_b / (10.0f64.powf(3.0)))*t + (chem.const_d / (10.0f64.powf(-5.0)))*t.powf(-2.0);
            }
            total_heat_capacity_const_pressure += cp_t* (chem_object.molar_quantity.get::<amount_of_substance::mole>()/self.total_mol.get::<amount_of_substance::mole>())*r.get::<molar_heat_capacity::joule_per_kelvin_mole>();
        }
        MolarHeatCapacity::new::<molar_heat_capacity::joule_per_kelvin_mole>(total_heat_capacity_const_pressure)
    }
    ///Determining internal energy
        //Need to figure out way to calculate Cv
    fn internal_energy(&self) -> MolarEnergy {
        MolarEnergy::new::<molar_energy::joule_per_mole>(0.0)
    }
    ///Determining temperature
    fn temperature(&self) -> ThermodynamicTemperature {
        let r = ThermodynamicConstants::UniversalGasConstant.value().downcast::<MolarHeatCapacity>().unwrap().get::<molar_heat_capacity::joule_per_kelvin_mole>();
        //T = PV/nR
        let p = self.pressure.get::<pressure::pascal>();
        let v = self.total_vol.get::<volume::cubic_meter>();
        let n = self.total_mol.get::<amount_of_substance::mole>();
        let ideal_temperature = (p*v)/(n*r);
        ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(ideal_temperature)
    }
    ///Determining volume
    fn volume(&self) -> Volume {
        let r = ThermodynamicConstants::UniversalGasConstant.value().downcast::<MolarHeatCapacity>().unwrap().get::<molar_heat_capacity::joule_per_kelvin_mole>();
        // V = (nRT)/P
        let n = self.total_mol.get::<amount_of_substance::mole>();
        let p = self.pressure.get::<pressure::pascal>();
        let t = self.temperature.get::<thermodynamic_temperature::kelvin>();
        let ideal_volume = (n*r*t)/(p);
        Volume::new::<volume::cubic_meter>(ideal_volume)
    }
    ///Determining the Gibbs free energy
    fn gibbs_free_energy(&self) -> Energy {
        let enthalpy = self.enthalpy().get::<molar_energy::joule_per_mole>()*self.total_mol.get::<amount_of_substance::mole>();
        let entropy = self.entropy().get::<molar_heat_capacity::joule_per_kelvin_mole>()*self.total_mol.get::<amount_of_substance::mole>();
        let gibbs_free_energy_value = enthalpy - self.temperature.get::<thermodynamic_temperature::kelvin>()*entropy;
        Energy::new::<energy::joule>(gibbs_free_energy_value)
    }
}

