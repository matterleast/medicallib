//! Esophagus organ simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Peristalsis state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeristalsisState {
    Idle,
    Contracting,
    Relaxing,
}

/// Food bolus
#[derive(Debug, Clone)]
pub struct Bolus {
    pub position_cm: f64,  // Position along esophagus (0-25 cm)
    pub mass_g: f64,
}

/// Esophagus organ
#[derive(Debug)]
pub struct Esophagus {
    id: OrganId,
    /// Peristalsis state
    pub peristalsis_state: PeristalsisState,
    /// Current bolus being transported
    pub bolus: Option<Bolus>,
    /// Length (cm)
    pub length_cm: f64,
    /// Transport speed (cm/s)
    pub transport_speed: f64,
}

impl Esophagus {
    /// Create new esophagus
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            peristalsis_state: PeristalsisState::Idle,
            bolus: None,
            length_cm: 25.0,
            transport_speed: 3.0,
        }
    }

    /// Initiate swallow
    pub fn initiate_swallow(&mut self, mass_g: f64) {
        self.bolus = Some(Bolus {
            position_cm: 0.0,
            mass_g,
        });
        self.peristalsis_state = PeristalsisState::Contracting;
    }
}

impl Organ for Esophagus {
    fn update(&mut self, _patient: &mut Patient, delta_time_s: f64) {
        if let Some(ref mut bolus) = self.bolus {
            // Move bolus down esophagus
            bolus.position_cm += self.transport_speed * delta_time_s;

            if bolus.position_cm >= self.length_cm {
                // Bolus reached stomach
                self.bolus = None;
                self.peristalsis_state = PeristalsisState::Relaxing;
            } else {
                self.peristalsis_state = PeristalsisState::Contracting;
            }
        } else {
            self.peristalsis_state = PeristalsisState::Idle;
        }
    }

    fn get_summary(&self) -> String {
        if let Some(ref bolus) = self.bolus {
            format!(
                "Esophagus: State={:?}, Bolus at {:.1} cm",
                self.peristalsis_state, bolus.position_cm
            )
        } else {
            format!("Esophagus: State={:?}", self.peristalsis_state)
        }
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Esophagus"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
