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
    pub blood_volume_ml: f64,        // Volume of blood in this vessel (mL)
    pub blood_flow_rate_ml_per_min: f64, // Flow rate through vessel (mL/min)
    pub pressure_mmhg: f64,          // Blood pressure in this vessel (mmHg)
    pub blood_velocity_cm_per_s: f64, // Velocity of blood flow (cm/s)
}

impl Vessel {
    pub fn new(name: &str, vessel_type: VesselType, diameter_mm: f64, length_cm: f64) -> Self {
        let mut vessel = Self {
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
            blood_volume_ml: 0.0,
            blood_flow_rate_ml_per_min: 0.0,
            pressure_mmhg: 0.0,
            blood_velocity_cm_per_s: 0.0,
        };
        vessel.calculate_volume();
        vessel.pressure_mmhg = vessel.vessel_type.typical_pressure();
        vessel
    }

    /// Calculate the volume of blood this vessel can hold based on dimensions
    /// Volume = π * r² * length
    pub fn calculate_volume(&mut self) {
        let radius_mm = self.effective_diameter() / 2.0;
        let radius_cm = radius_mm / 10.0;
        let length_cm = self.length_cm;
        // Volume in cm³ = mL
        self.blood_volume_ml = std::f64::consts::PI * radius_cm * radius_cm * length_cm;
    }

    /// Calculate blood flow rate using simplified Poiseuille's law
    /// Flow = ΔP / Resistance
    pub fn calculate_flow_rate(&mut self, upstream_pressure: f64, downstream_pressure: f64) {
        let delta_p = upstream_pressure - downstream_pressure;
        let resistance = self.flow_resistance();

        if resistance > 0.0 && delta_p > 0.0 {
            // Simplified flow calculation (actual Poiseuille uses viscosity constant)
            // Using a scaling factor to get realistic flow rates
            self.blood_flow_rate_ml_per_min = (delta_p / resistance) * 100.0;
        } else {
            self.blood_flow_rate_ml_per_min = 0.0;
        }
    }

    /// Calculate blood velocity from flow rate
    /// Velocity = Flow / Cross-sectional area
    pub fn calculate_velocity(&mut self) {
        let radius_mm = self.effective_diameter() / 2.0;
        let radius_cm = radius_mm / 10.0;
        let cross_sectional_area_cm2 = std::f64::consts::PI * radius_cm * radius_cm;

        if cross_sectional_area_cm2 > 0.0 {
            // Convert mL/min to cm³/s, then divide by area to get cm/s
            let flow_cm3_per_s = self.blood_flow_rate_ml_per_min / 60.0;
            self.blood_velocity_cm_per_s = flow_cm3_per_s / cross_sectional_area_cm2;
        } else {
            self.blood_velocity_cm_per_s = 0.0;
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
        self.calculate_volume(); // Recalculate volume with new diameter
    }

    /// Apply vasodilation (increase diameter)
    pub fn dilate(&mut self, amount: f64) {
        self.smooth_muscle_tone = (self.smooth_muscle_tone - amount).max(0.0);
        self.diameter_mm = self.baseline_diameter_mm * (1.0 - self.smooth_muscle_tone * 0.5);
        self.calculate_volume(); // Recalculate volume with new diameter
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
    pub arterial_blood_volume_ml: f64,   // Blood in arteries (mL)
    pub venous_blood_volume_ml: f64,     // Blood in veins (mL)
    pub capillary_blood_volume_ml: f64,  // Blood in capillaries (mL)
    pub arterial_compliance: f64,        // Overall arterial elasticity
    pub venous_compliance: f64,          // Overall venous elasticity
    pub total_peripheral_resistance: f64, // Overall resistance to flow
    pub capillary_permeability: f64,     // 0.0-1.0
    pub nitric_oxide_level: f64,         // Vasodilator (normal: 1.0)
    pub endothelin_level: f64,           // Vasoconstrictor (normal: 1.0)
    pub atherosclerosis_progression: f64, // Rate of plaque formation
    pub cardiac_output_l_per_min: f64,   // Cardiac output (L/min, normal: ~5L/min)
    pub venous_return_l_per_min: f64,    // Blood returning to heart (L/min)
    pub mean_arterial_pressure: f64,     // MAP (mmHg)
    pub central_venous_pressure: f64,    // CVP (mmHg, normal: 2-8)
}

impl VascularSystem {
    pub fn new(id: i32) -> Self {
        let mut vessels = Vec::new();

        // Major arteries
        vessels.push(Vessel::new("Aorta", VesselType::Artery, 25.0, 40.0));

        // Coronary arteries - CRITICAL for myocardial perfusion
        vessels.push(Vessel::new("Left Main Coronary", VesselType::Artery, 4.5, 1.0));
        vessels.push(Vessel::new("LAD", VesselType::Artery, 3.5, 12.0));  // Left anterior descending
        vessels.push(Vessel::new("LCx", VesselType::Artery, 3.0, 8.0));   // Left circumflex
        vessels.push(Vessel::new("RCA", VesselType::Artery, 3.5, 15.0));  // Right coronary artery

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
        // Note: diameter/length are aggregates representing the entire capillary network
        vessels.push(Vessel::new("Systemic Arterioles", VesselType::Arteriole, 0.5, 1000.0));
        vessels.push(Vessel::new("Systemic Capillaries", VesselType::Capillary, 1.6, 10000.0));
        vessels.push(Vessel::new("Systemic Venules", VesselType::Venule, 0.5, 1000.0));

        let mut system = Self {
            id: id as usize,
            vessels,
            total_blood_volume_l: 5.0,
            arterial_blood_volume_ml: 0.0,
            venous_blood_volume_ml: 0.0,
            capillary_blood_volume_ml: 0.0,
            arterial_compliance: 0.8,
            venous_compliance: 0.9,
            total_peripheral_resistance: 1.0,
            capillary_permeability: 0.5,
            nitric_oxide_level: 1.0,
            endothelin_level: 1.0,
            atherosclerosis_progression: 0.0,
            cardiac_output_l_per_min: 5.0,
            venous_return_l_per_min: 5.0,
            mean_arterial_pressure: 93.0,  // (120 + 2*80) / 3
            central_venous_pressure: 5.0,
        };
        system.calculate_blood_distribution();
        system
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

    /// Calculate blood distribution across compartments
    pub fn calculate_blood_distribution(&mut self) {
        self.arterial_blood_volume_ml = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Artery | VesselType::Arteriole))
            .map(|v| v.blood_volume_ml)
            .sum();

        self.venous_blood_volume_ml = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Vein | VesselType::Venule))
            .map(|v| v.blood_volume_ml)
            .sum();

        self.capillary_blood_volume_ml = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Capillary))
            .map(|v| v.blood_volume_ml)
            .sum();

        // Update total blood volume
        self.total_blood_volume_l = (self.arterial_blood_volume_ml +
                                     self.venous_blood_volume_ml +
                                     self.capillary_blood_volume_ml) / 1000.0;
    }

    /// Calculate blood flow rates through all vessels
    pub fn calculate_flow_rates(&mut self, cardiac_output_ml_per_min: f64) {
        // Pre-calculate total arterial conductance (1/R) for parallel circuit
        let total_arterial_conductance: f64 = self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Artery))
            .map(|v| {
                let r = v.flow_resistance();
                if r > 0.0 { 1.0 / r } else { 0.0 }
            })
            .sum();

        // Arteries receive blood from heart
        for vessel in &mut self.vessels {
            match vessel.vessel_type {
                VesselType::Artery => {
                    // Flow inversely proportional to resistance (parallel circuit)
                    // Q = ΔP / R, where total flow is distributed by conductance
                    let resistance = vessel.flow_resistance();

                    if total_arterial_conductance > 0.0 && resistance > 0.0 {
                        // Conductance fraction = (1/R) / Σ(1/R)
                        let conductance_fraction = (1.0 / resistance) / total_arterial_conductance;
                        vessel.blood_flow_rate_ml_per_min = cardiac_output_ml_per_min * conductance_fraction * 0.25;
                    } else {
                        vessel.blood_flow_rate_ml_per_min = 0.0;
                    }
                    vessel.calculate_velocity();
                }
                VesselType::Arteriole | VesselType::Capillary => {
                    // Smaller vessels get proportional flow based on resistance
                    let resistance = vessel.flow_resistance();
                    if resistance > 0.0 {
                        vessel.calculate_flow_rate(vessel.pressure_mmhg, vessel.pressure_mmhg - 20.0);
                    }
                    vessel.calculate_velocity();
                }
                VesselType::Venule | VesselType::Vein => {
                    // Veins collect blood and return to heart
                    let flow_fraction = vessel.blood_volume_ml / self.venous_blood_volume_ml.max(1.0);
                    vessel.blood_flow_rate_ml_per_min = cardiac_output_ml_per_min * flow_fraction * 0.3;
                    vessel.calculate_velocity();
                }
            }
        }
    }

    /// Calculate venous return based on venous volume and compliance
    pub fn calculate_venous_return(&mut self) {
        // Venous return depends on venous pressure gradient and compliance
        // Frank-Starling mechanism: increased venous return increases cardiac output
        let _venous_pressure_gradient = self.central_venous_pressure;

        // More blood in veins = more return
        let volume_factor = self.venous_blood_volume_ml / 3500.0; // Normal venous volume ~3500mL

        // Better compliance = easier return
        let compliance_factor = self.venous_compliance;

        self.venous_return_l_per_min = (5.0 * volume_factor * compliance_factor).clamp(2.0, 10.0);
    }

    /// Get total blood flow through arterial system
    pub fn get_total_arterial_flow(&self) -> f64 {
        self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Artery))
            .map(|v| v.blood_flow_rate_ml_per_min)
            .sum()
    }

    /// Get total blood flow through venous system
    pub fn get_total_venous_flow(&self) -> f64 {
        self.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, VesselType::Vein))
            .map(|v| v.blood_flow_rate_ml_per_min)
            .sum()
    }

    /// Get a mutable reference to a vessel by name
    pub fn get_vessel_mut(&mut self, name: &str) -> Option<&mut Vessel> {
        self.vessels.iter_mut().find(|v| v.name == name)
    }

    /// Get a reference to a vessel by name
    pub fn get_vessel(&self, name: &str) -> Option<&Vessel> {
        self.vessels.iter().find(|v| v.name == name)
    }

    /// Induce plaque buildup in a specific vessel
    pub fn add_plaque(&mut self, vessel_name: &str, plaque_amount: f64) {
        if let Some(vessel) = self.get_vessel_mut(vessel_name) {
            vessel.plaque_buildup = (vessel.plaque_buildup + plaque_amount).min(0.99);
            vessel.calculate_volume();
        }
    }

    /// Simulate plaque rupture leading to acute thrombosis
    /// This is the mechanism of acute coronary syndrome!
    pub fn rupture_plaque(&mut self, vessel_name: &str) {
        if let Some(vessel) = self.get_vessel_mut(vessel_name) {
            // Unstable plaque ruptures and triggers thrombosis
            // This acutely increases stenosis from baseline plaque to near-complete occlusion
            if vessel.plaque_buildup > 0.3 {
                // Thrombus formation on ruptured plaque
                vessel.plaque_buildup = (vessel.plaque_buildup + 0.5).min(0.95);
                vessel.inflammation = 1.0;  // Acute inflammation
                vessel.calculate_volume();
            }
        }
    }

    /// Get blood flow through a specific coronary artery
    pub fn get_coronary_flow(&self, artery_name: &str) -> f64 {
        self.get_vessel(artery_name)
            .map(|v| v.blood_flow_rate_ml_per_min)
            .unwrap_or(0.0)
    }
}

impl Organ for VascularSystem {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // 0. Update mean arterial pressure from blood pressure
        self.mean_arterial_pressure = patient.blood.get_mean_arterial_pressure();

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

        // 11. Calculate blood distribution across compartments
        self.calculate_blood_distribution();

        // 12. Calculate venous return
        self.calculate_venous_return();

        // 13. Estimate cardiac output (simplified - would normally come from heart)
        // Cardiac output = heart rate × stroke volume
        // Using a simple estimate based on venous return (Frank-Starling)
        self.cardiac_output_l_per_min = self.venous_return_l_per_min;

        // 14. Calculate blood flow rates through all vessels
        self.calculate_flow_rates(self.cardiac_output_l_per_min * 1000.0); // Convert to mL/min

        // 15. Update central venous pressure based on venous blood volume
        // More blood in veins = higher CVP
        let normal_venous_volume = 3500.0; // mL
        let volume_ratio = self.venous_blood_volume_ml / normal_venous_volume;
        self.central_venous_pressure = (5.0 * volume_ratio).clamp(0.0, 15.0);
    }

    fn get_summary(&self) -> String {
        format!(
            "Vascular - TPR: {:.2}, MAP: {:.0} mmHg, CVP: {:.1} mmHg, \
             Blood Vol: {:.2}L (Art: {:.0}mL, Ven: {:.0}mL, Cap: {:.0}mL), \
             CO: {:.2}L/min, VR: {:.2}L/min, \
             Vessel health: {:.1}%, Plaque: {:.1}%, Stenoses: {}, \
             Compliance: {:.2}, NO: {:.2}, ET-1: {:.2}",
            self.total_peripheral_resistance,
            self.mean_arterial_pressure,
            self.central_venous_pressure,
            self.total_blood_volume_l,
            self.arterial_blood_volume_ml,
            self.venous_blood_volume_ml,
            self.capillary_blood_volume_ml,
            self.cardiac_output_l_per_min,
            self.venous_return_l_per_min,
            self.average_vessel_health() * 100.0,
            self.average_plaque_burden() * 100.0,
            self.critically_stenosed_count(),
            self.arterial_compliance,
            self.nitric_oxide_level,
            self.endothelin_level
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
