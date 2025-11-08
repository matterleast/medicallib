//! Myocardial tissue simulation with emergent pathophysiology
//!
//! This module simulates individual myocardial regions and their cellular states.
//! Pathology emerges from the simulation rather than being hardcoded:
//! - Ischemia develops when O2 delivery < O2 demand
//! - Cellular injury progresses through states based on ischemia duration
//! - ECG changes emerge from altered cellular electrical properties
//! - Arrhythmias arise from electrical instability and heterogeneity

use std::collections::VecDeque;

/// Myocardial cell state - progresses based on ischemia duration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellularState {
    /// Healthy cells with normal electrical and mechanical function
    Healthy,
    /// Ischemic: O2 delivery < demand, but reversible
    /// - Shortened action potential duration
    /// - Reduced contractility
    /// - Anaerobic metabolism → lactic acid → pain
    Ischemic {
        duration_seconds: f64,
    },
    /// Injured: Prolonged ischemia causing membrane instability
    /// - Abnormal resting membrane potential
    /// - Ectopic pacemaker activity
    /// - ST segment changes
    Injured {
        duration_seconds: f64,
    },
    /// Necrotic: Cell death, no recovery possible
    /// - No electrical activity
    /// - Forms conduction block
    /// - Eventual scar formation
    Necrotic {
        days_old: f64,
    },
}

impl CellularState {
    /// Progress the cell state based on ischemia
    pub fn progress(&mut self, is_ischemic: bool, delta_time_s: f64) {
        match self {
            CellularState::Healthy => {
                if is_ischemic {
                    *self = CellularState::Ischemic { duration_seconds: 0.0 };
                }
            }
            CellularState::Ischemic { duration_seconds } => {
                if is_ischemic {
                    *duration_seconds += delta_time_s;
                    // After 20 minutes of ischemia, cells become injured
                    if *duration_seconds > 1200.0 {
                        *self = CellularState::Injured { duration_seconds: 0.0 };
                    }
                } else {
                    // Reperfusion - can recover if caught early
                    if *duration_seconds < 300.0 {  // < 5 minutes
                        *self = CellularState::Healthy;
                    } else {
                        // Partial recovery but moves to injured state
                        *self = CellularState::Injured { duration_seconds: 0.0 };
                    }
                }
            }
            CellularState::Injured { duration_seconds } => {
                if is_ischemic {
                    *duration_seconds += delta_time_s;
                    // After 60 minutes of injury, cells die
                    if *duration_seconds > 3600.0 {
                        *self = CellularState::Necrotic { days_old: 0.0 };
                    }
                } else {
                    // Slow recovery possible, but takes time
                    *duration_seconds += delta_time_s;
                    if *duration_seconds > 7200.0 {  // 2 hours of reperfusion
                        *self = CellularState::Healthy;
                    }
                }
            }
            CellularState::Necrotic { days_old } => {
                // Dead is dead, but scars mature over time
                *days_old += delta_time_s / 86400.0;
            }
        }
    }

    /// Get the resting membrane potential (mV)
    /// Normal: -90 mV, Ischemic: -80 mV, Injured: -60 mV, Necrotic: 0 mV
    pub fn resting_potential_mv(&self) -> f64 {
        match self {
            CellularState::Healthy => -90.0,
            CellularState::Ischemic { duration_seconds } => {
                // Gradually depolarizes as ischemia worsens
                -90.0 + (duration_seconds / 1200.0) * 10.0
            }
            CellularState::Injured { duration_seconds } => {
                // Very depolarized
                -60.0 + (duration_seconds / 3600.0) * 20.0
            }
            CellularState::Necrotic { .. } => 0.0,  // No potential
        }
    }

    /// Get action potential duration (ms)
    /// Normal: 250 ms, Ischemic: shorter, Injured: variable, Necrotic: 0
    pub fn action_potential_duration_ms(&self) -> f64 {
        match self {
            CellularState::Healthy => 250.0,
            CellularState::Ischemic { duration_seconds } => {
                // ATP depletion shortens action potential
                250.0 - (duration_seconds / 1200.0) * 100.0
            }
            CellularState::Injured { .. } => 180.0,  // Shortened
            CellularState::Necrotic { .. } => 0.0,
        }
    }

    /// Get contractility (0.0 - 1.0)
    pub fn contractility(&self) -> f64 {
        match self {
            CellularState::Healthy => 1.0,
            CellularState::Ischemic { duration_seconds } => {
                (1.0 - duration_seconds / 1200.0).max(0.3)
            }
            CellularState::Injured { duration_seconds } => {
                (0.3 - duration_seconds / 3600.0 * 0.3).max(0.0)
            }
            CellularState::Necrotic { .. } => 0.0,
        }
    }

    /// Get automaticity (spontaneous depolarization rate, impulses/min)
    /// Normal: 0, Injured cells can become pacemakers
    pub fn automaticity_rate(&self) -> f64 {
        match self {
            CellularState::Healthy | CellularState::Ischemic { .. } => 0.0,
            CellularState::Injured { duration_seconds } => {
                // Injured cells can fire spontaneously (PVCs)
                if *duration_seconds > 600.0 {  // After 10 min of injury
                    (duration_seconds - 600.0) / 1800.0 * 30.0  // Up to 30 ectopic beats/min
                } else {
                    0.0
                }
            }
            CellularState::Necrotic { .. } => 0.0,
        }
    }

    /// Get conduction velocity (m/s)
    /// Normal: 0.5 m/s, reduced in ischemia/injury, blocked in necrosis
    pub fn conduction_velocity(&self) -> f64 {
        match self {
            CellularState::Healthy => 0.5,
            CellularState::Ischemic { duration_seconds } => {
                (0.5 - duration_seconds / 1200.0 * 0.2).max(0.2)
            }
            CellularState::Injured { .. } => 0.15,
            CellularState::Necrotic { .. } => 0.0,  // Complete block
        }
    }

    /// Check if cell is electrically unstable (prone to arrhythmias)
    pub fn is_unstable(&self) -> bool {
        matches!(self, CellularState::Injured { .. })
    }
}

/// Anatomical region of the heart supplied by a specific coronary artery
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MyocardialRegion {
    /// Anterior wall - supplied by LAD
    Anterior,
    /// Septal wall - supplied by LAD septal branches
    Septal,
    /// Lateral wall - supplied by LCx
    Lateral,
    /// Inferior wall - supplied by RCA
    Inferior,
    /// Posterior wall - supplied by RCA or LCx
    Posterior,
    /// Right ventricle - supplied by RCA
    RightVentricular,
}

impl MyocardialRegion {
    /// Get the ECG leads that best visualize this region
    pub fn primary_leads(&self) -> Vec<usize> {
        match self {
            MyocardialRegion::Anterior => vec![2, 3, 4],      // V1-V3
            MyocardialRegion::Septal => vec![1, 2],            // V1-V2
            MyocardialRegion::Lateral => vec![0, 5, 6],        // I, aVL, V5-V6
            MyocardialRegion::Inferior => vec![7, 8, 9],       // II, III, aVF
            MyocardialRegion::Posterior => vec![10, 11],       // V7-V8 (if available)
            MyocardialRegion::RightVentricular => vec![10],    // V4R
        }
    }

    /// Get the supplying coronary artery name
    pub fn supplying_artery(&self) -> &'static str {
        match self {
            MyocardialRegion::Anterior => "LAD",
            MyocardialRegion::Septal => "LAD",
            MyocardialRegion::Lateral => "LCx",
            MyocardialRegion::Inferior => "RCA",
            MyocardialRegion::Posterior => "RCA/LCx",
            MyocardialRegion::RightVentricular => "RCA",
        }
    }
}

/// A region of myocardial tissue with its own blood supply and cellular state
#[derive(Debug, Clone)]
pub struct MyocardialSegment {
    pub region: MyocardialRegion,
    pub cellular_state: CellularState,
    pub mass_grams: f64,

    // Perfusion tracking
    pub blood_flow_ml_per_min: f64,           // Actual flow through coronary artery
    pub baseline_flow_ml_per_min: f64,        // Normal flow requirement
    pub oxygen_delivery_ml_per_min: f64,      // O2 delivery = flow × O2 content
    pub oxygen_consumption_ml_per_min: f64,   // O2 demand from metabolism

    // Metabolic byproducts
    pub lactic_acid_mmol: f64,                // Anaerobic metabolism
    pub adenosine_au: f64,                    // Vasodilator released during ischemia
    pub troponin_release_ng_ml: f64,          // Marker of myocardial necrosis

    // Electrical properties
    pub depolarization_time: f64,             // When this segment depolarizes in cycle
    pub repolarization_time: f64,             // When this segment repolarizes
    pub ectopic_beats: VecDeque<f64>,         // Timing of spontaneous beats

    // Mechanical properties
    pub contractility: f64,                   // 0.0 - 1.0
    pub wall_motion_score: f64,               // 1=normal, 2=hypokinetic, 3=akinetic, 4=dyskinetic
}

impl MyocardialSegment {
    pub fn new(region: MyocardialRegion, mass_grams: f64) -> Self {
        // Calculate baseline flow: ~1 mL/min per gram of myocardium
        let baseline_flow = mass_grams;

        Self {
            region,
            cellular_state: CellularState::Healthy,
            mass_grams,
            blood_flow_ml_per_min: baseline_flow,
            baseline_flow_ml_per_min: baseline_flow,
            oxygen_delivery_ml_per_min: 0.0,
            oxygen_consumption_ml_per_min: mass_grams * 0.1,  // ~10% of flow is O2
            lactic_acid_mmol: 0.0,
            adenosine_au: 0.0,
            troponin_release_ng_ml: 0.0,
            depolarization_time: 0.0,
            repolarization_time: 0.0,
            ectopic_beats: VecDeque::new(),
            contractility: 1.0,
            wall_motion_score: 1.0,
        }
    }

    /// Update the segment's state based on blood flow and oxygen delivery
    pub fn update(&mut self, blood_flow_ml_per_min: f64, arterial_o2_content_ml_per_dl: f64, delta_time_s: f64) {
        self.blood_flow_ml_per_min = blood_flow_ml_per_min;

        // Calculate oxygen delivery: flow (mL/min) × O2 content (mL O2 per 100mL blood)
        self.oxygen_delivery_ml_per_min = (blood_flow_ml_per_min / 100.0) * arterial_o2_content_ml_per_dl;

        // Determine if tissue is ischemic
        let is_ischemic = self.oxygen_delivery_ml_per_min < self.oxygen_consumption_ml_per_min;

        // Progress cellular state
        self.cellular_state.progress(is_ischemic, delta_time_s);

        // Update metabolic byproducts
        if is_ischemic {
            // Anaerobic metabolism produces lactic acid
            let o2_deficit = self.oxygen_consumption_ml_per_min - self.oxygen_delivery_ml_per_min;
            self.lactic_acid_mmol += o2_deficit * delta_time_s * 0.01;

            // Adenosine released to try to dilate coronary arteries
            self.adenosine_au += o2_deficit * delta_time_s * 0.005;
        } else {
            // Washout of metabolites when perfused
            self.lactic_acid_mmol *= 0.95_f64.powf(delta_time_s);
            self.adenosine_au *= 0.90_f64.powf(delta_time_s);
        }

        // Troponin release from injured/necrotic cells
        match self.cellular_state {
            CellularState::Injured { duration_seconds } => {
                // Troponin starts rising after 3-4 hours of injury
                if duration_seconds > 10800.0 {
                    self.troponin_release_ng_ml += delta_time_s * 0.01;
                }
            }
            CellularState::Necrotic { days_old } => {
                // Peak troponin 24-48 hours after infarction
                if days_old < 2.0 {
                    self.troponin_release_ng_ml += delta_time_s * 0.05;
                } else {
                    // Gradual decline
                    self.troponin_release_ng_ml *= 0.98_f64.powf(delta_time_s);
                }
            }
            _ => {}
        }

        // Update contractility from cellular state
        self.contractility = self.cellular_state.contractility();

        // Update wall motion score
        self.wall_motion_score = if self.contractility > 0.8 {
            1.0  // Normal
        } else if self.contractility > 0.5 {
            2.0  // Hypokinetic
        } else if self.contractility > 0.2 {
            3.0  // Akinetic
        } else {
            4.0  // Dyskinetic or akinetic
        };

        // Generate ectopic beats from unstable tissue
        if self.cellular_state.is_unstable() {
            let automaticity = self.cellular_state.automaticity_rate();
            // Probabilistic ectopic beat generation
            let beat_probability = automaticity / 60.0 * delta_time_s;
            if rand::random::<f64>() < beat_probability {
                self.ectopic_beats.push_back(0.0);  // Mark new ectopic beat
            }
        }

        // Age ectopic beats
        for beat_time in self.ectopic_beats.iter_mut() {
            *beat_time += delta_time_s;
        }

        // Remove old ectopic beats
        while self.ectopic_beats.front().map_or(false, |&t| t > 2.0) {
            self.ectopic_beats.pop_front();
        }
    }

    /// Get the ST segment deviation (mV) for ECG
    /// Ischemia causes ST depression or elevation depending on depth
    pub fn st_segment_deviation_mv(&self) -> f64 {
        match self.cellular_state {
            CellularState::Healthy => 0.0,
            CellularState::Ischemic { duration_seconds } => {
                // Transmural ischemia → ST elevation
                // Subendocardial ischemia → ST depression
                // For simplicity, assuming transmural ischemia here
                let severity = (duration_seconds / 1200.0).min(1.0);
                severity * 3.0  // Up to 3mm ST elevation
            }
            CellularState::Injured { duration_seconds } => {
                // Persistent ST elevation
                let severity = (duration_seconds / 3600.0).min(1.0);
                2.0 + severity * 2.0  // 2-4mm elevation
            }
            CellularState::Necrotic { days_old } => {
                // ST elevation resolves over days, Q waves develop
                if days_old < 7.0 {
                    (2.0 - days_old * 0.3).max(0.0)
                } else {
                    0.0
                }
            }
        }
    }

    /// Get T wave inversion (mV)
    pub fn t_wave_inversion_mv(&self) -> f64 {
        match self.cellular_state {
            CellularState::Healthy | CellularState::Ischemic { .. } => 0.0,
            CellularState::Injured { duration_seconds } => {
                // T wave inversion develops after ST elevation
                if duration_seconds > 7200.0 {  // After 2 hours
                    -0.5
                } else {
                    0.0
                }
            }
            CellularState::Necrotic { days_old } => {
                // Deep T wave inversion in evolving MI
                if days_old < 30.0 {
                    -0.8
                } else {
                    -0.3  // Persistent inversion
                }
            }
        }
    }

    /// Check if segment has pathological Q waves (necrosis)
    pub fn has_pathologic_q_wave(&self) -> bool {
        matches!(self.cellular_state, CellularState::Necrotic { days_old } if days_old > 0.5)
    }

    /// Get percentage of tissue that is viable
    pub fn viability_percent(&self) -> f64 {
        match self.cellular_state {
            CellularState::Healthy => 100.0,
            CellularState::Ischemic { duration_seconds } => {
                100.0 - (duration_seconds / 1200.0 * 20.0).min(20.0)
            }
            CellularState::Injured { duration_seconds } => {
                80.0 - (duration_seconds / 3600.0 * 80.0).min(80.0)
            }
            CellularState::Necrotic { .. } => 0.0,
        }
    }
}
