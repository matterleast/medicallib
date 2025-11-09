//! Comprehensive blood system based on real blood characteristics
//!
//! This module provides detailed simulation of blood components including:
//! - Blood typing (ABO and Rh)
//! - Complete blood count (CBC) with differential
//! - Blood chemistry panel
//! - Coagulation factors
//! - Arterial blood gas (ABG) analysis

use std::fmt;

/// ABO blood type system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AboType {
    O,
    A,
    B,
    AB,
}

impl fmt::Display for AboType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AboType::O => write!(f, "O"),
            AboType::A => write!(f, "A"),
            AboType::B => write!(f, "B"),
            AboType::AB => write!(f, "AB"),
        }
    }
}

/// Rh factor (positive or negative)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl fmt::Display for RhFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RhFactor::Positive => write!(f, "+"),
            RhFactor::Negative => write!(f, "-"),
        }
    }
}

/// Complete blood type (ABO + Rh)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BloodType {
    pub abo: AboType,
    pub rh: RhFactor,
}

impl BloodType {
    pub fn new(abo: AboType, rh: RhFactor) -> Self {
        Self { abo, rh }
    }
}

impl fmt::Display for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.abo, self.rh)
    }
}

impl Default for BloodType {
    fn default() -> Self {
        // O+ is the most common blood type
        Self {
            abo: AboType::O,
            rh: RhFactor::Positive,
        }
    }
}

/// White blood cell differential (types of WBCs)
#[derive(Debug, Clone)]
pub struct WbcDifferential {
    /// Neutrophils (cells/µL) - fight bacterial infections
    pub neutrophils: f64,
    /// Lymphocytes (cells/µL) - immune response (T cells, B cells, NK cells)
    pub lymphocytes: f64,
    /// Monocytes (cells/µL) - become macrophages
    pub monocytes: f64,
    /// Eosinophils (cells/µL) - fight parasites and allergies
    pub eosinophils: f64,
    /// Basophils (cells/µL) - release histamine in allergic reactions
    pub basophils: f64,
}

impl WbcDifferential {
    /// Get total WBC count
    pub fn total_count(&self) -> f64 {
        self.neutrophils + self.lymphocytes + self.monocytes + self.eosinophils + self.basophils
    }
}

impl Default for WbcDifferential {
    fn default() -> Self {
        // Normal adult WBC differential (cells/µL)
        Self {
            neutrophils: 4000.0,  // 40-70% of WBC (normal: 1800-7800)
            lymphocytes: 2500.0,  // 20-40% of WBC (normal: 1000-4800)
            monocytes: 500.0,     // 2-8% of WBC (normal: 200-1000)
            eosinophils: 200.0,   // 1-4% of WBC (normal: 0-450)
            basophils: 50.0,      // 0.5-1% of WBC (normal: 0-200)
        }
    }
}

/// Complete blood count (CBC) - blood cell components
#[derive(Debug, Clone)]
pub struct BloodCells {
    /// Red blood cell count (million cells/µL)
    /// Normal: Male 4.7-6.1, Female 4.2-5.4
    pub rbc_count_million_per_ul: f64,

    /// Hemoglobin concentration (g/dL) - oxygen-carrying protein in RBCs
    /// Normal: Male 13.8-17.2, Female 12.1-15.1
    pub hemoglobin_g_dl: f64,

    /// Hematocrit (%) - percentage of blood volume that is RBCs
    /// Normal: Male 40.7-50.3, Female 36.1-44.3
    pub hematocrit_percent: f64,

    /// Mean corpuscular volume (fL) - average RBC size
    /// Normal: 80-100 fL
    pub mcv_fl: f64,

    /// Mean corpuscular hemoglobin (pg) - average hemoglobin per RBC
    /// Normal: 27-31 pg
    pub mch_pg: f64,

    /// Mean corpuscular hemoglobin concentration (g/dL)
    /// Normal: 32-36 g/dL
    pub mchc_g_dl: f64,

    /// Red cell distribution width (%) - variation in RBC size
    /// Normal: 11.5-14.5%
    pub rdw_percent: f64,

    /// White blood cell differential
    pub wbc_differential: WbcDifferential,

    /// Platelet count (thousand cells/µL)
    /// Normal: 150-400 thousand/µL
    pub platelet_count_thousand_per_ul: f64,

    /// Mean platelet volume (fL)
    /// Normal: 7.5-11.5 fL
    pub mpv_fl: f64,
}

impl Default for BloodCells {
    fn default() -> Self {
        // Normal adult values (averaged for male/female)
        Self {
            rbc_count_million_per_ul: 5.0,
            hemoglobin_g_dl: 14.5,
            hematocrit_percent: 42.0,
            mcv_fl: 90.0,
            mch_pg: 29.0,
            mchc_g_dl: 34.0,
            rdw_percent: 13.0,
            wbc_differential: WbcDifferential::default(),
            platelet_count_thousand_per_ul: 250.0,
            mpv_fl: 9.5,
        }
    }
}

/// Comprehensive metabolic panel and blood chemistry
#[derive(Debug, Clone)]
pub struct BloodChemistry {
    /// Blood glucose (mg/dL)
    /// Normal fasting: 70-100 mg/dL
    pub glucose_mg_dl: f64,

    /// Blood urea nitrogen (mg/dL) - kidney function marker
    /// Normal: 7-20 mg/dL
    pub bun_mg_dl: f64,

    /// Creatinine (mg/dL) - kidney function marker
    /// Normal: 0.6-1.2 mg/dL
    pub creatinine_mg_dl: f64,

    /// Sodium (mEq/L)
    /// Normal: 136-144 mEq/L
    pub sodium_meq_l: f64,

    /// Potassium (mEq/L)
    /// Normal: 3.5-5.0 mEq/L
    pub potassium_meq_l: f64,

    /// Chloride (mEq/L)
    /// Normal: 96-106 mEq/L
    pub chloride_meq_l: f64,

    /// Bicarbonate (mEq/L) - acid-base balance
    /// Normal: 23-29 mEq/L
    pub bicarbonate_meq_l: f64,

    /// Calcium (mg/dL)
    /// Normal: 8.5-10.2 mg/dL
    pub calcium_mg_dl: f64,

    /// Magnesium (mg/dL)
    /// Normal: 1.7-2.2 mg/dL
    pub magnesium_mg_dl: f64,

    /// Phosphate (mg/dL)
    /// Normal: 2.5-4.5 mg/dL
    pub phosphate_mg_dl: f64,

    /// Total protein (g/dL)
    /// Normal: 6.0-8.3 g/dL
    pub total_protein_g_dl: f64,

    /// Albumin (g/dL) - major blood protein
    /// Normal: 3.5-5.5 g/dL
    pub albumin_g_dl: f64,

    /// Total bilirubin (mg/dL) - liver function
    /// Normal: 0.1-1.2 mg/dL
    pub bilirubin_total_mg_dl: f64,

    /// Direct bilirubin (mg/dL)
    /// Normal: 0.0-0.3 mg/dL
    pub bilirubin_direct_mg_dl: f64,

    /// Alanine aminotransferase (U/L) - liver enzyme
    /// Normal: 7-56 U/L
    pub alt_u_l: f64,

    /// Aspartate aminotransferase (U/L) - liver enzyme
    /// Normal: 10-40 U/L
    pub ast_u_l: f64,

    /// Alkaline phosphatase (U/L) - liver/bone enzyme
    /// Normal: 44-147 U/L
    pub alp_u_l: f64,

    /// Total cholesterol (mg/dL)
    /// Desirable: <200 mg/dL
    pub cholesterol_total_mg_dl: f64,

    /// HDL cholesterol (mg/dL) - "good" cholesterol
    /// Desirable: >40 mg/dL (men), >50 mg/dL (women)
    pub hdl_cholesterol_mg_dl: f64,

    /// LDL cholesterol (mg/dL) - "bad" cholesterol
    /// Optimal: <100 mg/dL
    pub ldl_cholesterol_mg_dl: f64,

    /// Triglycerides (mg/dL)
    /// Normal: <150 mg/dL
    pub triglycerides_mg_dl: f64,

    /// Lactate/Lactic acid (mmol/L) - tissue oxygenation marker
    /// Normal: 0.5-2.2 mmol/L
    pub lactate_mmol_l: f64,

    /// Toxin levels (arbitrary units) - from original system
    pub toxin_level_au: f64,

    /// Angiotensin II concentration (arbitrary units) - from original system
    pub angiotensin_ii_au: f64,
}

impl Default for BloodChemistry {
    fn default() -> Self {
        Self {
            glucose_mg_dl: 90.0,
            bun_mg_dl: 12.0,
            creatinine_mg_dl: 0.9,
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            chloride_meq_l: 101.0,
            bicarbonate_meq_l: 24.0,
            calcium_mg_dl: 9.5,
            magnesium_mg_dl: 2.0,
            phosphate_mg_dl: 3.5,
            total_protein_g_dl: 7.0,
            albumin_g_dl: 4.0,
            bilirubin_total_mg_dl: 0.6,
            bilirubin_direct_mg_dl: 0.1,
            alt_u_l: 25.0,
            ast_u_l: 22.0,
            alp_u_l: 70.0,
            cholesterol_total_mg_dl: 180.0,
            hdl_cholesterol_mg_dl: 55.0,
            ldl_cholesterol_mg_dl: 100.0,
            triglycerides_mg_dl: 100.0,
            lactate_mmol_l: 1.0,
            toxin_level_au: 0.0,
            angiotensin_ii_au: 0.0,
        }
    }
}

/// Coagulation factors and clotting parameters
#[derive(Debug, Clone)]
pub struct ClottingFactors {
    /// Prothrombin time (seconds) - extrinsic pathway
    /// Normal: 11-13.5 seconds
    pub pt_seconds: f64,

    /// International normalized ratio - standardized PT
    /// Normal: 0.8-1.2
    pub inr: f64,

    /// Activated partial thromboplastin time (seconds) - intrinsic pathway
    /// Normal: 25-35 seconds
    pub aptt_seconds: f64,

    /// Fibrinogen (mg/dL) - clotting protein
    /// Normal: 200-400 mg/dL
    pub fibrinogen_mg_dl: f64,

    /// D-dimer (ng/mL) - clot breakdown product
    /// Normal: <500 ng/mL
    pub d_dimer_ng_ml: f64,

    /// Bleeding time (minutes)
    /// Normal: 2-7 minutes
    pub bleeding_time_min: f64,

    /// Clotting time (minutes)
    /// Normal: 5-15 minutes
    pub clotting_time_min: f64,
}

impl Default for ClottingFactors {
    fn default() -> Self {
        Self {
            pt_seconds: 12.0,
            inr: 1.0,
            aptt_seconds: 30.0,
            fibrinogen_mg_dl: 300.0,
            d_dimer_ng_ml: 250.0,
            bleeding_time_min: 4.0,
            clotting_time_min: 8.0,
        }
    }
}

/// Arterial blood gas (ABG) analysis
#[derive(Debug, Clone)]
pub struct BloodGases {
    /// Blood pH
    /// Normal: 7.35-7.45
    pub ph: f64,

    /// Partial pressure of oxygen (mmHg)
    /// Normal: 75-100 mmHg
    pub pao2_mmhg: f64,

    /// Partial pressure of carbon dioxide (mmHg)
    /// Normal: 35-45 mmHg
    pub paco2_mmhg: f64,

    /// Bicarbonate (mEq/L) - from chemistry panel
    /// Normal: 22-26 mEq/L
    pub hco3_meq_l: f64,

    /// Base excess (mEq/L) - metabolic component
    /// Normal: -2 to +2 mEq/L
    pub base_excess_meq_l: f64,

    /// Oxygen saturation (%)
    /// Normal: 95-100%
    pub sao2_percent: f64,
}

impl Default for BloodGases {
    fn default() -> Self {
        Self {
            ph: 7.4,
            pao2_mmhg: 95.0,
            paco2_mmhg: 40.0,
            hco3_meq_l: 24.0,
            base_excess_meq_l: 0.0,
            sao2_percent: 98.0,
        }
    }
}

impl BloodGases {
    /// Calculate anion gap for acid-base status
    /// Normal: 8-16 mEq/L
    pub fn calculate_anion_gap(&self, sodium: f64, chloride: f64) -> f64 {
        sodium - (chloride + self.hco3_meq_l)
    }

    /// Determine acid-base disorder
    pub fn get_acid_base_status(&self) -> &'static str {
        if self.ph < 7.35 {
            if self.paco2_mmhg > 45.0 {
                "Respiratory Acidosis"
            } else if self.hco3_meq_l < 22.0 {
                "Metabolic Acidosis"
            } else {
                "Mixed Acidosis"
            }
        } else if self.ph > 7.45 {
            if self.paco2_mmhg < 35.0 {
                "Respiratory Alkalosis"
            } else if self.hco3_meq_l > 26.0 {
                "Metabolic Alkalosis"
            } else {
                "Mixed Alkalosis"
            }
        } else {
            "Normal"
        }
    }
}

/// Comprehensive blood composition with all blood characteristics
#[derive(Debug, Clone)]
pub struct BloodComposition {
    /// Blood type (ABO and Rh)
    pub blood_type: BloodType,

    /// Complete blood count
    pub cells: BloodCells,

    /// Blood chemistry panel
    pub chemistry: BloodChemistry,

    /// Clotting factors
    pub clotting: ClottingFactors,

    /// Arterial blood gases
    pub gases: BloodGases,

    /// Blood pressure - systolic (mmHg)
    pub blood_pressure_systolic: f64,

    /// Blood pressure - diastolic (mmHg)
    pub blood_pressure_diastolic: f64,

    /// Coronary artery flows (mL/min) - cached from vascular system
    /// These are updated by update_patient() before organ updates
    pub coronary_lad_flow: f64,
    pub coronary_lcx_flow: f64,
    pub coronary_rca_flow: f64,
}

impl Default for BloodComposition {
    fn default() -> Self {
        Self {
            blood_type: BloodType::default(),
            cells: BloodCells::default(),
            chemistry: BloodChemistry::default(),
            clotting: ClottingFactors::default(),
            gases: BloodGases::default(),
            blood_pressure_systolic: 120.0,
            blood_pressure_diastolic: 80.0,
            coronary_lad_flow: 40.0,
            coronary_lcx_flow: 30.0,
            coronary_rca_flow: 35.0,
        }
    }
}

impl BloodComposition {
    /// Get mean arterial pressure (MAP)
    /// Formula: MAP = DBP + 1/3(SBP - DBP)
    pub fn get_mean_arterial_pressure(&self) -> f64 {
        self.blood_pressure_diastolic + (self.blood_pressure_systolic - self.blood_pressure_diastolic) / 3.0
    }

    /// Calculate oxygen delivery (DO2) in mL/min
    /// Formula: DO2 = CO × CaO2 × 10
    /// where CO = cardiac output (L/min), CaO2 = arterial oxygen content
    /// Simplified version using hemoglobin and SaO2
    pub fn calculate_oxygen_content(&self) -> f64 {
        // CaO2 = (Hb × 1.34 × SaO2) + (0.003 × PaO2)
        (self.cells.hemoglobin_g_dl * 1.34 * self.gases.sao2_percent / 100.0) +
        (0.003 * self.gases.pao2_mmhg)
    }

    /// Calculate estimated glomerular filtration rate (eGFR) using simplified formula
    /// Requires age and sex for full calculation - this is a simplified version
    pub fn calculate_egfr_simplified(&self) -> f64 {
        // Simplified eGFR = 186 × (Creatinine)^-1.154
        // Full formula requires age and sex
        186.0 * self.chemistry.creatinine_mg_dl.powf(-1.154)
    }

    /// Get comprehensive blood summary string
    pub fn get_summary(&self) -> String {
        format!(
            "Blood Type: {} | RBC: {:.2}M/µL | Hgb: {:.1} g/dL | WBC: {:.0}/µL | Plt: {:.0}K/µL\n\
             pH: {:.2} | PaO2: {:.0} mmHg | PaCO2: {:.0} mmHg | SaO2: {:.1}%\n\
             Na: {:.1} | K: {:.1} | Glucose: {:.0} mg/dL | Creatinine: {:.2} mg/dL\n\
             BP: {}/{} mmHg | MAP: {:.0} mmHg",
            self.blood_type,
            self.cells.rbc_count_million_per_ul,
            self.cells.hemoglobin_g_dl,
            self.cells.wbc_differential.total_count(),
            self.cells.platelet_count_thousand_per_ul,
            self.gases.ph,
            self.gases.pao2_mmhg,
            self.gases.paco2_mmhg,
            self.gases.sao2_percent,
            self.chemistry.sodium_meq_l,
            self.chemistry.potassium_meq_l,
            self.chemistry.glucose_mg_dl,
            self.chemistry.creatinine_mg_dl,
            self.blood_pressure_systolic,
            self.blood_pressure_diastolic,
            self.get_mean_arterial_pressure()
        )
    }

    /// Get complete blood count (CBC) summary
    pub fn get_cbc_summary(&self) -> String {
        format!(
            "=== Complete Blood Count (CBC) ===\n\
             RBC: {:.2} M/µL | Hemoglobin: {:.1} g/dL | Hematocrit: {:.1}%\n\
             MCV: {:.1} fL | MCH: {:.1} pg | MCHC: {:.1} g/dL | RDW: {:.1}%\n\
             WBC: {:.0}/µL (Neut: {:.0}, Lymph: {:.0}, Mono: {:.0}, Eos: {:.0}, Baso: {:.0})\n\
             Platelets: {:.0}K/µL | MPV: {:.1} fL",
            self.cells.rbc_count_million_per_ul,
            self.cells.hemoglobin_g_dl,
            self.cells.hematocrit_percent,
            self.cells.mcv_fl,
            self.cells.mch_pg,
            self.cells.mchc_g_dl,
            self.cells.rdw_percent,
            self.cells.wbc_differential.total_count(),
            self.cells.wbc_differential.neutrophils,
            self.cells.wbc_differential.lymphocytes,
            self.cells.wbc_differential.monocytes,
            self.cells.wbc_differential.eosinophils,
            self.cells.wbc_differential.basophils,
            self.cells.platelet_count_thousand_per_ul,
            self.cells.mpv_fl
        )
    }

    /// Get comprehensive metabolic panel (CMP) summary
    pub fn get_cmp_summary(&self) -> String {
        format!(
            "=== Comprehensive Metabolic Panel (CMP) ===\n\
             Glucose: {:.0} mg/dL | BUN: {:.1} mg/dL | Creatinine: {:.2} mg/dL\n\
             Na: {:.1} mEq/L | K: {:.2} mEq/L | Cl: {:.1} mEq/L | HCO3: {:.1} mEq/L\n\
             Ca: {:.1} mg/dL | Mg: {:.1} mg/dL | Phos: {:.1} mg/dL\n\
             Total Protein: {:.1} g/dL | Albumin: {:.1} g/dL\n\
             Bilirubin (T/D): {:.1}/{:.1} mg/dL\n\
             ALT: {:.0} U/L | AST: {:.0} U/L | ALP: {:.0} U/L",
            self.chemistry.glucose_mg_dl,
            self.chemistry.bun_mg_dl,
            self.chemistry.creatinine_mg_dl,
            self.chemistry.sodium_meq_l,
            self.chemistry.potassium_meq_l,
            self.chemistry.chloride_meq_l,
            self.chemistry.bicarbonate_meq_l,
            self.chemistry.calcium_mg_dl,
            self.chemistry.magnesium_mg_dl,
            self.chemistry.phosphate_mg_dl,
            self.chemistry.total_protein_g_dl,
            self.chemistry.albumin_g_dl,
            self.chemistry.bilirubin_total_mg_dl,
            self.chemistry.bilirubin_direct_mg_dl,
            self.chemistry.alt_u_l,
            self.chemistry.ast_u_l,
            self.chemistry.alp_u_l
        )
    }

    /// Get arterial blood gas (ABG) summary
    pub fn get_abg_summary(&self) -> String {
        let anion_gap = self.gases.calculate_anion_gap(
            self.chemistry.sodium_meq_l,
            self.chemistry.chloride_meq_l
        );
        format!(
            "=== Arterial Blood Gas (ABG) ===\n\
             pH: {:.2} | PaO2: {:.0} mmHg | PaCO2: {:.0} mmHg\n\
             HCO3: {:.1} mEq/L | Base Excess: {:.1} mEq/L\n\
             SaO2: {:.1}% | Anion Gap: {:.1} mEq/L\n\
             Status: {}",
            self.gases.ph,
            self.gases.pao2_mmhg,
            self.gases.paco2_mmhg,
            self.gases.hco3_meq_l,
            self.gases.base_excess_meq_l,
            self.gases.sao2_percent,
            anion_gap,
            self.gases.get_acid_base_status()
        )
    }

    /// Get coagulation panel summary
    pub fn get_coag_summary(&self) -> String {
        format!(
            "=== Coagulation Panel ===\n\
             PT: {:.1} sec | INR: {:.2} | aPTT: {:.1} sec\n\
             Fibrinogen: {:.0} mg/dL | D-Dimer: {:.0} ng/mL\n\
             Bleeding Time: {:.1} min | Clotting Time: {:.1} min",
            self.clotting.pt_seconds,
            self.clotting.inr,
            self.clotting.aptt_seconds,
            self.clotting.fibrinogen_mg_dl,
            self.clotting.d_dimer_ng_ml,
            self.clotting.bleeding_time_min,
            self.clotting.clotting_time_min
        )
    }
}
