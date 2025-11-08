//! Spinal cord organ simulation
//!
//! Simulates neural signal transmission pathways

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Signal status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignalStatus {
    Normal,
    Impaired,
    Severed,
}

/// Neural tract
#[derive(Debug, Clone)]
pub struct NeuralTract {
    pub name: String,
    pub status: SignalStatus,
    pub signal_strength: f64,  // 0.0 = no signal, 1.0 = normal
}

/// Spinal cord organ
#[derive(Debug)]
pub struct SpinalCord {
    id: OrganId,
    /// Descending motor tract
    pub descending_motor_tract: NeuralTract,
    /// Ascending sensory tract
    pub ascending_sensory_tract: NeuralTract,
    /// Reflex arc
    pub reflex_arc: NeuralTract,
}

impl SpinalCord {
    /// Create new spinal cord
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            descending_motor_tract: NeuralTract {
                name: "Descending Motor".to_string(),
                status: SignalStatus::Normal,
                signal_strength: 1.0,
            },
            ascending_sensory_tract: NeuralTract {
                name: "Ascending Sensory".to_string(),
                status: SignalStatus::Normal,
                signal_strength: 1.0,
            },
            reflex_arc: NeuralTract {
                name: "Reflex Arc".to_string(),
                status: SignalStatus::Normal,
                signal_strength: 1.0,
            },
        }
    }

    /// Sever a tract
    pub fn sever_tract(&mut self, tract_name: &str) {
        let tract = match tract_name {
            "motor" => &mut self.descending_motor_tract,
            "sensory" => &mut self.ascending_sensory_tract,
            "reflex" => &mut self.reflex_arc,
            _ => return,
        };
        tract.status = SignalStatus::Severed;
        tract.signal_strength = 0.0;
    }

    /// Impair a tract
    pub fn impair_tract(&mut self, tract_name: &str, impairment: f64) {
        let tract = match tract_name {
            "motor" => &mut self.descending_motor_tract,
            "sensory" => &mut self.ascending_sensory_tract,
            "reflex" => &mut self.reflex_arc,
            _ => return,
        };
        tract.status = SignalStatus::Impaired;
        tract.signal_strength = (tract.signal_strength - impairment).max(0.0);
        if tract.signal_strength == 0.0 {
            tract.status = SignalStatus::Severed;
        }
    }

    fn status_str(status: SignalStatus) -> &'static str {
        match status {
            SignalStatus::Normal => "Normal",
            SignalStatus::Impaired => "Impaired",
            SignalStatus::Severed => "Severed",
        }
    }
}

impl Organ for SpinalCord {
    fn update(&mut self, _patient: &mut Patient, _delta_time_s: f64) {
        // Spinal cord doesn't actively update - it's affected by external trauma
        // Signal strength remains constant unless damaged
    }

    fn get_summary(&self) -> String {
        format!(
            "SpinalCord: Motor={}, Sensory={}, Reflex={}",
            Self::status_str(self.descending_motor_tract.status),
            Self::status_str(self.ascending_sensory_tract.status),
            Self::status_str(self.reflex_arc.status)
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "SpinalCord"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
