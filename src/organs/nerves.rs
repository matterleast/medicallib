use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use std::any::Any;
use std::collections::HashMap;

/// Types of nerve fibers by diameter and conduction speed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NerveFiberType {
    AAlpha,  // Motor neurons, proprioception (80-120 m/s)
    ABeta,   // Touch, pressure (35-75 m/s)
    AGamma,  // Muscle spindles (15-30 m/s)
    ADelta,  // Pain, temperature, touch (5-30 m/s)
    B,       // Preganglionic autonomic (3-15 m/s)
    C,       // Pain, temperature, postganglionic (0.5-2 m/s)
}

impl NerveFiberType {
    /// Get the typical conduction velocity for this fiber type (m/s)
    pub fn conduction_velocity(&self) -> f64 {
        match self {
            NerveFiberType::AAlpha => 100.0,
            NerveFiberType::ABeta => 55.0,
            NerveFiberType::AGamma => 22.5,
            NerveFiberType::ADelta => 17.5,
            NerveFiberType::B => 9.0,
            NerveFiberType::C => 1.25,
        }
    }

    /// Get the typical diameter in micrometers
    pub fn diameter_um(&self) -> f64 {
        match self {
            NerveFiberType::AAlpha => 15.0,
            NerveFiberType::ABeta => 8.0,
            NerveFiberType::AGamma => 5.0,
            NerveFiberType::ADelta => 3.0,
            NerveFiberType::B => 2.0,
            NerveFiberType::C => 0.5,
        }
    }
}

/// A bundle of nerve fibers
#[derive(Debug, Clone)]
pub struct NerveBundle {
    pub name: String,
    pub fiber_type: NerveFiberType,
    pub fiber_count: u32,
    pub health: f64,              // 0.0-1.0
    pub myelination: f64,          // 0.0-1.0 (affects conduction speed)
    pub damage_severity: f64,      // 0.0-1.0
    pub regeneration_progress: f64, // 0.0-1.0
}

impl NerveBundle {
    pub fn new(name: &str, fiber_type: NerveFiberType, fiber_count: u32) -> Self {
        Self {
            name: name.to_string(),
            fiber_type,
            fiber_count,
            health: 1.0,
            myelination: 1.0,
            damage_severity: 0.0,
            regeneration_progress: 0.0,
        }
    }

    /// Get effective conduction velocity considering health and myelination
    pub fn effective_conduction_velocity(&self) -> f64 {
        self.fiber_type.conduction_velocity() * self.health * self.myelination
    }

    /// Damage this nerve bundle
    pub fn damage(&mut self, severity: f64) {
        self.damage_severity = (self.damage_severity + severity).min(1.0);
        self.health = (1.0 - self.damage_severity).max(0.0);
        self.regeneration_progress = 0.0;
    }
}

/// Peripheral Nervous System - nerves throughout the body
#[derive(Debug)]
pub struct Nerves {
    id: OrganId,
    pub nerve_bundles: Vec<NerveBundle>,
    pub neurotransmitters: HashMap<String, f64>, // nmol/L or arbitrary units
    pub acetylcholine_level: f64,        // Neuromuscular junction (normal: 1.0)
    pub norepinephrine_level: f64,       // Sympathetic nervous system
    pub dopamine_level: f64,             // Motor control, reward
    pub serotonin_level: f64,            // Mood, sleep
    pub gaba_level: f64,                 // Inhibitory neurotransmitter
    pub glutamate_level: f64,            // Excitatory neurotransmitter
    pub nerve_growth_factor: f64,        // Promotes regeneration
    pub overall_conduction_efficiency: f64, // 0.0-1.0
    pub sensory_function: f64,           // 0.0-1.0
    pub motor_function: f64,             // 0.0-1.0
    pub autonomic_function: f64,         // 0.0-1.0
}

impl Nerves {
    pub fn new(id: i32) -> Self {
        let mut nerve_bundles = Vec::new();

        // Major peripheral nerves
        // Motor nerves (A-alpha fibers)
        nerve_bundles.push(NerveBundle::new("Median Nerve", NerveFiberType::AAlpha, 50000));
        nerve_bundles.push(NerveBundle::new("Ulnar Nerve", NerveFiberType::AAlpha, 40000));
        nerve_bundles.push(NerveBundle::new("Radial Nerve", NerveFiberType::AAlpha, 45000));
        nerve_bundles.push(NerveBundle::new("Sciatic Nerve", NerveFiberType::AAlpha, 80000));
        nerve_bundles.push(NerveBundle::new("Femoral Nerve", NerveFiberType::AAlpha, 60000));
        nerve_bundles.push(NerveBundle::new("Tibial Nerve", NerveFiberType::AAlpha, 50000));

        // Sensory nerves (A-beta fibers)
        nerve_bundles.push(NerveBundle::new("Sensory - Arms", NerveFiberType::ABeta, 100000));
        nerve_bundles.push(NerveBundle::new("Sensory - Legs", NerveFiberType::ABeta, 120000));
        nerve_bundles.push(NerveBundle::new("Sensory - Torso", NerveFiberType::ABeta, 80000));

        // Pain fibers (A-delta and C fibers)
        nerve_bundles.push(NerveBundle::new("Pain - Fast", NerveFiberType::ADelta, 150000));
        nerve_bundles.push(NerveBundle::new("Pain - Slow", NerveFiberType::C, 200000));

        // Autonomic nerves (B and C fibers)
        nerve_bundles.push(NerveBundle::new("Vagus Nerve", NerveFiberType::B, 100000));
        nerve_bundles.push(NerveBundle::new("Sympathetic Chain", NerveFiberType::C, 80000));

        Self {
            id: id as usize,
            nerve_bundles,
            neurotransmitters: HashMap::new(),
            acetylcholine_level: 1.0,
            norepinephrine_level: 1.0,
            dopamine_level: 1.0,
            serotonin_level: 1.0,
            gaba_level: 1.0,
            glutamate_level: 1.0,
            nerve_growth_factor: 1.0,
            overall_conduction_efficiency: 1.0,
            sensory_function: 1.0,
            motor_function: 1.0,
            autonomic_function: 1.0,
        }
    }

    /// Calculate average nerve health
    pub fn average_nerve_health(&self) -> f64 {
        if self.nerve_bundles.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.nerve_bundles.iter().map(|n| n.health).sum();
        sum / self.nerve_bundles.len() as f64
    }

    /// Get number of damaged nerves
    pub fn damaged_nerve_count(&self) -> usize {
        self.nerve_bundles.iter().filter(|n| n.damage_severity > 0.1).count()
    }

    /// Damage a specific nerve
    pub fn damage_nerve(&mut self, nerve_index: usize, severity: f64) {
        if nerve_index < self.nerve_bundles.len() {
            self.nerve_bundles[nerve_index].damage(severity);
        }
    }

    /// Calculate motor function based on motor nerve health
    fn calculate_motor_function(&self) -> f64 {
        let motor_nerves: Vec<&NerveBundle> = self.nerve_bundles
            .iter()
            .filter(|n| matches!(n.fiber_type, NerveFiberType::AAlpha))
            .collect();

        if motor_nerves.is_empty() {
            return 0.0;
        }

        let avg_health: f64 = motor_nerves.iter().map(|n| n.health).sum::<f64>()
            / motor_nerves.len() as f64;
        avg_health
    }

    /// Calculate sensory function based on sensory nerve health
    fn calculate_sensory_function(&self) -> f64 {
        let sensory_nerves: Vec<&NerveBundle> = self.nerve_bundles
            .iter()
            .filter(|n| matches!(n.fiber_type, NerveFiberType::ABeta | NerveFiberType::ADelta | NerveFiberType::C))
            .collect();

        if sensory_nerves.is_empty() {
            return 0.0;
        }

        let avg_health: f64 = sensory_nerves.iter().map(|n| n.health).sum::<f64>()
            / sensory_nerves.len() as f64;
        avg_health
    }

    /// Calculate autonomic function based on autonomic nerve health
    fn calculate_autonomic_function(&self) -> f64 {
        let autonomic_nerves: Vec<&NerveBundle> = self.nerve_bundles
            .iter()
            .filter(|n| matches!(n.fiber_type, NerveFiberType::B))
            .collect();

        if autonomic_nerves.is_empty() {
            return 0.5; // Default moderate function
        }

        let avg_health: f64 = autonomic_nerves.iter().map(|n| n.health).sum::<f64>()
            / autonomic_nerves.len() as f64;
        avg_health
    }
}

impl Organ for Nerves {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // 1. Neurotransmitter synthesis and degradation
        // Acetylcholine - requires choline (from diet) and acetyl-CoA (from glucose)
        let glucose_factor = (patient.blood.chemistry.glucose_mg_dl / 90.0).clamp(0.5, 1.5);
        self.acetylcholine_level = (self.acetylcholine_level * 0.98 + glucose_factor * 0.02)
            .clamp(0.1, 2.0);

        // Norepinephrine - affected by stress response and blood pressure
        let map = patient.blood.get_mean_arterial_pressure();
        let stress_factor = if map < 70.0 {
            1.5 // Increase to raise BP
        } else if map > 110.0 {
            0.7 // Decrease to lower BP
        } else {
            1.0
        };
        self.norepinephrine_level = (self.norepinephrine_level * 0.95 + stress_factor * 0.05)
            .clamp(0.3, 2.0);

        // Dopamine, serotonin, GABA - affected by blood oxygen and glucose
        let o2_sat = patient.blood.gases.sao2_percent / 100.0;
        let metabolic_health = (o2_sat + glucose_factor) / 2.0;

        self.dopamine_level = (self.dopamine_level * 0.97 + metabolic_health * 0.03)
            .clamp(0.2, 1.8);
        self.serotonin_level = (self.serotonin_level * 0.98 + metabolic_health * 0.02)
            .clamp(0.3, 1.7);
        self.gaba_level = (self.gaba_level * 0.96 + metabolic_health * 0.04)
            .clamp(0.4, 1.6);
        self.glutamate_level = (self.glutamate_level * 0.97 + metabolic_health * 0.03)
            .clamp(0.5, 1.5);

        // 2. Nerve regeneration - requires nerve growth factor and good blood supply
        // NGF production depends on adequate nutrition
        let protein_factor = (patient.blood.chemistry.total_protein_g_dl / 7.0).clamp(0.5, 1.5);
        self.nerve_growth_factor = (self.nerve_growth_factor * 0.99 + protein_factor * o2_sat * 0.01)
            .clamp(0.2, 2.0);

        // Regenerate damaged nerves (peripheral nerves can regenerate at ~1mm/day)
        let regeneration_rate = self.nerve_growth_factor * 0.001 * delta_time_s; // Very slow

        for nerve in &mut self.nerve_bundles {
            if nerve.damage_severity > 0.0 {
                // Progress regeneration
                nerve.regeneration_progress += regeneration_rate;
                nerve.regeneration_progress = nerve.regeneration_progress.min(1.0);

                // Reduce damage as regeneration progresses
                if nerve.regeneration_progress > 0.3 {
                    nerve.damage_severity -= regeneration_rate * 0.5;
                    nerve.damage_severity = nerve.damage_severity.max(0.0);
                    nerve.health = (1.0 - nerve.damage_severity).clamp(0.0, 1.0);
                }
            }
        }

        // 3. Update nerve conduction efficiency based on blood chemistry
        // Electrolytes are critical for nerve conduction
        let sodium_factor = (patient.blood.chemistry.sodium_meq_l / 140.0).clamp(0.5, 1.5);
        let potassium_factor = (patient.blood.chemistry.potassium_meq_l / 4.0).clamp(0.5, 1.5);
        let calcium_factor = (patient.blood.chemistry.calcium_mg_dl / 9.5).clamp(0.5, 1.5);

        let electrolyte_efficiency = (sodium_factor + potassium_factor + calcium_factor) / 3.0;

        // Glucose is needed for nerve metabolism
        let energy_efficiency = glucose_factor * o2_sat;

        // Overall conduction efficiency
        self.overall_conduction_efficiency = (electrolyte_efficiency * 0.5 + energy_efficiency * 0.5)
            .clamp(0.1, 1.0);

        // Apply efficiency to nerve bundles
        for nerve in &mut self.nerve_bundles {
            nerve.myelination = (nerve.myelination * 0.99 + self.overall_conduction_efficiency * 0.01)
                .clamp(0.3, 1.0);
        }

        // 4. Calculate functional capabilities
        self.motor_function = self.calculate_motor_function() * self.overall_conduction_efficiency;
        self.sensory_function = self.calculate_sensory_function() * self.overall_conduction_efficiency;
        self.autonomic_function = self.calculate_autonomic_function() * self.overall_conduction_efficiency;

        // 5. Autonomic nervous system effects on other organs
        // Sympathetic (norepinephrine) increases heart rate and blood pressure
        // This is handled by existing organs, but we can influence it slightly

        // 6. Toxin effects on nerves
        // High toxin levels damage peripheral nerves
        let toxin_damage = patient.blood.chemistry.toxin_level_au * 0.0001 * delta_time_s;
        if toxin_damage > 0.001 {
            for nerve in &mut self.nerve_bundles {
                nerve.health -= toxin_damage;
                nerve.health = nerve.health.max(0.0);
                if nerve.health < 0.9 {
                    nerve.damage_severity = 1.0 - nerve.health;
                }
            }
        }

        // 7. Blood glucose effects - both hypo and hyperglycemia damage nerves
        let glucose = patient.blood.chemistry.glucose_mg_dl;
        if glucose < 60.0 || glucose > 180.0 {
            let glucose_damage = 0.0001 * delta_time_s;
            for nerve in &mut self.nerve_bundles {
                nerve.health -= glucose_damage;
                nerve.health = nerve.health.max(0.0);
            }
        }
    }

    fn get_summary(&self) -> String {
        format!(
            "Nerves - Health: {:.1}%, Damaged: {}, Conduction: {:.1}%, \
             Motor: {:.1}%, Sensory: {:.1}%, Autonomic: {:.1}%, \
             ACh: {:.2}, NE: {:.2}, DA: {:.2}, 5-HT: {:.2}, NGF: {:.2}",
            self.average_nerve_health() * 100.0,
            self.damaged_nerve_count(),
            self.overall_conduction_efficiency * 100.0,
            self.motor_function * 100.0,
            self.sensory_function * 100.0,
            self.autonomic_function * 100.0,
            self.acetylcholine_level,
            self.norepinephrine_level,
            self.dopamine_level,
            self.serotonin_level,
            self.nerve_growth_factor
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Nerves"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
