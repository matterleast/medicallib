//! # MedicalLib
//!
//! A comprehensive medical simulation library for modeling human physiological systems.
//!
//! This library provides detailed simulations of various organ systems including:
//! - Circulatory system (Heart)
//! - Respiratory system (Lungs)
//! - Nervous system (Brain, SpinalCord)
//! - Digestive system (Stomach, Esophagus, Intestines, Pancreas, Liver, Gallbladder)
//! - Urinary system (Kidneys, Bladder)
//! - Immune system (Spleen)

pub mod organ;
pub mod patient;
pub mod organs;

pub use organ::Organ;
pub use patient::{BloodComposition, Patient, initialize_patient, update_patient, get_patient_summary};

/// Calculate Body Mass Index (BMI)
///
/// # Arguments
/// * `weight_kg` - Weight in kilograms
/// * `height_m` - Height in meters
///
/// # Returns
/// BMI value
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    if height_m <= 0.0 {
        return 0.0;
    }
    weight_kg / (height_m * height_m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bmi() {
        let bmi = calculate_bmi(70.0, 1.75);
        assert!((bmi - 22.86).abs() < 0.01);
    }
}
