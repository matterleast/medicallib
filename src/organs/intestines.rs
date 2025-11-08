//! Intestines organ simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;

/// Intestinal segment
#[derive(Debug, Clone)]
pub struct IntestinalSegment {
    pub name: String,
    pub chyme_volume_ml: f64,
    pub absorption_rate: f64,
}

/// Intestines organ
#[derive(Debug)]
pub struct Intestines {
    id: OrganId,
    /// Duodenum (first part of small intestine)
    pub duodenum: IntestinalSegment,
    /// Jejunum (middle part of small intestine)
    pub jejunum: IntestinalSegment,
    /// Ileum (last part of small intestine)
    pub ileum: IntestinalSegment,
    /// Colon (large intestine)
    pub colon: IntestinalSegment,
    /// Nutrient absorption rate (mg/min)
    pub nutrient_absorption_rate: f64,
    /// Water absorption rate (mL/min)
    pub water_absorption_rate: f64,
    /// Motility (0.0 = no movement, 1.0 = normal)
    pub motility: f64,
}

impl Intestines {
    /// Create new intestines
    pub fn new(id: OrganId) -> Self {
        Self {
            id,
            duodenum: IntestinalSegment {
                name: "Duodenum".to_string(),
                chyme_volume_ml: 0.0,
                absorption_rate: 1.0,
            },
            jejunum: IntestinalSegment {
                name: "Jejunum".to_string(),
                chyme_volume_ml: 0.0,
                absorption_rate: 1.5,
            },
            ileum: IntestinalSegment {
                name: "Ileum".to_string(),
                chyme_volume_ml: 0.0,
                absorption_rate: 1.0,
            },
            colon: IntestinalSegment {
                name: "Colon".to_string(),
                chyme_volume_ml: 0.0,
                absorption_rate: 0.5,
            },
            nutrient_absorption_rate: 100.0,
            water_absorption_rate: 50.0,
            motility: 1.0,
        }
    }

    /// Receive chyme from stomach
    pub fn receive_chyme(&mut self, volume_ml: f64) {
        self.duodenum.chyme_volume_ml += volume_ml;
    }
}

impl Organ for Intestines {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // Move chyme through segments
        let transfer_rate = 10.0 * self.motility * delta_time_s / 60.0;

        // Duodenum -> Jejunum
        let transfer = self.duodenum.chyme_volume_ml.min(transfer_rate);
        self.duodenum.chyme_volume_ml -= transfer;
        self.jejunum.chyme_volume_ml += transfer;

        // Jejunum -> Ileum
        let transfer = self.jejunum.chyme_volume_ml.min(transfer_rate);
        self.jejunum.chyme_volume_ml -= transfer;
        self.ileum.chyme_volume_ml += transfer;

        // Ileum -> Colon
        let transfer = self.ileum.chyme_volume_ml.min(transfer_rate);
        self.ileum.chyme_volume_ml -= transfer;
        self.colon.chyme_volume_ml += transfer;

        // Absorption in jejunum (main absorption site)
        let nutrient_absorbed = self.nutrient_absorption_rate
            * self.jejunum.absorption_rate
            * delta_time_s / 60.0;

        // Increase blood glucose from nutrient absorption
        patient.blood.blood_glucose_mg_dl += nutrient_absorbed * 0.01;

        // Water absorption in colon
        let water_absorbed = self.water_absorption_rate * delta_time_s / 60.0;
        self.colon.chyme_volume_ml = (self.colon.chyme_volume_ml - water_absorbed).max(0.0);
    }

    fn get_summary(&self) -> String {
        format!(
            "Intestines: Motility={:.1}, Duodenum={:.0}mL, Jejunum={:.0}mL, Ileum={:.0}mL, Colon={:.0}mL",
            self.motility,
            self.duodenum.chyme_volume_ml,
            self.jejunum.chyme_volume_ml,
            self.ileum.chyme_volume_ml,
            self.colon.chyme_volume_ml
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Intestines"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
