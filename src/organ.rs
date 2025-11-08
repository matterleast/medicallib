//! Base organ trait and common types

use crate::patient::Patient;
use std::fmt;
use std::any::Any;

/// Unique identifier for organs
pub type OrganId = usize;

/// Base trait that all organs must implement
pub trait Organ: fmt::Debug {
    /// Update the organ's state based on the patient's condition
    ///
    /// # Arguments
    /// * `patient` - Mutable reference to the patient
    /// * `delta_time_s` - Time step in seconds
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64);

    /// Get a summary of the organ's vital signs and status
    ///
    /// # Returns
    /// A formatted string with the organ's current state
    fn get_summary(&self) -> String;

    /// Get the organ's unique identifier
    fn get_id(&self) -> OrganId;

    /// Get the organ's type name
    fn get_type(&self) -> &'static str;

    /// Get a reference to Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get a mutable reference to Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Macro to implement the as_any methods for organ types
#[macro_export]
macro_rules! impl_organ_any {
    () => {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    };
}
