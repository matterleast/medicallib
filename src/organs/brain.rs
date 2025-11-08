//! Brain organ simulation
//!
//! Simulates neurological vitals and autonomic control including:
//! - 5 brain regions
//! - Glasgow Coma Scale (GCS)
//! - Intracranial pressure (ICP)
//! - Cerebral perfusion pressure (CPP)
//! - EEG waveform

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use std::collections::VecDeque;

/// Brain region
#[derive(Debug, Clone)]
pub struct BrainRegion {
    pub name: String,
    pub metabolic_activity: f64,  // 0.0 = inactive, 1.0 = normal
    pub blood_flow_ml_per_min: f64,
}

/// Glasgow Coma Scale components
#[derive(Debug, Clone)]
pub struct GlasgowComaScale {
    pub eye_response: i32,      // 1-4
    pub verbal_response: i32,   // 1-5
    pub motor_response: i32,    // 1-6
}

impl GlasgowComaScale {
    /// Get total GCS score (3-15)
    pub fn total(&self) -> i32 {
        self.eye_response + self.verbal_response + self.motor_response
    }

    /// Get GCS category
    pub fn category(&self) -> &'static str {
        match self.total() {
            3..=8 => "Severe",
            9..=12 => "Moderate",
            13..=15 => "Minor",
            _ => "Invalid",
        }
    }
}

impl Default for GlasgowComaScale {
    fn default() -> Self {
        Self {
            eye_response: 4,
            verbal_response: 5,
            motor_response: 6,
        }
    }
}

/// Brain organ
#[derive(Debug)]
pub struct Brain {
    id: OrganId,
    /// Frontal lobe
    pub frontal_lobe: BrainRegion,
    /// Parietal lobe
    pub parietal_lobe: BrainRegion,
    /// Temporal lobe
    pub temporal_lobe: BrainRegion,
    /// Occipital lobe
    pub occipital_lobe: BrainRegion,
    /// Cerebellum
    pub cerebellum: BrainRegion,
    /// Glasgow Coma Scale
    pub gcs: GlasgowComaScale,
    /// Intracranial pressure (mmHg)
    pub intracranial_pressure_mmhg: f64,
    /// Cerebral perfusion pressure (mmHg)
    pub cerebral_perfusion_pressure_mmhg: f64,
    /// EEG waveform
    pub eeg_waveform: VecDeque<f64>,
    /// Autonomic control of heart rate
    pub autonomic_heart_rate_target: f64,
    /// Autonomic control of respiration
    pub autonomic_respiration_target: f64,
}

impl Brain {
    /// Create new brain
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            frontal_lobe: BrainRegion {
                name: "Frontal".to_string(),
                metabolic_activity: 1.0,
                blood_flow_ml_per_min: 50.0,
            },
            parietal_lobe: BrainRegion {
                name: "Parietal".to_string(),
                metabolic_activity: 1.0,
                blood_flow_ml_per_min: 45.0,
            },
            temporal_lobe: BrainRegion {
                name: "Temporal".to_string(),
                metabolic_activity: 1.0,
                blood_flow_ml_per_min: 45.0,
            },
            occipital_lobe: BrainRegion {
                name: "Occipital".to_string(),
                metabolic_activity: 1.0,
                blood_flow_ml_per_min: 40.0,
            },
            cerebellum: BrainRegion {
                name: "Cerebellum".to_string(),
                metabolic_activity: 1.0,
                blood_flow_ml_per_min: 30.0,
            },
            gcs: GlasgowComaScale::default(),
            intracranial_pressure_mmhg: 10.0,
            cerebral_perfusion_pressure_mmhg: 70.0,
            eeg_waveform: VecDeque::with_capacity(1000),
            autonomic_heart_rate_target: 75.0,
            autonomic_respiration_target: 16.0,
        }
    }

    /// Calculate average metabolic activity
    fn average_metabolic_activity(&self) -> f64 {
        (self.frontal_lobe.metabolic_activity
            + self.parietal_lobe.metabolic_activity
            + self.temporal_lobe.metabolic_activity
            + self.occipital_lobe.metabolic_activity
            + self.cerebellum.metabolic_activity)
            / 5.0
    }
}

impl Organ for Brain {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // Calculate cerebral perfusion pressure
        // CPP = MAP - ICP (where MAP = mean arterial pressure)
        let map = patient.blood.blood_pressure_diastolic
            + (patient.blood.blood_pressure_systolic - patient.blood.blood_pressure_diastolic) / 3.0;
        self.cerebral_perfusion_pressure_mmhg = map - self.intracranial_pressure_mmhg;

        // Update metabolic activity based on perfusion
        let perfusion_factor = (self.cerebral_perfusion_pressure_mmhg / 70.0).clamp(0.0, 1.5);
        let oxygen_factor = (patient.blood.oxygen_saturation_percent / 98.0).clamp(0.0, 1.0);

        self.frontal_lobe.metabolic_activity = perfusion_factor * oxygen_factor;
        self.parietal_lobe.metabolic_activity = perfusion_factor * oxygen_factor;
        self.temporal_lobe.metabolic_activity = perfusion_factor * oxygen_factor;
        self.occipital_lobe.metabolic_activity = perfusion_factor * oxygen_factor;
        self.cerebellum.metabolic_activity = perfusion_factor * oxygen_factor;

        // Update GCS based on metabolic activity
        let avg_activity = self.average_metabolic_activity();

        if avg_activity >= 0.9 {
            self.gcs.eye_response = 4;
            self.gcs.verbal_response = 5;
            self.gcs.motor_response = 6;
        } else if avg_activity >= 0.7 {
            self.gcs.eye_response = 3;
            self.gcs.verbal_response = 4;
            self.gcs.motor_response = 5;
        } else if avg_activity >= 0.5 {
            self.gcs.eye_response = 2;
            self.gcs.verbal_response = 3;
            self.gcs.motor_response = 4;
        } else if avg_activity >= 0.3 {
            self.gcs.eye_response = 2;
            self.gcs.verbal_response = 2;
            self.gcs.motor_response = 3;
        } else {
            self.gcs.eye_response = 1;
            self.gcs.verbal_response = 1;
            self.gcs.motor_response = 2;
        }

        // ICP affected by blood pressure
        self.intracranial_pressure_mmhg = 10.0 + (map - 93.0) * 0.1;
        self.intracranial_pressure_mmhg = self.intracranial_pressure_mmhg.clamp(5.0, 30.0);

        // Generate EEG waveform (simplified)
        let eeg_amplitude = avg_activity * 50.0;
        let eeg_value = eeg_amplitude * (delta_time_s * 10.0 * std::f64::consts::PI).sin();
        self.eeg_waveform.push_back(eeg_value);
        if self.eeg_waveform.len() > 1000 {
            self.eeg_waveform.pop_front();
        }

        // Autonomic control
        // High CO2 increases both heart rate and respiration
        if patient.blood.paco2_mmhg > 45.0 {
            self.autonomic_heart_rate_target = 75.0 + (patient.blood.paco2_mmhg - 45.0) * 0.5;
            self.autonomic_respiration_target = 16.0 + (patient.blood.paco2_mmhg - 45.0) * 0.3;
        } else {
            self.autonomic_heart_rate_target = 75.0;
            self.autonomic_respiration_target = 16.0;
        }

        // Low oxygen increases heart rate
        if patient.blood.oxygen_saturation_percent < 95.0 {
            self.autonomic_heart_rate_target += (95.0 - patient.blood.oxygen_saturation_percent) * 0.5;
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Brain: GCS={} (E{}V{}M{}), ICP={:.1} mmHg, CPP={:.1} mmHg",
            self.gcs.total(),
            self.gcs.eye_response,
            self.gcs.verbal_response,
            self.gcs.motor_response,
            self.intracranial_pressure_mmhg,
            self.cerebral_perfusion_pressure_mmhg
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Brain"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
