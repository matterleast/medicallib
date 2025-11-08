//! Bladder organ simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Bladder state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BladderState {
    Filling,
    Full,
    Voiding,
}

/// Bladder organ
#[derive(Debug)]
pub struct Bladder {
    id: OrganId,
    /// Current state
    pub state: BladderState,
    /// Urine volume (mL)
    pub urine_volume_ml: f64,
    /// Bladder pressure (cmH2O)
    pub pressure_cm_h2o: f64,
    /// Capacity (mL)
    pub capacity_ml: f64,
    /// Voiding threshold pressure (cmH2O)
    pub voiding_threshold: f64,
}

impl Bladder {
    /// Create new bladder
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            state: BladderState::Filling,
            urine_volume_ml: 0.0,
            pressure_cm_h2o: 5.0,
            capacity_ml: 500.0,
            voiding_threshold: 40.0,
        }
    }

    /// Add urine from kidneys
    pub fn add_urine(&mut self, volume_ml: f64) {
        self.urine_volume_ml += volume_ml;
        if self.urine_volume_ml > self.capacity_ml {
            self.urine_volume_ml = self.capacity_ml;
        }
    }

    /// Void bladder
    pub fn void(&mut self) {
        self.urine_volume_ml = 0.0;
        self.state = BladderState::Filling;
    }
}

impl Organ for Bladder {
    fn update(&mut self, _patient: &mut Patient, _delta_time_s: f64) {
        // Calculate pressure based on volume
        let fill_ratio = self.urine_volume_ml / self.capacity_ml;
        self.pressure_cm_h2o = 5.0 + fill_ratio * 45.0;

        // Update state
        if self.pressure_cm_h2o >= self.voiding_threshold {
            self.state = BladderState::Full;
        } else if self.urine_volume_ml > self.capacity_ml * 0.7 {
            self.state = BladderState::Full;
        } else {
            self.state = BladderState::Filling;
        }

        // Auto-void when full (simplified)
        if self.state == BladderState::Full && self.urine_volume_ml >= self.capacity_ml {
            self.void();
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Bladder: State={:?}, Volume={:.0} mL, Pressure={:.1} cmH2O",
            self.state, self.urine_volume_ml, self.pressure_cm_h2o
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Bladder"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
