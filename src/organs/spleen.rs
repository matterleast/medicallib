//! Spleen organ simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Red pulp component (blood filtration)
#[derive(Debug, Clone)]
pub struct RedPulp {
    pub rbc_breakdown_rate: f64,  // Red blood cells/min
}

/// White pulp component (immune function)
#[derive(Debug, Clone)]
pub struct WhitePulp {
    pub lymphocyte_count: f64,  // Cells/μL
    pub macrophage_count: f64,  // Cells/μL
}

/// Spleen organ
#[derive(Debug)]
pub struct Spleen {
    id: OrganId,
    /// Red pulp (blood filtration)
    pub red_pulp: RedPulp,
    /// White pulp (immunity)
    pub white_pulp: WhitePulp,
}

impl Spleen {
    /// Create new spleen
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            red_pulp: RedPulp {
                rbc_breakdown_rate: 5000.0,
            },
            white_pulp: WhitePulp {
                lymphocyte_count: 1500.0,
                macrophage_count: 500.0,
            },
        }
    }
}

impl Organ for Spleen {
    fn update(&mut self, _patient: &mut Patient, _delta_time_s: f64) {
        // Spleen function is relatively constant
        // Could be enhanced to respond to infections or blood disorders
    }

    fn get_summary(&self) -> String {
        format!(
            "Spleen: RBC breakdown={:.0}/min, Lymphocytes={:.0}/μL",
            self.red_pulp.rbc_breakdown_rate,
            self.white_pulp.lymphocyte_count
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Spleen"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
