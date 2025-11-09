//! Kidneys organ simulation with emergent AKI pathophysiology
//!
//! Simulates:
//! - Renal perfusion and autoregulation
//! - Acute kidney injury from hypoperfusion (ATN)
//! - Uremia, electrolyte imbalances, metabolic acidosis
//! - Cascading effects: AKI → hyperkalemia → cardiac arrest

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use crate::tissue_injury::TissuePerfusion;

/// Nephron state - from healthy to necrotic
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NephronState {
    Healthy,
    Ischemic { duration_seconds: f64 },
    ATN,  // Acute tubular necrosis
    Necrotic,
}

impl NephronState {
    fn filtration_efficiency(&self) -> f64 {
        match self {
            NephronState::Healthy => 1.0,
            NephronState::Ischemic { duration_seconds } => {
                (1.0 - duration_seconds / 3600.0 * 0.8).max(0.2)
            }
            NephronState::ATN => 0.1,  // Severely impaired
            NephronState::Necrotic => 0.0,
        }
    }

    fn progress(&mut self, perfusion_adequate: bool, delta_time_s: f64) {
        match self {
            NephronState::Healthy => {
                if !perfusion_adequate {
                    *self = NephronState::Ischemic { duration_seconds: 0.0 };
                }
            }
            NephronState::Ischemic { duration_seconds } => {
                if perfusion_adequate {
                    // Can recover if caught early
                    if *duration_seconds < 1800.0 {  // < 30 min
                        *self = NephronState::Healthy;
                    } else {
                        *self = NephronState::ATN;  // Already damaged
                    }
                } else {
                    *duration_seconds += delta_time_s;
                    // Prolonged ischemia → ATN
                    if *duration_seconds > 3600.0 {  // 1 hour
                        *self = NephronState::ATN;
                    }
                }
            }
            NephronState::ATN => {
                if perfusion_adequate {
                    // ATN can slowly recover over days
                    if rand::random::<f64>() < 0.0001 * delta_time_s {
                        *self = NephronState::Healthy;
                    }
                } else {
                    // Continued ischemia → necrosis
                    if rand::random::<f64>() < 0.0005 * delta_time_s {
                        *self = NephronState::Necrotic;
                    }
                }
            }
            NephronState::Necrotic => {
                // Dead nephrons don't recover
            }
        }
    }
}

/// Nephron (functional unit of kidney)
#[derive(Debug, Clone)]
pub struct Nephron {
    pub state: NephronState,
}

/// Kidneys organ with emergent AKI
#[derive(Debug)]
pub struct Kidneys {
    id: OrganId,
    /// Nephrons (sampled - represents ~1M total)
    pub nephrons: Vec<Nephron>,
    /// Renal tissue perfusion
    pub tissue: TissuePerfusion,
    /// Renal blood flow (mL/min)
    pub renal_blood_flow_ml_per_min: f64,
    /// Baseline RBF (20-25% of cardiac output normally)
    pub baseline_rbf_ml_per_min: f64,
    /// Glomerular filtration rate (mL/min)
    pub gfr_ml_per_min: f64,
    /// Urine output rate (mL/min)
    pub urine_output_rate: f64,
    /// Renin secretion (AU/min)
    pub renin_secretion: f64,
    /// Erythropoietin production (for anemia in CKD)
    pub epo_production: f64,
    /// Uremic toxin accumulation
    pub uremic_toxins_au: f64,
}

impl Kidneys {
    /// Create new kidneys
    pub fn new(id: OrganId) -> Self {
        // Sample 10,000 nephrons to represent ~1 million
        let mut nephrons = Vec::new();
        for _ in 0..10_000 {
            nephrons.push(Nephron {
                state: NephronState::Healthy,
            });
        }

        // Normal RBF is ~1200 mL/min (25% of CO of ~5L/min)
        let baseline_rbf = 1200.0;

        Self {
            id,
            nephrons,
            tissue: TissuePerfusion::new(300.0, 4.0),  // ~300g kidney tissue, high flow
            renal_blood_flow_ml_per_min: baseline_rbf,
            baseline_rbf_ml_per_min: baseline_rbf,
            gfr_ml_per_min: 100.0,
            urine_output_rate: 1.0,
            renin_secretion: 1.0,
            epo_production: 1.0,
            uremic_toxins_au: 0.0,
        }
    }

    /// Calculate average nephron efficiency
    fn average_efficiency(&self) -> f64 {
        let total: f64 = self.nephrons
            .iter()
            .map(|n| n.state.filtration_efficiency())
            .sum();
        total / self.nephrons.len() as f64
    }

    /// Get fraction of healthy nephrons
    fn healthy_fraction(&self) -> f64 {
        self.nephrons
            .iter()
            .filter(|n| matches!(n.state, NephronState::Healthy))
            .count() as f64
            / self.nephrons.len() as f64
    }

    /// Get fraction of necrotic nephrons
    fn necrotic_fraction(&self) -> f64 {
        self.nephrons
            .iter()
            .filter(|n| matches!(n.state, NephronState::Necrotic))
            .count() as f64
            / self.nephrons.len() as f64
    }

    /// Check if in AKI
    pub fn is_aki(&self) -> bool {
        self.gfr_ml_per_min < 50.0
    }

    /// Get AKI stage (1-3)
    pub fn aki_stage(&self) -> u8 {
        let baseline_cr = 0.9;
        let current_cr = 0.9 + (1.0 - self.gfr_ml_per_min / 100.0) * 5.0;

        if current_cr >= baseline_cr * 3.0 {
            3  // Stage 3 - severe
        } else if current_cr >= baseline_cr * 2.0 {
            2  // Stage 2
        } else if current_cr >= baseline_cr * 1.5 {
            1  // Stage 1
        } else {
            0  // No AKI
        }
    }

    /// Get renin secretion (for RAAS system)
    pub fn get_renin_secretion(&self) -> f64 {
        self.renin_secretion
    }
}

impl Organ for Kidneys {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // 1. Calculate renal blood flow from cardiac output and MAP
        let _cardiac_output_ml_per_min = 5000.0;  // Default, could get from heart
        let map = patient.blood.get_mean_arterial_pressure();

        // Renal autoregulation maintains RBF between MAP 80-180 mmHg
        // Below 80 mmHg, RBF drops linearly
        if map >= 80.0 && map <= 180.0 {
            self.renal_blood_flow_ml_per_min = self.baseline_rbf_ml_per_min;
        } else if map < 80.0 {
            // Hypoperfusion - critical for AKI!
            self.renal_blood_flow_ml_per_min = self.baseline_rbf_ml_per_min * (map / 80.0).max(0.0);
        } else {
            // Hypertension damages kidneys over time
            self.renal_blood_flow_ml_per_min = self.baseline_rbf_ml_per_min * (1.0 + (map - 180.0) / 180.0 * 0.1);
        }

        // 2. Update tissue perfusion
        let hgb = patient.blood.cells.hemoglobin_g_dl;
        let sao2 = patient.blood.gases.sao2_percent / 100.0;
        let pao2 = patient.blood.gases.pao2_mmhg;
        let arterial_o2_content = (hgb * 1.34 * sao2) + (0.003 * pao2);

        self.tissue.update(
            self.renal_blood_flow_ml_per_min,
            arterial_o2_content,
            1.0,  // Baseline metabolic rate
            delta_time_s
        );

        // 3. Update individual nephrons based on perfusion
        let perfusion_adequate = self.renal_blood_flow_ml_per_min >= self.baseline_rbf_ml_per_min * 0.7;

        for nephron in &mut self.nephrons {
            nephron.state.progress(perfusion_adequate, delta_time_s);
        }

        // 4. Calculate GFR from nephron function
        let efficiency = self.average_efficiency();
        self.gfr_ml_per_min = (100.0 * efficiency).max(5.0);  // Minimum 5 to avoid division by zero

        // 5. Urine output
        self.urine_output_rate = self.gfr_ml_per_min * 0.01;
        if self.gfr_ml_per_min < 30.0 {
            // Oliguria in severe AKI
            self.urine_output_rate = self.gfr_ml_per_min * 0.005;
        }

        // 6. Update blood chemistry - EMERGENT UREMIA!
        // Creatinine rises as GFR falls
        let creatinine = 0.9 * (100.0 / self.gfr_ml_per_min.max(10.0));
        patient.blood.chemistry.creatinine_mg_dl = creatinine.min(15.0);

        // BUN rises (ratio 10:1 with creatinine in kidney failure)
        let bun = 12.0 + (creatinine - 0.9) * 10.0;
        patient.blood.chemistry.bun_mg_dl = bun.min(150.0);

        // 7. Electrolyte dysregulation - EMERGENT HYPERKALEMIA!
        // Kidneys normally excrete K+ - when they fail, K+ rises
        if self.gfr_ml_per_min < 50.0 {
            // Hyperkalemia develops
            let k_rise = (50.0 - self.gfr_ml_per_min) / 50.0 * 3.0 * delta_time_s / 3600.0;
            patient.blood.chemistry.potassium_meq_l = (patient.blood.chemistry.potassium_meq_l + k_rise).min(8.0);
        } else {
            // Normal K+ regulation
            patient.blood.chemistry.potassium_meq_l = 4.0;
        }

        // Sodium regulation
        if self.gfr_ml_per_min < 30.0 {
            // Dilutional hyponatremia in severe AKI
            patient.blood.chemistry.sodium_meq_l = (140.0 - (30.0 - self.gfr_ml_per_min) / 30.0 * 10.0).max(125.0);
        } else {
            patient.blood.chemistry.sodium_meq_l = 140.0;
        }

        // 8. Metabolic acidosis from reduced H+ excretion
        if self.gfr_ml_per_min < 50.0 {
            // Acidosis develops - reduce HCO3-
            let hco3_drop = (50.0 - self.gfr_ml_per_min) / 50.0 * 0.5 * delta_time_s / 3600.0;
            patient.blood.chemistry.bicarbonate_meq_l = (patient.blood.chemistry.bicarbonate_meq_l - hco3_drop).max(10.0);

            // pH drops (calculated from HCO3 and CO2)
            let pco2 = patient.blood.gases.paco2_mmhg;
            patient.blood.gases.ph = 6.1 + ((patient.blood.chemistry.bicarbonate_meq_l / (0.03 * pco2))).log10();
        }

        // 9. Uremic toxin accumulation
        if self.gfr_ml_per_min < 60.0 {
            // Toxins accumulate as kidneys fail
            self.uremic_toxins_au += (60.0 - self.gfr_ml_per_min) / 60.0 * 0.01 * delta_time_s;
        } else {
            // Toxins cleared normally
            self.uremic_toxins_au *= 0.99_f64.powf(delta_time_s);
        }

        // Uremic toxins cause symptoms and organ damage
        patient.blood.chemistry.toxin_level_au += self.uremic_toxins_au * 0.01 * delta_time_s;

        // 10. Toxin clearance (when kidneys work)
        let toxin_clearance = (self.gfr_ml_per_min / 100.0) * 0.5 * delta_time_s;
        patient.blood.chemistry.toxin_level_au = (patient.blood.chemistry.toxin_level_au - toxin_clearance).max(0.0);

        // 11. RAAS activation (renin secretion)
        if map < 90.0 || self.tissue.perfusion_ratio() < 0.8 {
            self.renin_secretion = 1.0 + (1.0 - self.tissue.perfusion_ratio()) * 2.0;
        } else {
            self.renin_secretion = 1.0;
        }

        // 12. EPO production (reduced in kidney disease → anemia)
        self.epo_production = efficiency;
        if efficiency < 0.5 {
            // Anemia of CKD develops
            let rbc_drop = 0.0001 * delta_time_s;
            patient.blood.cells.rbc_count_million_per_ul = (patient.blood.cells.rbc_count_million_per_ul - rbc_drop).max(2.0);
            patient.blood.cells.hemoglobin_g_dl = patient.blood.cells.rbc_count_million_per_ul * 2.9;
        }

        // 13. Fluid overload in severe AKI (affects blood volume and pressure)
        if self.urine_output_rate < 0.5 {
            // Oliguria → volume overload
            patient.blood.blood_pressure_systolic += 0.1 * delta_time_s;
        }
    }

    fn get_summary(&self) -> String {
        let aki = if self.is_aki() {
            format!(" [AKI Stage {}]", self.aki_stage())
        } else {
            String::new()
        };

        format!(
            "Kidneys: GFR={:.1} mL/min{}, RBF={:.0}/{:.0} mL/min, UOP={:.2} mL/min, Healthy nephrons={:.0}%, Necrotic={:.0}%",
            self.gfr_ml_per_min,
            aki,
            self.renal_blood_flow_ml_per_min,
            self.baseline_rbf_ml_per_min,
            self.urine_output_rate,
            self.healthy_fraction() * 100.0,
            self.necrotic_fraction() * 100.0
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
