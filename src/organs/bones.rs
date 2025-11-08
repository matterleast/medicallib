use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use std::any::Any;

/// Represents a single bone in the skeletal system
#[derive(Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub density: f64,           // g/cm³ (normal: 1.0-1.2)
    pub length_cm: f64,
    pub fracture_severity: f64, // 0.0 = healthy, 1.0 = complete fracture
    pub healing_progress: f64,  // 0.0 to 1.0
}

impl Bone {
    pub fn new(name: &str, length_cm: f64) -> Self {
        Self {
            name: name.to_string(),
            density: 1.1,
            length_cm,
            fracture_severity: 0.0,
            healing_progress: 0.0,
        }
    }

    /// Inflict a fracture on this bone
    pub fn fracture(&mut self, severity: f64) {
        self.fracture_severity = severity.clamp(0.0, 1.0);
        self.healing_progress = 0.0;
    }

    /// Check if bone is fractured
    pub fn is_fractured(&self) -> bool {
        self.fracture_severity > 0.1
    }
}

/// Bone marrow - produces blood cells
#[derive(Debug, Clone)]
pub struct BoneMarrow {
    pub red_marrow_volume_ml: f64,      // Active hematopoietic tissue
    pub yellow_marrow_volume_ml: f64,   // Fatty marrow (can convert to red)
    pub production_efficiency: f64,      // 0.0-1.0
}

impl BoneMarrow {
    pub fn new() -> Self {
        Self {
            red_marrow_volume_ml: 2600.0,    // Average adult
            yellow_marrow_volume_ml: 1400.0,
            production_efficiency: 1.0,
        }
    }

    /// Calculate blood cell production rate
    pub fn get_rbc_production_rate(&self) -> f64 {
        // Millions of RBCs per second (normal ~2-3 million/sec)
        self.red_marrow_volume_ml * 0.001 * self.production_efficiency
    }

    pub fn get_wbc_production_rate(&self) -> f64 {
        // Thousands of WBCs per second
        self.red_marrow_volume_ml * 0.0003 * self.production_efficiency
    }

    pub fn get_platelet_production_rate(&self) -> f64 {
        // Thousands of platelets per second
        self.red_marrow_volume_ml * 0.03 * self.production_efficiency
    }
}

/// The skeletal system - bones, calcium homeostasis, blood cell production
#[derive(Debug)]
pub struct Bones {
    id: OrganId,
    pub bones: Vec<Bone>,
    pub bone_marrow: BoneMarrow,
    pub total_calcium_stores_g: f64,     // Total calcium in bones (normal: ~1000g)
    pub bone_turnover_rate: f64,         // Fraction of bone replaced per day
    pub osteoblast_activity: f64,        // 0.0-1.0 (bone building)
    pub osteoclast_activity: f64,        // 0.0-1.0 (bone resorption)
    pub vitamin_d_receptors: f64,        // Sensitivity to vitamin D
}

impl Bones {
    pub fn new(id: i32) -> Self {
        let mut bones = Vec::new();

        // Major bones of the skeleton
        bones.push(Bone::new("Skull", 18.0));
        bones.push(Bone::new("Cervical Vertebrae", 10.0));
        bones.push(Bone::new("Thoracic Vertebrae", 20.0));
        bones.push(Bone::new("Lumbar Vertebrae", 18.0));
        bones.push(Bone::new("Sacrum", 10.0));
        bones.push(Bone::new("Ribs", 60.0));
        bones.push(Bone::new("Sternum", 15.0));
        bones.push(Bone::new("Clavicle", 15.0));
        bones.push(Bone::new("Scapula", 16.0));
        bones.push(Bone::new("Humerus", 30.0));
        bones.push(Bone::new("Radius", 25.0));
        bones.push(Bone::new("Ulna", 25.0));
        bones.push(Bone::new("Pelvis", 20.0));
        bones.push(Bone::new("Femur", 45.0));
        bones.push(Bone::new("Tibia", 38.0));
        bones.push(Bone::new("Fibula", 36.0));

        Self {
            id: id as usize,
            bones,
            bone_marrow: BoneMarrow::new(),
            total_calcium_stores_g: 1000.0,
            bone_turnover_rate: 0.10,        // 10% per year
            osteoblast_activity: 0.5,
            osteoclast_activity: 0.5,
            vitamin_d_receptors: 1.0,
        }
    }

    /// Calculate average bone density across all bones
    pub fn average_density(&self) -> f64 {
        if self.bones.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.bones.iter().map(|b| b.density).sum();
        sum / self.bones.len() as f64
    }

    /// Get number of fractured bones
    pub fn fractured_bone_count(&self) -> usize {
        self.bones.iter().filter(|b| b.is_fractured()).count()
    }

    /// Inflict fracture on a random bone
    pub fn inflict_fracture(&mut self, bone_index: usize, severity: f64) {
        if bone_index < self.bones.len() {
            self.bones[bone_index].fracture(severity);
        }
    }

    /// Calculate structural integrity (0.0-1.0)
    pub fn structural_integrity(&self) -> f64 {
        let density_factor = self.average_density() / 1.1;
        let fracture_penalty = self.fractured_bone_count() as f64 * 0.05;
        (density_factor - fracture_penalty).clamp(0.0, 1.0)
    }
}

impl Organ for Bones {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // 1. Calcium homeostasis - maintain blood calcium levels
        let blood_calcium = patient.blood.chemistry.calcium_mg_dl;
        let target_calcium = 9.5; // mg/dL

        // If blood calcium is low, resorb bone to release calcium
        if blood_calcium < target_calcium {
            let deficit = target_calcium - blood_calcium;
            let release_amount = deficit * 0.01 * delta_time_s; // mg/dL

            // Increase osteoclast activity to break down bone
            self.osteoclast_activity = (self.osteoclast_activity + 0.1 * delta_time_s).min(1.0);

            // Release calcium from bone stores
            let calcium_released_g = release_amount * 0.001;
            if self.total_calcium_stores_g > calcium_released_g {
                self.total_calcium_stores_g -= calcium_released_g;
                patient.blood.chemistry.calcium_mg_dl += release_amount;
            }
        }
        // If blood calcium is high, deposit into bone
        else if blood_calcium > target_calcium {
            let excess = blood_calcium - target_calcium;
            let deposit_amount = excess * 0.005 * delta_time_s; // mg/dL

            // Increase osteoblast activity to build bone
            self.osteoblast_activity = (self.osteoblast_activity + 0.1 * delta_time_s).min(1.0);

            // Deposit calcium into bone stores
            let calcium_deposited_g = deposit_amount * 0.001;
            self.total_calcium_stores_g += calcium_deposited_g;
            patient.blood.chemistry.calcium_mg_dl -= deposit_amount;
        }

        // 2. Update bone density based on osteoblast/osteoclast balance
        let net_bone_formation = self.osteoblast_activity - self.osteoclast_activity;
        let density_change = net_bone_formation * 0.001 * delta_time_s;

        for bone in &mut self.bones {
            bone.density = (bone.density + density_change).clamp(0.3, 1.3);
        }

        // 3. Bone marrow blood cell production
        // RBC production (erythropoiesis)
        let rbc_production = self.bone_marrow.get_rbc_production_rate() * delta_time_s;
        patient.blood.cells.rbc_count_million_per_ul += rbc_production * 0.0001;

        // Maintain hemoglobin with RBC production
        let target_hgb = patient.blood.cells.rbc_count_million_per_ul * 3.0;
        patient.blood.cells.hemoglobin_g_dl =
            patient.blood.cells.hemoglobin_g_dl * 0.99 + target_hgb * 0.01;

        // WBC production (leukopoiesis) - increase neutrophils as primary WBC
        let wbc_production = self.bone_marrow.get_wbc_production_rate() * delta_time_s;
        patient.blood.cells.wbc_differential.neutrophils += wbc_production * 0.1;

        // Platelet production (thrombopoiesis)
        let platelet_production = self.bone_marrow.get_platelet_production_rate() * delta_time_s;
        patient.blood.cells.platelet_count_thousand_per_ul += platelet_production * 0.001;

        // 4. Fracture healing - requires adequate blood supply and nutrients
        let healing_factor =
            (blood_calcium / 9.5) *
            (patient.blood.cells.rbc_count_million_per_ul / 5.0) *
            (patient.blood.chemistry.total_protein_g_dl / 7.0);

        for bone in &mut self.bones {
            if bone.is_fractured() {
                // Progress healing
                bone.healing_progress += healing_factor * 0.01 * delta_time_s;
                bone.healing_progress = bone.healing_progress.min(1.0);

                // Reduce fracture severity as healing progresses
                if bone.healing_progress > 0.5 {
                    bone.fracture_severity *= 1.0 - (0.01 * delta_time_s);
                    if bone.fracture_severity < 0.1 {
                        bone.fracture_severity = 0.0; // Fully healed
                    }
                }
            }
        }

        // 5. Bone marrow efficiency depends on oxygen and nutrients
        let o2_saturation = patient.blood.gases.sao2_percent / 100.0;
        let glucose_factor = (patient.blood.chemistry.glucose_mg_dl / 90.0).clamp(0.5, 1.5);
        self.bone_marrow.production_efficiency =
            (o2_saturation * glucose_factor * 0.3 + self.bone_marrow.production_efficiency * 0.7)
            .clamp(0.1, 1.0);

        // 6. Phosphate homeostasis (works with calcium)
        let target_phosphate = 3.5; // mg/dL
        let phosphate_diff = target_phosphate - patient.blood.chemistry.phosphate_mg_dl;
        patient.blood.chemistry.phosphate_mg_dl += phosphate_diff * 0.01 * delta_time_s;
    }

    fn get_summary(&self) -> String {
        format!(
            "Bones - Density: {:.2} g/cm³, Calcium stores: {:.1}g, Fractured bones: {}, \
             Marrow efficiency: {:.1}%, Structural integrity: {:.1}%, \
             Osteoblast/Osteoclast: {:.2}/{:.2}",
            self.average_density(),
            self.total_calcium_stores_g,
            self.fractured_bone_count(),
            self.bone_marrow.production_efficiency * 100.0,
            self.structural_integrity() * 100.0,
            self.osteoblast_activity,
            self.osteoclast_activity
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Bones"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for BoneMarrow {
    fn default() -> Self {
        Self::new()
    }
}
