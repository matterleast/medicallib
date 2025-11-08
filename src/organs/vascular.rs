use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use std::any::Any;

/// Type of blood vessel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VesselType {
    Artery,
    Arteriole,
    Capillary,
    Venule,
    Vein,
}

impl VesselType {
    /// Get typical wall thickness as proportion of diameter
    pub fn wall_thickness_ratio(&self) -> f64 {
        match self {
            VesselType::Artery => 0.25,      // Thick muscular walls
            VesselType::Arteriole => 0.20,
            VesselType::Capillary => 0.02,   // Very thin walls for exchange
            VesselType::Venule => 0.10,
            VesselType::Vein => 0.15,        // Thinner than arteries
        }
    }

    /// Get typical pressure in mmHg
    pub fn typical_pressure(&self) -> f64 {
        match self {
            VesselType::Artery => 100.0,     // Mean arterial pressure
            VesselType::Arteriole => 60.0,
            VesselType::Capillary => 25.0,
            VesselType::Venule => 15.0,
            VesselType::Vein => 5.0,         // Low pressure system
        }
    }
}

/// A single blood vessel
#[derive(Debug, Clone)]
pub struct Vessel {
    pub name: String,
    pub vessel_type: VesselType,
    pub diameter_mm: f64,
    pub length_cm: f64,
    pub baseline_diameter_mm: f64,   // For calculating vasodilation/constriction
    pub elasticity: f64,             // 0.0-1.0 (compliance)
    pub plaque_buildup: f64,         // 0.0-1.0 (atherosclerosis)
    pub smooth_muscle_tone: f64,     // 0.0-1.0 (vasoconstriction)
    pub endothelial_health: f64,     // 0.0-1.0 (lining of vessel)
    pub inflammation: f64,           // 0.0-1.0
}

impl Vessel {
    pub fn new(name: &str, vessel_type: VesselType, diameter_mm: f64, length_cm: f64) -> Self {
        Self {
            name: name.to_string(),
            vessel_type,
            diameter_mm,
            length_cm,
            baseline_diameter_mm: diameter_mm,
            elasticity: 0.8,
            plaque_buildup: 0.0,
            smooth_muscle_tone: 0.5,
            endothelial_health: 1.0,
            inflammation: 0.0,
        }
    }

    /// Calculate effective diameter considering plaque buildup
    pub fn effective_diameter(&self) -> f64 {
        self.diameter_mm * (1.0 - self.plaque_buildup * 0.8)
    }

    /// Calculate resistance to blood flow (Poiseuille's law simplified)
    /// Resistance is proportional to length and inversely proportional to radius^4
    pub fn flow_resistance(&self) -> f64 {
        let radius_mm = self.effective_diameter() / 2.0;
        if radius_mm <= 0.0 {
            return f64::MAX;
        }
        // Simplified resistance calculation
        self.length_cm / (radius_mm.powi(4))
    }

    /// Apply vasoconstriction (decrease diameter)
    pub fn constrict(&mut self, amount: f64) {
        self.smooth_muscle_tone = (self.smooth_muscle_tone + amount).min(1.0);
        self.diameter_mm = self.baseline_diameter_mm * (1.0 - self.smooth_muscle_tone * 0.5);
    }

    /// Apply vasodilation (increase diameter)
    pub fn dilate(&mut self, amount: f64) {
        self.smooth_muscle_tone = (self.smooth_muscle_tone - amount).max(0.0);
        self.diameter_mm = self.baseline_diameter_mm * (1.0 - self.smooth_muscle_tone * 0.5);
    }

    /// Check if vessel is critically stenosed (>70% blocked)
    pub fn is_critically_stenosed(&self) -> bool {
        self.plaque_buildup > 0.7
    }
}

/// Vascular system - arteries, veins, and capillaries
#[derive(Debug)]
pub struct VascularSystem {
    id: OrganId,
    pub vessels: Vec<Vessel>,
    pub total_blood_volume_l: f64,       // Liters (normal: ~5L)
    pub arterial_compliance: f64,        // Overall arterial elasticity
    pub venous_compliance: f64,          // Overall venous elasticity
    pub total_peripheral_resistance: f64, // Overall resistance to flow
    pub capillary_permeability: f64,     // 0.0-1.0
    pub nitric_oxide_level: f64,         // Vasodilator (normal: 1.0)
    pub endothelin_level: f64,           // Vasoconstrictor (normal: 1.0)
    pub atherosclerosis_progression: f64, // Rate of plaque formation
}

impl VascularSystem {
    pub fn new(id: i32) -> Self {
        let mut vessels = Vec::new();

        // Major arteries
        vessels.push(Vessel::new("Aorta", VesselType::Artery, 25.0, 40.0));
        vessels.push(Vessel::new("Carotid Artery (L)", VesselType::Artery, 8.0, 20.0));
        vessels.push(Vessel::new("Carotid Artery (R)", VesselType::Artery, 8.0, 20.0));
        vessels.push(Vessel::new("Subclavian Artery (L)", VesselType::Artery, 9.0, 15.0));
        vessels.push(Vessel::new("Subclavian Artery (R)", VesselType::Artery, 9.0, 15.0));
        vessels.push(Vessel::new("Brachial Artery (L)", VesselType::Artery, 5.0, 30.0));
        vessels.push(Vessel::new("Brachial Artery (R)", VesselType::Artery, 5.0, 30.0));
        vessels.push(Vessel::new("Radial Artery (L)", VesselType::Artery, 3.0, 25.0));
        vessels.push(Vessel::new("Radial Artery (R)", VesselType::Artery, 3.0, 25.0));
        vessels.push(Vessel::new("Celiac Artery", VesselType::Artery, 7.0, 10.0));
        vessels.push(Vessel::new("Renal Artery (L)", VesselType::Artery, 5.0, 8.0));
        vessels.push(Vessel::new("Renal Artery (R)", VesselType::Artery, 5.0, 8.0));
        vessels.push(Vessel::new("Iliac Artery (L)", VesselType::Artery, 10.0, 15.0));
        vessels.push(Vessel::new("Iliac Artery (R)", VesselType::Artery, 10.0, 15.0));
        vessels.push(Vessel::new("Femoral Artery (L)", VesselType::Artery, 8.0, 40.0));
        vessels.push(Vessel::new("Femoral Artery (R)", VesselType::Artery, 8.0, 40.0));

        // Major veins
        vessels.push(Vessel::new("Superior Vena Cava", VesselType::Vein, 20.0, 15.0));
        vessels.push(Vessel::new("Inferior Vena Cava", VesselType::Vein, 22.0, 35.0));
        vessels.push(Vessel::new("Jugular Vein (L)", VesselType::Vein, 10.0, 20.0));
        vessels.push(Vessel::new("Jugular Vein (R)", VesselType::Vein, 10.0, 20.0));
        vessels.push(Vessel::new("Subclavian Vein (L)", VesselType::Vein, 12.0, 15.0));
        vessels.push(Vessel::new("Subclavian Vein (R)", VesselType::Vein, 12.0, 15.0));
        vessels.push(Vessel::new("Femoral Vein (L)", VesselType::Vein, 10.0, 40.0));
        vessels.push(Vessel::new("Femoral Vein (R)", VesselType::Vein, 10.0, 40.0));
        vessels.push(Vessel::new("Renal Vein (L)", VesselType::Vein, 6.0, 8.0));
        vessels.push(Vessel::new("Renal Vein (R)", VesselType::Vein, 6.0, 8.0));

        // Arterioles and capillaries (representing thousands as aggregates)
        vessels.push(Vessel::new("Systemic Arterioles", VesselType::Arteriole, 0.5, 1000.0));
        vessels.push(Vessel::new("Systemic Capillaries", VesselType::Capillary, 0.008, 10000.0));
        vessels.push(Vessel::new("Systemic Venules", VesselType::Venule, 0.5, 1000.0));

        Self {
            id: id as usize,
            vessels,
            total_blood_volume_l: 5.0,
            arterial_compliance: 0.8,
            venous_compliance: 0.9,
            total_peripheral_resistance: 1.0,
            capillary_permeability: 0.5,
            nitric_oxide_level: 1.0,
            endothelin_level: 1.0,
            atherosclerosis_progression: 0.0,
        }
    }

    /// Calculate total vascular resistance
    pub fn calculate_total_resistance(&self) -> f64 {
        // Sum resistances (simplified)
        let resistance_sum: f64 = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Artery | VesselType::Arteriole))
            .map(|v| v.flow_resistance())
            .sum();

        // Normalize to a reasonable range
        (resistance_sum / 1000.0).clamp(0.5, 3.0)
    }

    /// Get number of critically stenosed vessels
    pub fn critically_stenosed_count(&self) -> usize {
        self.vessels.iter().filter(|v| v.is_critically_stenosed()).count()
    }

    /// Calculate average vessel health
    pub fn average_vessel_health(&self) -> f64 {
        if self.vessels.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.vessels.iter().map(|v| v.endothelial_health).sum();
        sum / self.vessels.len() as f64
    }

    /// Calculate average plaque burden
    pub fn average_plaque_burden(&self) -> f64 {
        if self.vessels.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.vessels.iter().map(|v| v.plaque_buildup).sum();
        sum / self.vessels.len() as f64
    }
}

impl Organ for VascularSystem {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // 1. Calculate total peripheral resistance
        self.total_peripheral_resistance = self.calculate_total_resistance();

        // 2. Update blood pressure based on vascular resistance
        // Mean arterial pressure = cardiac output × total peripheral resistance
        // Simplified: BP increases with resistance
        let resistance_effect = (self.total_peripheral_resistance - 1.0) * 20.0;

        // Also affected by blood volume
        let volume_effect = (self.total_blood_volume_l - 5.0) * 5.0;

        // Apply to blood pressure (gently nudge toward new values)
        patient.blood.blood_pressure_systolic += (resistance_effect + volume_effect) * 0.01;
        patient.blood.blood_pressure_diastolic += (resistance_effect + volume_effect) * 0.007;

        // 3. Nitric oxide production (vasodilator)
        // Produced by healthy endothelium, requires oxygen
        let o2_sat = patient.blood.gases.sao2_percent / 100.0;
        let endothelial_no_production = self.average_vessel_health() * o2_sat;
        self.nitric_oxide_level = (self.nitric_oxide_level * 0.95 + endothelial_no_production * 0.05)
            .clamp(0.2, 2.0);

        // 4. Endothelin production (vasoconstrictor)
        // Increased by inflammation and low oxygen
        let inflammation_avg: f64 = self.vessels.iter().map(|v| v.inflammation).sum::<f64>()
            / self.vessels.len() as f64;
        let endothelin_production = 1.0 + inflammation_avg * 0.5 + (1.0 - o2_sat) * 0.5;
        self.endothelin_level = (self.endothelin_level * 0.96 + endothelin_production * 0.04)
            .clamp(0.5, 2.5);

        // 5. Apply vasoconstriction/dilation based on various factors
        let vasodilation_signal = self.nitric_oxide_level - 1.0;
        let vasoconstriction_signal = self.endothelin_level - 1.0;

        // Angiotensin II from blood (vasoconstrictor)
        let angiotensin_signal = (patient.blood.chemistry.angiotensin_ii_au - 1.0) * 0.5;

        let net_tone_change = (vasoconstriction_signal + angiotensin_signal - vasodilation_signal) * 0.01 * delta_time_s;

        for vessel in &mut self.vessels {
            if matches!(vessel.vessel_type, VesselType::Artery | VesselType::Arteriole) {
                if net_tone_change > 0.0 {
                    vessel.constrict(net_tone_change);
                } else {
                    vessel.dilate(-net_tone_change);
                }
            }
        }

        // 6. Atherosclerosis progression
        // Risk factors: high LDL, low HDL, inflammation, high glucose
        let ldl_risk = (patient.blood.chemistry.ldl_cholesterol_mg_dl - 100.0).max(0.0) / 100.0;
        let hdl_protection = (60.0 - patient.blood.chemistry.hdl_cholesterol_mg_dl).max(0.0) / 60.0;
        let glucose_risk = (patient.blood.chemistry.glucose_mg_dl - 100.0).max(0.0) / 100.0;

        self.atherosclerosis_progression = (ldl_risk + hdl_protection + glucose_risk) / 3.0;

        // Apply plaque buildup (very slow process)
        let plaque_growth = self.atherosclerosis_progression * 0.00001 * delta_time_s;

        for vessel in &mut self.vessels {
            if matches!(vessel.vessel_type, VesselType::Artery) {
                vessel.plaque_buildup = (vessel.plaque_buildup + plaque_growth).min(0.95);

                // Plaque causes inflammation
                if vessel.plaque_buildup > 0.3 {
                    vessel.inflammation = (vessel.inflammation + 0.001 * delta_time_s).min(1.0);
                }

                // Inflammation damages endothelium
                if vessel.inflammation > 0.3 {
                    vessel.endothelial_health -= 0.0001 * delta_time_s;
                    vessel.endothelial_health = vessel.endothelial_health.max(0.1);
                }
            }
        }

        // 7. Update arterial and venous compliance
        let artery_health: f64 = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Artery))
            .map(|v| v.elasticity * v.endothelial_health)
            .sum::<f64>();
        let artery_count = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Artery))
            .count() as f64;

        self.arterial_compliance = if artery_count > 0.0 {
            artery_health / artery_count
        } else {
            0.5
        };

        // 8. Capillary permeability - affected by inflammation
        // Normal permeability allows nutrient/gas exchange
        // Too much causes edema
        self.capillary_permeability = (0.5 + inflammation_avg * 0.3).clamp(0.3, 0.9);

        // 9. Blood volume regulation
        // Kidneys regulate this, but we can track shifts
        // High sodium increases blood volume
        let sodium_effect = (patient.blood.chemistry.sodium_meq_l - 140.0) / 140.0;
        self.total_blood_volume_l += sodium_effect * 0.001 * delta_time_s;
        self.total_blood_volume_l = self.total_blood_volume_l.clamp(3.0, 7.0);

        // 10. Vessel elasticity decreases with age and damage
        // Toxins, high glucose, and oxidative stress reduce elasticity
        let toxin_damage = patient.blood.chemistry.toxin_level_au * 0.00001 * delta_time_s;
        let glucose_damage = if patient.blood.chemistry.glucose_mg_dl > 180.0 {
            0.00001 * delta_time_s
        } else {
            0.0
        };

        for vessel in &mut self.vessels {
            vessel.elasticity -= toxin_damage + glucose_damage;
            vessel.elasticity = vessel.elasticity.max(0.2);
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Vascular - Resistance: {:.2}, Vessel health: {:.1}%, Plaque burden: {:.1}%, \
             Critical stenoses: {}, Arterial compliance: {:.2}, NO: {:.2}, ET-1: {:.2}, \
             Blood volume: {:.2}L",
            self.total_peripheral_resistance,
            self.average_vessel_health() * 100.0,
            self.average_plaque_burden() * 100.0,
            self.critically_stenosed_count(),
            self.arterial_compliance,
            self.nitric_oxide_level,
            self.endothelin_level,
            self.total_blood_volume_l
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "VascularSystem"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
