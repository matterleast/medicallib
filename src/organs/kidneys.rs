//! Kidneys organ simulation
//!
//! Blood filtration and electrolyte balance

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Nephron (functional unit of kidney)
#[derive(Debug, Clone)]
pub struct Nephron {
    pub filtration_efficiency: f64,  // 0.0 = non-functional, 1.0 = normal
}

/// Kidneys organ
#[derive(Debug)]
pub struct Kidneys {
    id: OrganId,
    /// Nephrons (functional units)
    pub nephrons: Vec<Nephron>,
    /// Glomerular filtration rate (mL/min)
    pub gfr_ml_per_min: f64,
    /// Urine output rate (mL/min)
    pub urine_output_rate: f64,
    /// Blood sodium level (mEq/L)
    pub blood_sodium_meq_l: f64,
    /// Blood potassium level (mEq/L)
    pub blood_potassium_meq_l: f64,
    /// Renin secretion (AU/min)
    pub renin_secretion: f64,
}

impl Kidneys {
    /// Create new kidneys
    pub fn new(id: OrganId) -> Self {
        let mut nephrons = Vec::new();
        for _ in 0..1_000_000 {
            nephrons.push(Nephron {
                filtration_efficiency: 1.0,
            });
        }

        Self {
            id,
            nephrons,
            gfr_ml_per_min: 100.0,
            urine_output_rate: 1.0,
            blood_sodium_meq_l: 140.0,
            blood_potassium_meq_l: 4.0,
            renin_secretion: 1.0,
        }
    }

    /// Calculate average nephron efficiency
    fn average_efficiency(&self) -> f64 {
        let sample_size = 1000.min(self.nephrons.len());
        let total: f64 = self.nephrons[..sample_size]
            .iter()
            .map(|n| n.filtration_efficiency)
            .sum();
        total / sample_size as f64
    }

    /// Damage nephrons
    pub fn damage_nephrons(&mut self, damage_percent: f64) {
        let num_to_damage = (self.nephrons.len() as f64 * damage_percent / 100.0) as usize;
        for i in 0..num_to_damage.min(self.nephrons.len()) {
            self.nephrons[i].filtration_efficiency *= 0.5;
        }
    }

    /// Get renin secretion (for RAAS system)
    pub fn get_renin_secretion(&self) -> f64 {
        self.renin_secretion
    }
}

impl Organ for Kidneys {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        let efficiency = self.average_efficiency();

        // GFR based on nephron efficiency
        self.gfr_ml_per_min = 100.0 * efficiency;

        // Urine output
        self.urine_output_rate = self.gfr_ml_per_min * 0.01;

        // Maintain electrolyte balance - update both local and blood values
        self.blood_sodium_meq_l = 140.0;
        self.blood_potassium_meq_l = 4.0;
        patient.blood.chemistry.sodium_meq_l = self.blood_sodium_meq_l;
        patient.blood.chemistry.potassium_meq_l = self.blood_potassium_meq_l;

        // Update creatinine based on kidney function
        patient.blood.chemistry.creatinine_mg_dl = 0.9 + (1.0 - self.gfr_ml_per_min / 120.0) * 3.0;
        patient.blood.chemistry.bun_mg_dl = 12.0 + (1.0 - self.gfr_ml_per_min / 120.0) * 30.0;

        // Renin secretion (RAAS system)
        // Increase renin when blood pressure is low
        let map = patient.blood.blood_pressure_diastolic
            + (patient.blood.blood_pressure_systolic - patient.blood.blood_pressure_diastolic) / 3.0;

        if map < 90.0 {
            self.renin_secretion = 1.0 + (90.0 - map) * 0.1;
        } else {
            self.renin_secretion = 1.0;
        }

        // Remove some toxins through filtration
        // Kidneys can clear approximately 0.5 toxin units per second at normal GFR
        let toxin_clearance = (self.gfr_ml_per_min / 100.0) * 0.5 * delta_time_s;
        patient.blood.chemistry.toxin_level_au = (patient.blood.chemistry.toxin_level_au - toxin_clearance).max(0.0);
    }

    fn get_summary(&self) -> String {
        format!(
            "Kidneys: GFR={:.1} mL/min, Urine={:.2} mL/min, Na+={:.0} mEq/L, K+={:.1} mEq/L",
            self.gfr_ml_per_min,
            self.urine_output_rate,
            self.blood_sodium_meq_l,
            self.blood_potassium_meq_l
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Kidneys"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
