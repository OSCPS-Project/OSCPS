//! # Thermodynamics
//!
//! This module will hold all the functions related to calculating themrodynamic properties for the
//! blocks and chemical species.

use uom::si::f64::*; // Use f64 as the underlying type for units
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::pressure::pascal;
use uom::si::amount_of_substance::mole;
use std::collections::HashMap;

struct ThermoState {
    pressure: Pressure,                // Pressure in Pascals
    temperature: ThermodynamicTemperature, // Temperature in Kelvin
    mole_fractions: Vec<AmountOfSubstance>, // Mole fractions, typically unitless
}

impl ThermoState {
    // Constructor for creating a ThermoState
    pub fn new(
        pressure: f64,      // in Pascals
        temperature: f64,   // in Kelvin
        mole_fractions: Vec<f64>,
    ) -> Self {
        ThermoState {
            pressure: Pressure::new::<pascal>(pressure),
            temperature: ThermodynamicTemperature::new::<kelvin>(temperature),
            mole_fractions: mole_fractions
                .into_iter()
                .map(AmountOfSubstance::new::<mole>)
                .collect(),
        }
    }
}


#[cfg(test)]
mod thermo_tests {
    use super::*;
    use uom::si::pressure::pascal;
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::amount_of_substance::mole;

    #[test]
    fn test_thermostate_initialization() {
        // Create a ThermoState instance
        let state = ThermoState::new(
            101_325.0,  // Pressure in Pa
            298.15,     // Temperature in K
            vec![0.5, 0.5], // Mole fractions
        );

        // Assert pressure
        assert_eq!(
            state.pressure.get::<pascal>(),
            101_325.0,
            "Pressure should match the input value"
        );

        // Assert temperature
        assert_eq!(
            state.temperature.get::<kelvin>(),
            298.15,
            "Temperature should match the input value"
        );

        // Assert mole fractions
        assert_eq!(
            state.mole_fractions.len(),
            2,
            "Mole fractions should have the correct number of components"
        );
        assert!(
            (state.mole_fractions[0].get::<mole>() - 0.5).abs() < 1e-12,
            "First mole fraction should match the input value"
        );
    }

    #[test]
    fn test_unit_conversions() {
        // Create a ThermoState instance
        let state = ThermoState::new(
            101_325.0,  // Pressure in Pa
            273.15,     // Temperature in K
            vec![1.0],  // Mole fraction
        );

        // Convert pressure to bar
        let pressure_in_bar = state.pressure.get::<uom::si::pressure::bar>();
        assert!(
            (pressure_in_bar - 1.01325).abs() < 1e-5,
            "Pressure in bar should be approximately 1.01325"
        );

        // Convert temperature to Celsius
        let temp_in_celsius = state.temperature.get::<uom::si::thermodynamic_temperature::degree_celsius>();
        assert!(
            (temp_in_celsius - 0.0).abs() < 1e-12,
            "Temperature in Celsius should be 0.0"
        );
    }

    #[test]
    fn test_invalid_mole_fractions() {
        // Test with mole fractions that don't sum to 1
        let state = ThermoState::new(
            101_325.0,  // Pressure in Pa
            298.15,     // Temperature in K
            vec![0.6, 0.3], // Mole fractions
        );

        let mole_fraction_sum: f64 = state.mole_fractions
            .iter()
            .map(|m| m.get::<mole>())
            .sum();

        assert!(
            (mole_fraction_sum - 0.9).abs() < 1e-12,
            "Mole fraction sum should match the input values"
        );
    }
}
