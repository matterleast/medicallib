//! Gallbladder organ simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Gallbladder state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GallbladderState {
    Storing,
    Contracting,
}

/// Gallbladder organ
#[derive(Debug)]
pub struct Gallbladder {
    id: OrganId,
    /// Current state
    pub state: GallbladderState,
    /// Bile volume (mL)
    pub bile_volume_ml: f64,
    /// Bile concentration factor (1.0 = unconcentrated, higher = more concentrated)
    pub bile_concentration: f64,
    /// Capacity (mL)
    pub capacity_ml: f64,
}

impl Gallbladder {
    /// Create new gallbladder
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            state: GallbladderState::Storing,
            bile_volume_ml: 0.0,
            bile_concentration: 1.0,
            capacity_ml: 50.0,
        }
    }

    /// Store bile from liver
    pub fn store_bile(&mut self, volume_ml: f64) {
        if self.bile_volume_ml < self.capacity_ml {
            self.bile_volume_ml += volume_ml;
            self.state = GallbladderState::Storing;
        }
    }

    /// Release bile to duodenum
    pub fn release_bile(&mut self, volume_ml: f64) -> f64 {
        let released = self.bile_volume_ml.min(volume_ml);
        self.bile_volume_ml -= released;
        self.state = GallbladderState::Contracting;
        released * self.bile_concentration
    }
}

impl Organ for Gallbladder {
    fn update(&mut self, _patient: &mut Patient, delta_time_s: f64) {
        // Concentrate bile over time
        if self.bile_volume_ml > 0.0 {
            self.bile_concentration += delta_time_s * 0.01;
            self.bile_concentration = self.bile_concentration.min(5.0);
        }

        // Return to storing state when not actively contracting
        if self.state == GallbladderState::Contracting {
            self.state = GallbladderState::Storing;
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Gallbladder: State={:?}, Volume={:.0} mL, Concentration={:.1}x",
            self.state, self.bile_volume_ml, self.bile_concentration
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Gallbladder"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
