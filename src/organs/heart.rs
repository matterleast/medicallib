//! Heart organ simulation
//!
//! Simulates cardiac electrical and mechanical function including:
//! - 4 chambers (left/right atrium and ventricle)
//! - 4 valves (mitral, tricuspid, aortic, pulmonary)
//! - EKG waveform generation
//! - Blood pressure regulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use std::collections::VecDeque;

/// Chamber state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChamberState {
    Systole,
    Diastole,
}

/// Heart valve
#[derive(Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub is_open: bool,
    pub stenosis_severity: f64,  // 0.0 = normal, 1.0 = complete stenosis
    pub regurgitation_severity: f64,  // 0.0 = normal, 1.0 = severe regurgitation
}

/// Heart chamber
#[derive(Debug, Clone)]
pub struct Chamber {
    pub name: String,
    pub state: ChamberState,
    pub volume_ml: f64,
    pub pressure_mmhg: f64,
}

/// Heart organ
#[derive(Debug)]
pub struct Heart {
    id: OrganId,
    /// Left atrium
    pub left_atrium: Chamber,
    /// Right atrium
    pub right_atrium: Chamber,
    /// Left ventricle
    pub left_ventricle: Chamber,
    /// Right ventricle
    pub right_ventricle: Chamber,
    /// Mitral valve (left atrium to left ventricle)
    pub mitral_valve: Valve,
    /// Tricuspid valve (right atrium to right ventricle)
    pub tricuspid_valve: Valve,
    /// Aortic valve (left ventricle to aorta)
    pub aortic_valve: Valve,
    /// Pulmonary valve (right ventricle to pulmonary artery)
    pub pulmonary_valve: Valve,
    /// Heart rate (beats per minute)
    pub heart_rate_bpm: f64,
    /// Ejection fraction (percentage)
    pub ejection_fraction_percent: f64,
    /// Aortic pressure (systolic/diastolic)
    pub aortic_pressure_systolic: f64,
    pub aortic_pressure_diastolic: f64,
    /// EKG lead data
    pub ekg_leads: Vec<VecDeque<f64>>,
    /// Internal cardiac cycle timer
    cardiac_cycle_time: f64,
}

impl Heart {
    /// Create a new heart
    pub fn new(id: OrganId, num_leads: usize) -> Self {
        let num_leads = match num_leads {
            3 | 5 | 12 => num_leads,
            _ => 12,  // Default to 12-lead
        };

        let mut ekg_leads = Vec::new();
        for _ in 0..num_leads {
            ekg_leads.push(VecDeque::with_capacity(1000));
        }

        Self {
            id,
            left_atrium: Chamber {
                name: "Left Atrium".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 50.0,
                pressure_mmhg: 8.0,
            },
            right_atrium: Chamber {
                name: "Right Atrium".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 50.0,
                pressure_mmhg: 4.0,
            },
            left_ventricle: Chamber {
                name: "Left Ventricle".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 120.0,
                pressure_mmhg: 8.0,
            },
            right_ventricle: Chamber {
                name: "Right Ventricle".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 120.0,
                pressure_mmhg: 4.0,
            },
            mitral_valve: Valve {
                name: "Mitral".to_string(),
                is_open: true,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            tricuspid_valve: Valve {
                name: "Tricuspid".to_string(),
                is_open: true,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            aortic_valve: Valve {
                name: "Aortic".to_string(),
                is_open: false,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            pulmonary_valve: Valve {
                name: "Pulmonary".to_string(),
                is_open: false,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            heart_rate_bpm: 75.0,
            ejection_fraction_percent: 60.0,
            aortic_pressure_systolic: 120.0,
            aortic_pressure_diastolic: 80.0,
            ekg_leads,
            cardiac_cycle_time: 0.0,
        }
    }

    /// Generate EKG waveform value
    fn generate_ekg(&self, lead_index: usize) -> f64 {
        let cycle_progress = self.cardiac_cycle_time / (60.0 / self.heart_rate_bpm);
        let phase = cycle_progress * 2.0 * std::f64::consts::PI;

        // Simple EKG waveform approximation
        let p_wave = if cycle_progress < 0.2 {
            0.2 * (phase * 5.0).sin()
        } else {
            0.0
        };

        let qrs_complex = if cycle_progress > 0.3 && cycle_progress < 0.4 {
            1.0 * ((phase - 2.0) * 10.0).sin()
        } else {
            0.0
        };

        let t_wave = if cycle_progress > 0.5 && cycle_progress < 0.7 {
            0.3 * ((phase - 3.5) * 5.0).sin()
        } else {
            0.0
        };

        // Different leads have different amplitudes
        let amplitude_factor = match lead_index {
            0 => 1.0,
            1 => 0.8,
            2 => 0.9,
            _ => 0.7 + (lead_index as f64 * 0.05),
        };

        (p_wave + qrs_complex + t_wave) * amplitude_factor
    }
}

impl Organ for Heart {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // Update cardiac cycle
        self.cardiac_cycle_time += delta_time_s;
        let cycle_duration = 60.0 / self.heart_rate_bpm;

        if self.cardiac_cycle_time >= cycle_duration {
            self.cardiac_cycle_time = 0.0;
        }

        let cycle_progress = self.cardiac_cycle_time / cycle_duration;

        // Atrial systole (0.0 - 0.2)
        if cycle_progress < 0.2 {
            self.left_atrium.state = ChamberState::Systole;
            self.right_atrium.state = ChamberState::Systole;
            self.mitral_valve.is_open = true;
            self.tricuspid_valve.is_open = true;
        }
        // Ventricular systole (0.2 - 0.5)
        else if cycle_progress < 0.5 {
            self.left_atrium.state = ChamberState::Diastole;
            self.right_atrium.state = ChamberState::Diastole;
            self.left_ventricle.state = ChamberState::Systole;
            self.right_ventricle.state = ChamberState::Systole;
            self.mitral_valve.is_open = false;
            self.tricuspid_valve.is_open = false;
            self.aortic_valve.is_open = true;
            self.pulmonary_valve.is_open = true;
        }
        // Diastole (0.5 - 1.0)
        else {
            self.left_ventricle.state = ChamberState::Diastole;
            self.right_ventricle.state = ChamberState::Diastole;
            self.aortic_valve.is_open = false;
            self.pulmonary_valve.is_open = false;
            self.mitral_valve.is_open = true;
            self.tricuspid_valve.is_open = true;
        }

        // Calculate pressures
        if self.left_ventricle.state == ChamberState::Systole {
            self.aortic_pressure_systolic = 100.0 + self.ejection_fraction_percent * 0.5;
            self.left_ventricle.pressure_mmhg = self.aortic_pressure_systolic;
        } else {
            self.aortic_pressure_diastolic = 70.0 + self.ejection_fraction_percent * 0.2;
            self.left_ventricle.pressure_mmhg = 8.0;
        }

        // Update patient blood pressure
        patient.blood.blood_pressure_systolic = self.aortic_pressure_systolic;
        patient.blood.blood_pressure_diastolic = self.aortic_pressure_diastolic;

        // Generate EKG data
        let num_leads = self.ekg_leads.len();
        let ekg_values: Vec<f64> = (0..num_leads).map(|i| self.generate_ekg(i)).collect();
        for (lead, &ekg_value) in self.ekg_leads.iter_mut().zip(ekg_values.iter()) {
            lead.push_back(ekg_value);
            if lead.len() > 1000 {
                lead.pop_front();
            }
        }

        // Respond to blood chemistry
        // High toxins reduce heart rate and ejection fraction
        if patient.blood.toxin_level_au > 50.0 {
            self.heart_rate_bpm = (75.0 - patient.blood.toxin_level_au * 0.1).max(40.0);
            self.ejection_fraction_percent = (60.0 - patient.blood.toxin_level_au * 0.05).max(30.0);
        } else {
            self.heart_rate_bpm = 75.0;
            self.ejection_fraction_percent = 60.0;
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Heart: HR={:.0} bpm, EF={:.0}%, BP={:.0}/{:.0} mmHg",
            self.heart_rate_bpm,
            self.ejection_fraction_percent,
            self.aortic_pressure_systolic,
            self.aortic_pressure_diastolic
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Heart"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
