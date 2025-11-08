//! Liver organ simulation
//!
//! Metabolic processing and detoxification

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Hepatic lobule (functional unit of liver)
#[derive(Debug, Clone)]
pub struct HepaticLobule {
    pub metabolic_capacity: f64,  // 0.0 = damaged, 1.0 = healthy
}

/// Liver organ
#[derive(Debug)]
pub struct Liver {
    id: OrganId,
    /// Hepatic lobules
    pub lobules: Vec<HepaticLobule>,
    /// Bile production rate (mL/min)
    pub bile_production_rate: f64,
    /// Glucose production rate (gluconeogenesis, mg/min)
    pub glucose_production_rate: f64,
    /// ALT enzyme level (U/L)
    pub alt_level: f64,
    /// AST enzyme level (U/L)
    pub ast_level: f64,
    /// Bilirubin level (mg/dL)
    pub bilirubin_level: f64,
    /// Angiotensinogen production (AU/min)
    pub angiotensinogen_production: f64,
}

impl Liver {
    /// Create new liver
    pub fn new(id: OrganId) -> Self {
        let mut lobules = Vec::new();
        for _ in 0..1000 {
            lobules.push(HepaticLobule {
                metabolic_capacity: 1.0,
            });
        }

        Self {
            id,
            lobules,
            bile_production_rate: 40.0,
            glucose_production_rate: 100.0,
            alt_level: 20.0,
            ast_level: 20.0,
            bilirubin_level: 0.5,
            angiotensinogen_production: 10.0,
        }
    }

    /// Calculate average metabolic capacity
    fn average_capacity(&self) -> f64 {
        let total: f64 = self.lobules.iter().map(|l| l.metabolic_capacity).sum();
        total / self.lobules.len() as f64
    }

    /// Inflict damage to lobules
    pub fn inflict_damage(&mut self, damage_percent: f64) {
        let num_to_damage = (self.lobules.len() as f64 * damage_percent / 100.0) as usize;
        for i in 0..num_to_damage.min(self.lobules.len()) {
            self.lobules[i].metabolic_capacity *= 0.5;
        }
    }

    /// Get angiotensinogen level
    pub fn get_angiotensinogen(&self) -> f64 {
        self.angiotensinogen_production
    }
}

impl Organ for Liver {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        let capacity = self.average_capacity();

        // Bile production
        self.bile_production_rate = 40.0 * capacity;

        // Glucose production (when blood glucose is low)
        if patient.blood.chemistry.glucose_mg_dl < 80.0 {
            let glucose_produced = self.glucose_production_rate * capacity * delta_time_s / 60.0;
            patient.blood.chemistry.glucose_mg_dl += glucose_produced * 0.01;
        }

        // Detoxification - remove toxins from blood
        // Liver can clear approximately 1-2 toxin units per second at full capacity
        let detox_rate = 1.5 * capacity * delta_time_s;
        patient.blood.chemistry.toxin_level_au = (patient.blood.chemistry.toxin_level_au - detox_rate).max(0.0);

        // Enzyme levels increase with damage - update both local and blood values
        self.alt_level = 20.0 + (1.0 - capacity) * 200.0;
        self.ast_level = 20.0 + (1.0 - capacity) * 180.0;
        patient.blood.chemistry.alt_u_l = self.alt_level;
        patient.blood.chemistry.ast_u_l = self.ast_level;

        // Bilirubin increases with damage - update both local and blood values
        self.bilirubin_level = 0.5 + (1.0 - capacity) * 5.0;
        patient.blood.chemistry.bilirubin_total_mg_dl = self.bilirubin_level;
        patient.blood.chemistry.bilirubin_direct_mg_dl = self.bilirubin_level * 0.3; // ~30% is direct

        // Angiotensinogen production (RAAS system)
        self.angiotensinogen_production = 10.0 * capacity;
    }

    fn get_summary(&self) -> String {
        format!(
            "Liver: Bile={:.0} mL/min, ALT={:.0} U/L, AST={:.0} U/L, Bili={:.1} mg/dL",
            self.bile_production_rate,
            self.alt_level,
            self.ast_level,
            self.bilirubin_level
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Liver"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
