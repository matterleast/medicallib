//! Lungs organ simulation
//!
//! Simulates respiratory mechanics and gas exchange including:
//! - 5 lobes (right upper/middle/lower, left upper/lower)
//! - Respiratory cycle (inspiration, expiration, pause)
//! - Oxygen saturation
//! - CO2 exchange and capnography

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use std::collections::VecDeque;

/// Respiratory phase
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RespiratoryPhase {
    Inspiration,
    Expiration,
    Pause,
}

/// Lung lobe
#[derive(Debug, Clone)]
pub struct Lobe {
    pub name: String,
    pub volume_ml: f64,
    pub compliance: f64,  // 0.0 = no compliance, 1.0 = normal
    pub ventilation_rate: f64,
}

/// Bronchus
#[derive(Debug, Clone)]
pub struct Bronchus {
    pub name: String,
    pub resistance: f64,  // Airway resistance
}

/// Lungs organ
#[derive(Debug)]
pub struct Lungs {
    id: OrganId,
    /// Right upper lobe
    pub right_upper_lobe: Lobe,
    /// Right middle lobe
    pub right_middle_lobe: Lobe,
    /// Right lower lobe
    pub right_lower_lobe: Lobe,
    /// Left upper lobe
    pub left_upper_lobe: Lobe,
    /// Left lower lobe
    pub left_lower_lobe: Lobe,
    /// Main bronchus
    pub main_bronchus: Bronchus,
    /// Respiratory rate (breaths per minute)
    pub respiration_rate_bpm: f64,
    /// Tidal volume (mL)
    pub tidal_volume_ml: f64,
    /// Oxygen saturation (%)
    pub oxygen_saturation_percent: f64,
    /// End-tidal CO2 (mmHg)
    pub end_tidal_co2_mmhg: f64,
    /// Peak inspiratory pressure (cmH2O)
    pub peak_inspiratory_pressure: f64,
    /// Capnography waveform
    pub capnography_waveform: VecDeque<f64>,
    /// Current respiratory phase
    pub current_phase: RespiratoryPhase,
    /// Internal respiratory cycle timer
    respiratory_cycle_time: f64,
}

impl Lungs {
    /// Create new lungs
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            right_upper_lobe: Lobe {
                name: "Right Upper".to_string(),
                volume_ml: 600.0,
                compliance: 1.0,
                ventilation_rate: 1.0,
            },
            right_middle_lobe: Lobe {
                name: "Right Middle".to_string(),
                volume_ml: 500.0,
                compliance: 1.0,
                ventilation_rate: 1.0,
            },
            right_lower_lobe: Lobe {
                name: "Right Lower".to_string(),
                volume_ml: 800.0,
                compliance: 1.0,
                ventilation_rate: 1.0,
            },
            left_upper_lobe: Lobe {
                name: "Left Upper".to_string(),
                volume_ml: 600.0,
                compliance: 1.0,
                ventilation_rate: 1.0,
            },
            left_lower_lobe: Lobe {
                name: "Left Lower".to_string(),
                volume_ml: 800.0,
                compliance: 1.0,
                ventilation_rate: 1.0,
            },
            main_bronchus: Bronchus {
                name: "Main".to_string(),
                resistance: 1.0,
            },
            respiration_rate_bpm: 16.0,
            tidal_volume_ml: 500.0,
            oxygen_saturation_percent: 98.0,
            end_tidal_co2_mmhg: 38.0,
            peak_inspiratory_pressure: 15.0,
            capnography_waveform: VecDeque::with_capacity(1000),
            current_phase: RespiratoryPhase::Pause,
            respiratory_cycle_time: 0.0,
        }
    }

    /// Inflict damage to a lobe
    pub fn inflict_damage(&mut self, lobe_index: usize, damage_amount: f64) {
        let lobe = match lobe_index {
            0 => &mut self.right_upper_lobe,
            1 => &mut self.right_middle_lobe,
            2 => &mut self.right_lower_lobe,
            3 => &mut self.left_upper_lobe,
            4 => &mut self.left_lower_lobe,
            _ => return,
        };
        lobe.compliance = (lobe.compliance - damage_amount).max(0.0);
    }

    /// Calculate total lung compliance
    fn total_compliance(&self) -> f64 {
        (self.right_upper_lobe.compliance
            + self.right_middle_lobe.compliance
            + self.right_lower_lobe.compliance
            + self.left_upper_lobe.compliance
            + self.left_lower_lobe.compliance)
            / 5.0
    }
}

impl Organ for Lungs {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // Update respiratory cycle
        self.respiratory_cycle_time += delta_time_s;
        let cycle_duration = 60.0 / self.respiration_rate_bpm;

        if self.respiratory_cycle_time >= cycle_duration {
            self.respiratory_cycle_time = 0.0;
        }

        let cycle_progress = self.respiratory_cycle_time / cycle_duration;

        // Determine respiratory phase
        if cycle_progress < 0.4 {
            self.current_phase = RespiratoryPhase::Inspiration;
        } else if cycle_progress < 0.8 {
            self.current_phase = RespiratoryPhase::Expiration;
        } else {
            self.current_phase = RespiratoryPhase::Pause;
        }

        // Calculate gas exchange based on compliance
        let compliance_factor = self.total_compliance();
        let effective_ventilation = self.tidal_volume_ml * compliance_factor;

        // Oxygen saturation
        self.oxygen_saturation_percent = 98.0 * compliance_factor;
        self.oxygen_saturation_percent = self.oxygen_saturation_percent.clamp(70.0, 100.0);

        // CO2 clearance
        let _co2_clearance = effective_ventilation * 0.05;
        self.end_tidal_co2_mmhg = 38.0 + (1.0 - compliance_factor) * 20.0;

        // Update patient blood gases
        patient.blood.gases.sao2_percent = self.oxygen_saturation_percent;
        patient.blood.gases.paco2_mmhg = self.end_tidal_co2_mmhg;
        patient.blood.gases.pao2_mmhg = self.oxygen_saturation_percent * 0.95; // Approximate PaO2 from SpO2

        // Peak inspiratory pressure affected by compliance
        self.peak_inspiratory_pressure = 15.0 / compliance_factor.max(0.1);

        // Generate capnography waveform
        let capno_value = match self.current_phase {
            RespiratoryPhase::Inspiration => 0.0,
            RespiratoryPhase::Expiration => {
                if cycle_progress < 0.6 {
                    self.end_tidal_co2_mmhg * (cycle_progress - 0.4) / 0.2
                } else {
                    self.end_tidal_co2_mmhg
                }
            }
            RespiratoryPhase::Pause => 0.0,
        };

        self.capnography_waveform.push_back(capno_value);
        if self.capnography_waveform.len() > 1000 {
            self.capnography_waveform.pop_front();
        }

        // Respond to blood chemistry
        // High CO2 increases respiration rate
        if patient.blood.gases.paco2_mmhg > 45.0 {
            self.respiration_rate_bpm = 16.0 + (patient.blood.gases.paco2_mmhg - 45.0) * 0.5;
            self.respiration_rate_bpm = self.respiration_rate_bpm.min(30.0);
        } else {
            self.respiration_rate_bpm = 16.0;
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Lungs: RR={:.0} bpm, TV={:.0} mL, SpO2={:.1}%, etCO2={:.1} mmHg, PIP={:.1} cmH2O",
            self.respiration_rate_bpm,
            self.tidal_volume_ml,
            self.oxygen_saturation_percent,
            self.end_tidal_co2_mmhg,
            self.peak_inspiratory_pressure
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Lungs"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
