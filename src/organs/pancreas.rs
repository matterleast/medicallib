//! Pancreas organ simulation
//!
//! Dual function: endocrine (hormones) and exocrine (digestive enzymes)

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Digestive enzymes
#[derive(Debug, Clone)]
pub struct DigestiveEnzymes {
    pub volume_ml: f64,
    pub amylase_concentration: f64,
    pub lipase_concentration: f64,
}

/// Pancreas organ
#[derive(Debug)]
pub struct Pancreas {
    id: OrganId,
    /// Insulin secretion rate (units/min)
    pub insulin_secretion_rate: f64,
    /// Glucagon secretion rate (units/min)
    pub glucagon_secretion_rate: f64,
    /// Digestive enzymes production
    pub digestive_enzymes: DigestiveEnzymes,
    /// Enzyme production rate (mL/min)
    pub enzyme_production_rate: f64,
}

impl Pancreas {
    /// Create new pancreas
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            insulin_secretion_rate: 1.0,
            glucagon_secretion_rate: 0.5,
            digestive_enzymes: DigestiveEnzymes {
                volume_ml: 0.0,
                amylase_concentration: 1.0,
                lipase_concentration: 1.0,
            },
            enzyme_production_rate: 5.0,
        }
    }
}

impl Organ for Pancreas {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // Endocrine function: regulate blood glucose
        let glucose_error = patient.blood.chemistry.glucose_mg_dl - 90.0;

        if glucose_error > 0.0 {
            // High glucose: secrete insulin
            self.insulin_secretion_rate = 1.0 + glucose_error * 0.05;
            self.glucagon_secretion_rate = 0.5;

            // Insulin lowers blood glucose
            let glucose_consumed = self.insulin_secretion_rate * delta_time_s / 60.0;
            patient.blood.chemistry.glucose_mg_dl -= glucose_consumed;
        } else {
            // Low glucose: secrete glucagon
            self.insulin_secretion_rate = 0.5;
            self.glucagon_secretion_rate = 1.0 - glucose_error * 0.05;

            // Glucagon raises blood glucose
            let glucose_produced = self.glucagon_secretion_rate * delta_time_s / 60.0;
            patient.blood.chemistry.glucose_mg_dl += glucose_produced;
        }

        patient.blood.chemistry.glucose_mg_dl = patient.blood.chemistry.glucose_mg_dl.clamp(60.0, 200.0);

        // Exocrine function: produce digestive enzymes
        let enzyme_produced = self.enzyme_production_rate * delta_time_s / 60.0;
        self.digestive_enzymes.volume_ml += enzyme_produced;
    }

    fn get_summary(&self) -> String {
        format!(
            "Pancreas: Insulin={:.1} U/min, Glucagon={:.1} U/min, Enzymes={:.0} mL",
            self.insulin_secretion_rate,
            self.glucagon_secretion_rate,
            self.digestive_enzymes.volume_ml
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Pancreas"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
