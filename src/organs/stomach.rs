//! Stomach organ simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Stomach state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StomachState {
    Empty,
    Filling,
    Digesting,
    Emptying,
}

/// Chyme (partially digested food)
#[derive(Debug, Clone)]
pub struct Chyme {
    pub volume_ml: f64,
    pub ph: f64,
}

/// Stomach organ
#[derive(Debug)]
pub struct Stomach {
    id: OrganId,
    /// Current state
    pub state: StomachState,
    /// Chyme contents
    pub chyme: Chyme,
    /// Maximum capacity (mL)
    pub capacity_ml: f64,
    /// Digestion rate (mL/min)
    pub digestion_rate: f64,
}

impl Stomach {
    /// Create new stomach
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            state: StomachState::Empty,
            chyme: Chyme {
                volume_ml: 0.0,
                ph: 2.0,  // Acidic
            },
            capacity_ml: 1500.0,
            digestion_rate: 50.0,
        }
    }

    /// Add substance to stomach
    pub fn add_substance(&mut self, volume_ml: f64, ph: f64) {
        self.chyme.volume_ml += volume_ml;
        // Mix pH values (simplified)
        if self.chyme.volume_ml > 0.0 {
            self.chyme.ph = (self.chyme.ph + ph) / 2.0;
        }
        self.state = if self.chyme.volume_ml > 0.0 {
            StomachState::Filling
        } else {
            StomachState::Empty
        };
    }
}

impl Organ for Stomach {
    fn update(&mut self, _patient: &mut Patient, delta_time_s: f64) {
        // Update state based on volume
        if self.chyme.volume_ml == 0.0 {
            self.state = StomachState::Empty;
        } else if self.chyme.volume_ml < self.capacity_ml * 0.3 {
            self.state = StomachState::Filling;
        } else if self.chyme.volume_ml < self.capacity_ml * 0.7 {
            self.state = StomachState::Digesting;
        } else {
            self.state = StomachState::Emptying;
        }

        // Digest and empty chyme
        if self.chyme.volume_ml > 0.0 {
            let digestion_amount = self.digestion_rate * delta_time_s / 60.0;
            self.chyme.volume_ml = (self.chyme.volume_ml - digestion_amount).max(0.0);

            // Maintain acidic pH
            self.chyme.ph = (self.chyme.ph - delta_time_s * 0.1).max(1.5);
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Stomach: State={:?}, Volume={:.0} mL, pH={:.1}",
            self.state, self.chyme.volume_ml, self.chyme.ph
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Stomach"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
