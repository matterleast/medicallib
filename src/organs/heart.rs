//! Heart organ simulation with emergent pathophysiology
//!
//! Simulates cardiac electrical and mechanical function including:
//! - 4 chambers (left/right atrium and ventricle)
//! - 4 valves (mitral, tricuspid, aortic, pulmonary)
//! - Myocardial segments with perfusion tracking
//! - EKG waveform generation based on actual tissue electrical properties
//! - Emergent arrhythmias from cellular instability
//! - STEMI, arrhythmias, and cardiac arrest arise from simulation

use crate::organ::{Organ, OrganId};
use crate::patient::Patient;
use crate::myocardial_tissue::{MyocardialSegment, MyocardialRegion, CellularState};
use std::collections::VecDeque;

/// Chamber state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChamberState {
    Systole,
    Diastole,
}

/// Heart valve
#[derive(Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub is_open: bool,
    pub stenosis_severity: f64,  // 0.0 = normal, 1.0 = complete stenosis
    pub regurgitation_severity: f64,  // 0.0 = normal, 1.0 = severe regurgitation
}

/// Heart chamber
#[derive(Debug, Clone)]
pub struct Chamber {
    pub name: String,
    pub state: ChamberState,
    pub volume_ml: f64,
    pub pressure_mmhg: f64,
}

/// Rhythm type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rhythm {
    /// Normal sinus rhythm
    Sinus,
    /// Sinus tachycardia
    SinusTachycardia,
    /// Sinus bradycardia
    SinusBradycardia,
    /// Premature ventricular contractions
    PVCs,
    /// Ventricular tachycardia
    VentricularTachycardia,
    /// Ventricular fibrillation
    VentricularFibrillation,
    /// Asystole (cardiac arrest)
    Asystole,
}

/// Heart organ with emergent pathophysiology
#[derive(Debug)]
pub struct Heart {
    id: OrganId,
    /// Left atrium
    pub left_atrium: Chamber,
    /// Right atrium
    pub right_atrium: Chamber,
    /// Left ventricle
    pub left_ventricle: Chamber,
    /// Right ventricle
    pub right_ventricle: Chamber,
    /// Mitral valve (left atrium to left ventricle)
    pub mitral_valve: Valve,
    /// Tricuspid valve (right atrium to right ventricle)
    pub tricuspid_valve: Valve,
    /// Aortic valve (left ventricle to aorta)
    pub aortic_valve: Valve,
    /// Pulmonary valve (right ventricle to pulmonary artery)
    pub pulmonary_valve: Valve,
    /// Myocardial segments with perfusion tracking
    pub myocardial_segments: Vec<MyocardialSegment>,
    /// Heart rate (beats per minute)
    pub heart_rate_bpm: f64,
    /// Baseline heart rate (for comparison)
    pub baseline_heart_rate_bpm: f64,
    /// Ejection fraction (percentage)
    pub ejection_fraction_percent: f64,
    /// Aortic pressure (systolic/diastolic)
    pub aortic_pressure_systolic: f64,
    pub aortic_pressure_diastolic: f64,
    /// EKG lead data (generated from actual tissue electrical properties)
    pub ekg_leads: Vec<VecDeque<f64>>,
    /// Internal cardiac cycle timer
    cardiac_cycle_time: f64,
    /// Current rhythm
    pub rhythm: Rhythm,
    /// PVC counter for detecting VT/VF progression
    pvc_count_last_minute: usize,
    /// Time in VT (for progression to VF)
    vt_duration_seconds: f64,
    /// Time in VF (for progression to asystole)
    vf_duration_seconds: f64,
}

impl Heart {
    /// Create a new heart with myocardial segments
    pub fn new(id: OrganId, num_leads: usize) -> Self {
        let num_leads = match num_leads {
            3 | 5 | 12 => num_leads,
            _ => 12,  // Default to 12-lead
        };

        let mut ekg_leads = Vec::new();
        for _ in 0..num_leads {
            ekg_leads.push(VecDeque::with_capacity(1000));
        }

        // Create myocardial segments with typical mass distribution
        // Total LV mass ~150g, RV mass ~50g
        let mut myocardial_segments = Vec::new();
        myocardial_segments.push(MyocardialSegment::new(MyocardialRegion::Anterior, 40.0));
        myocardial_segments.push(MyocardialSegment::new(MyocardialRegion::Septal, 30.0));
        myocardial_segments.push(MyocardialSegment::new(MyocardialRegion::Lateral, 35.0));
        myocardial_segments.push(MyocardialSegment::new(MyocardialRegion::Inferior, 35.0));
        myocardial_segments.push(MyocardialSegment::new(MyocardialRegion::Posterior, 25.0));
        myocardial_segments.push(MyocardialSegment::new(MyocardialRegion::RightVentricular, 50.0));

        Self {
            id,
            left_atrium: Chamber {
                name: "Left Atrium".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 50.0,
                pressure_mmhg: 8.0,
            },
            right_atrium: Chamber {
                name: "Right Atrium".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 50.0,
                pressure_mmhg: 4.0,
            },
            left_ventricle: Chamber {
                name: "Left Ventricle".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 120.0,
                pressure_mmhg: 8.0,
            },
            right_ventricle: Chamber {
                name: "Right Ventricle".to_string(),
                state: ChamberState::Diastole,
                volume_ml: 120.0,
                pressure_mmhg: 4.0,
            },
            mitral_valve: Valve {
                name: "Mitral".to_string(),
                is_open: true,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            tricuspid_valve: Valve {
                name: "Tricuspid".to_string(),
                is_open: true,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            aortic_valve: Valve {
                name: "Aortic".to_string(),
                is_open: false,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            pulmonary_valve: Valve {
                name: "Pulmonary".to_string(),
                is_open: false,
                stenosis_severity: 0.0,
                regurgitation_severity: 0.0,
            },
            myocardial_segments,
            heart_rate_bpm: 75.0,
            baseline_heart_rate_bpm: 75.0,
            ejection_fraction_percent: 60.0,
            aortic_pressure_systolic: 120.0,
            aortic_pressure_diastolic: 80.0,
            ekg_leads,
            cardiac_cycle_time: 0.0,
            rhythm: Rhythm::Sinus,
            pvc_count_last_minute: 0,
            vt_duration_seconds: 0.0,
            vf_duration_seconds: 0.0,
        }
    }

    /// Generate EKG waveform based on actual myocardial electrical properties
    /// This creates emergent ECG changes from tissue pathology!
    fn generate_ekg(&self, lead_index: usize) -> f64 {
        // Handle VF and asystole
        if self.rhythm == Rhythm::VentricularFibrillation {
            // Chaotic, irregular waveform
            return (rand::random::<f64>() - 0.5) * 0.3;
        }

        if self.rhythm == Rhythm::Asystole {
            // Flat line (with tiny electrical noise)
            return (rand::random::<f64>() - 0.5) * 0.01;
        }

        let cycle_progress = self.cardiac_cycle_time / (60.0 / self.heart_rate_bpm);
        let phase = cycle_progress * 2.0 * std::f64::consts::PI;

        // P wave (atrial depolarization) - normal
        let p_wave = if cycle_progress < 0.15 {
            0.2 * (phase * 7.0).sin()
        } else {
            0.0
        };

        // QRS complex - affected by necrosis (Q waves) and conduction blocks
        let qrs = if cycle_progress > 0.2 && cycle_progress < 0.3 {
            let mut qrs_amplitude = 1.0;

            // Check for Q waves (necrosis in regions seen by this lead)
            for segment in &self.myocardial_segments {
                if segment.region.primary_leads().contains(&lead_index) {
                    if segment.has_pathologic_q_wave() {
                        // Pathological Q wave - initial negative deflection
                        if cycle_progress < 0.22 {
                            return -0.3 * ((phase - 1.3) * 20.0).sin();
                        }
                    }

                    // Reduced amplitude if segment is dead
                    qrs_amplitude *= segment.contractility.max(0.3);
                }
            }

            qrs_amplitude * ((phase - 1.5) * 15.0).sin()
        } else {
            0.0
        };

        // ST segment - elevated in ischemia/injury
        let st_segment = if cycle_progress > 0.3 && cycle_progress < 0.5 {
            let mut st_deviation = 0.0;

            // Each segment contributes ST changes to its corresponding leads
            for segment in &self.myocardial_segments {
                if segment.region.primary_leads().contains(&lead_index) {
                    st_deviation += segment.st_segment_deviation_mv();
                }
            }

            // Normalize ST deviation
            st_deviation * 0.1  // Scale to mV
        } else {
            0.0
        };

        // T wave - inverted in ischemia
        let t_wave = if cycle_progress > 0.5 && cycle_progress < 0.7 {
            let mut t_amplitude = 0.3;

            // Check for T wave inversions
            for segment in &self.myocardial_segments {
                if segment.region.primary_leads().contains(&lead_index) {
                    t_amplitude += segment.t_wave_inversion_mv() * 0.5;
                }
            }

            t_amplitude * ((phase - 3.5) * 6.0).sin()
        } else {
            0.0
        };

        // PVCs - Wide, bizarre QRS
        if self.rhythm == Rhythm::PVCs {
            // Every few beats is a PVC
            if (self.cardiac_cycle_time * 10.0) as i32 % 3 == 0 {
                // Wide, bizarre complex
                if cycle_progress > 0.15 && cycle_progress < 0.4 {
                    return 0.7 * ((phase - 1.2) * 8.0).sin();
                }
            }
        }

        // VT - Regular wide complex tachycardia
        if self.rhythm == Rhythm::VentricularTachycardia {
            if cycle_progress > 0.15 && cycle_progress < 0.4 {
                return 0.8 * ((phase - 1.2) * 8.0).sin();
            }
            return 0.0;
        }

        // Different leads have different amplitudes and axes
        let amplitude_factor = match lead_index {
            0 => 1.0,    // Lead I
            1 => 0.5,    // Lead II
            2 => 0.8,    // Lead III
            3 => 1.2,    // V1-V6
            4 => 1.3,
            5 => 1.1,
            _ => 0.7 + (lead_index as f64 * 0.05),
        };

        (p_wave + qrs + st_segment + t_wave) * amplitude_factor
    }

    /// Update myocardial segments based on coronary blood flow
    fn update_myocardial_perfusion(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // Calculate arterial oxygen content
        let hgb = patient.blood.cells.hemoglobin_g_dl;
        let sao2 = patient.blood.gases.sao2_percent / 100.0;
        let pao2 = patient.blood.gases.pao2_mmhg;

        // O2 content = (Hgb × 1.34 × SaO2) + (0.003 × PaO2)
        let arterial_o2_content = (hgb * 1.34 * sao2) + (0.003 * pao2);

        // Get vascular system to check coronary flow
        // We need to find the vascular system organ
        let lad_flow = 40.0;  // Default flow - will be updated
        let lcx_flow = 30.0;
        let rca_flow = 35.0;

        // Try to get actual coronary flows from vascular system
        for organ in patient.organs() {
            if organ.get_type() == "VascularSystem" {
                if let Some(vascular) = organ.as_any().downcast_ref::<crate::organs::vascular::VascularSystem>() {
                    // Update flows from actual vascular system
                    let lad_flow_actual = vascular.get_coronary_flow("LAD");
                    let lcx_flow_actual = vascular.get_coronary_flow("LCx");
                    let rca_flow_actual = vascular.get_coronary_flow("RCA");

                    // Update each segment based on its supplying artery
                    for segment in &mut self.myocardial_segments {
                        let flow = match segment.region {
                            MyocardialRegion::Anterior | MyocardialRegion::Septal => lad_flow_actual,
                            MyocardialRegion::Lateral => lcx_flow_actual,
                            MyocardialRegion::Inferior | MyocardialRegion::Posterior | MyocardialRegion::RightVentricular => rca_flow_actual,
                        };

                        segment.update(flow, arterial_o2_content, delta_time_s);
                    }
                    return;
                }
            }
        }

        // Fallback: use default flows if vascular system not found
        for segment in &mut self.myocardial_segments {
            let flow = match segment.region {
                MyocardialRegion::Anterior | MyocardialRegion::Septal => lad_flow,
                MyocardialRegion::Lateral => lcx_flow,
                MyocardialRegion::Inferior | MyocardialRegion::Posterior | MyocardialRegion::RightVentricular => rca_flow,
            };

            segment.update(flow, arterial_o2_content, delta_time_s);
        }
    }

    /// Detect and progress arrhythmias based on myocardial instability
    fn update_rhythm(&mut self, delta_time_s: f64) {
        // Count ectopic beats from all segments
        let total_ectopic_beats: usize = self.myocardial_segments
            .iter()
            .map(|s| s.ectopic_beats.len())
            .sum();

        // Count severely ischemic/injured segments
        let unstable_segments: usize = self.myocardial_segments
            .iter()
            .filter(|s| matches!(s.cellular_state, CellularState::Injured { .. }))
            .count();

        let necrotic_segments: usize = self.myocardial_segments
            .iter()
            .filter(|s| matches!(s.cellular_state, CellularState::Necrotic { .. }))
            .count();

        // Rhythm progression logic - emergent from tissue state!
        match self.rhythm {
            Rhythm::Sinus | Rhythm::SinusTachycardia | Rhythm::SinusBradycardia => {
                if total_ectopic_beats > 0 {
                    self.rhythm = Rhythm::PVCs;
                    self.pvc_count_last_minute = total_ectopic_beats;
                }
            }
            Rhythm::PVCs => {
                // Frequent PVCs + unstable tissue → VT
                if unstable_segments >= 2 && total_ectopic_beats > 5 {
                    self.rhythm = Rhythm::VentricularTachycardia;
                    self.heart_rate_bpm = 150.0 + rand::random::<f64>() * 50.0;
                    self.vt_duration_seconds = 0.0;
                } else if total_ectopic_beats == 0 {
                    // PVCs resolved
                    self.rhythm = Rhythm::Sinus;
                }
            }
            Rhythm::VentricularTachycardia => {
                self.vt_duration_seconds += delta_time_s;

                // VT can degenerate to VF if sustained or if more tissue becomes unstable
                if self.vt_duration_seconds > 60.0 || unstable_segments >= 3 {
                    self.rhythm = Rhythm::VentricularFibrillation;
                    self.vf_duration_seconds = 0.0;
                }
            }
            Rhythm::VentricularFibrillation => {
                self.vf_duration_seconds += delta_time_s;
                self.heart_rate_bpm = 250.0 + rand::random::<f64>() * 100.0;

                // VF progresses to asystole if untreated
                if self.vf_duration_seconds > 300.0 || necrotic_segments >= 4 {
                    self.rhythm = Rhythm::Asystole;
                }

                // No effective cardiac output in VF
                self.ejection_fraction_percent = 0.0;
            }
            Rhythm::Asystole => {
                // Dead is dead
                self.heart_rate_bpm = 0.0;
                self.ejection_fraction_percent = 0.0;
            }
        }
    }

    /// Calculate ejection fraction based on myocardial contractility
    fn calculate_ejection_fraction(&mut self) {
        if self.rhythm == Rhythm::Asystole || self.rhythm == Rhythm::VentricularFibrillation {
            self.ejection_fraction_percent = 0.0;
            return;
        }

        // Average contractility of all LV segments
        let lv_segments: Vec<&MyocardialSegment> = self.myocardial_segments
            .iter()
            .filter(|s| s.region != MyocardialRegion::RightVentricular)
            .collect();

        if lv_segments.is_empty() {
            self.ejection_fraction_percent = 60.0;
            return;
        }

        let avg_contractility: f64 = lv_segments.iter()
            .map(|s| s.contractility)
            .sum::<f64>() / lv_segments.len() as f64;

        // Normal EF is 60%, scales with contractility
        self.ejection_fraction_percent = (avg_contractility * 60.0).max(0.0);
    }

    /// Get chest pain level from ischemic myocardium
    pub fn get_chest_pain_level(&self) -> f64 {
        // Lactic acid and adenosine from ischemic tissue causes pain
        self.myocardial_segments
            .iter()
            .map(|s| s.lactic_acid_mmol * 0.1 + s.adenosine_au * 0.2)
            .sum::<f64>()
            .min(10.0)
    }

    /// Get total troponin release (marker of myocardial injury)
    pub fn get_troponin_level(&self) -> f64 {
        self.myocardial_segments
            .iter()
            .map(|s| s.troponin_release_ng_ml)
            .sum()
    }

    /// Check if patient is in cardiac arrest
    pub fn is_cardiac_arrest(&self) -> bool {
        matches!(self.rhythm, Rhythm::VentricularFibrillation | Rhythm::Asystole)
    }
}

impl Organ for Heart {
    fn update(&mut self, patient: &mut Patient, delta_time_s: f64) {
        // 1. Update myocardial perfusion based on coronary blood flow
        self.update_myocardial_perfusion(patient, delta_time_s);

        // 2. Detect and progress arrhythmias based on tissue state
        self.update_rhythm(delta_time_s);

        // 3. Calculate EF from actual contractility
        self.calculate_ejection_fraction();

        // 4. Update cardiac cycle (if heart is beating)
        if !matches!(self.rhythm, Rhythm::Asystole) {
            self.cardiac_cycle_time += delta_time_s;
            let cycle_duration = 60.0 / self.heart_rate_bpm.max(1.0);

            if self.cardiac_cycle_time >= cycle_duration {
                self.cardiac_cycle_time = 0.0;
            }

            let cycle_progress = self.cardiac_cycle_time / cycle_duration;

            // Atrial systole (0.0 - 0.2)
            if cycle_progress < 0.2 {
                self.left_atrium.state = ChamberState::Systole;
                self.right_atrium.state = ChamberState::Systole;
                self.mitral_valve.is_open = true;
                self.tricuspid_valve.is_open = true;
            }
            // Ventricular systole (0.2 - 0.5)
            else if cycle_progress < 0.5 {
                self.left_atrium.state = ChamberState::Diastole;
                self.right_atrium.state = ChamberState::Diastole;
                self.left_ventricle.state = ChamberState::Systole;
                self.right_ventricle.state = ChamberState::Systole;
                self.mitral_valve.is_open = false;
                self.tricuspid_valve.is_open = false;
                self.aortic_valve.is_open = true;
                self.pulmonary_valve.is_open = true;
            }
            // Diastole (0.5 - 1.0)
            else {
                self.left_ventricle.state = ChamberState::Diastole;
                self.right_ventricle.state = ChamberState::Diastole;
                self.aortic_valve.is_open = false;
                self.pulmonary_valve.is_open = false;
                self.mitral_valve.is_open = true;
                self.tricuspid_valve.is_open = true;
            }
        }

        // 5. Calculate pressures (scaled by EF and rhythm)
        let pressure_factor = if self.is_cardiac_arrest() {
            0.0
        } else {
            self.ejection_fraction_percent / 60.0
        };

        if self.left_ventricle.state == ChamberState::Systole {
            self.aortic_pressure_systolic = (100.0 + self.ejection_fraction_percent * 0.5) * pressure_factor;
            self.left_ventricle.pressure_mmhg = self.aortic_pressure_systolic;
        } else {
            self.aortic_pressure_diastolic = (70.0 + self.ejection_fraction_percent * 0.2) * pressure_factor;
            self.left_ventricle.pressure_mmhg = 8.0 * pressure_factor;
        }

        // Update patient blood pressure
        patient.blood.blood_pressure_systolic = self.aortic_pressure_systolic;
        patient.blood.blood_pressure_diastolic = self.aortic_pressure_diastolic;

        // 6. Generate EKG data from actual tissue electrical properties
        let num_leads = self.ekg_leads.len();
        let ekg_values: Vec<f64> = (0..num_leads).map(|i| self.generate_ekg(i)).collect();
        for (lead, &ekg_value) in self.ekg_leads.iter_mut().zip(ekg_values.iter()) {
            lead.push_back(ekg_value);
            if lead.len() > 1000 {
                lead.pop_front();
            }
        }

        // 7. Respond to blood chemistry toxins (backup mechanism)
        if patient.blood.chemistry.toxin_level_au > 50.0 {
            let toxin_effect = (patient.blood.chemistry.toxin_level_au - 50.0) * 0.1;
            self.heart_rate_bpm = (self.baseline_heart_rate_bpm - toxin_effect).max(40.0);
        }
    }

    fn get_summary(&self) -> String {
        let rhythm_str = match self.rhythm {
            Rhythm::Sinus => "Sinus",
            Rhythm::SinusTachycardia => "Sinus Tach",
            Rhythm::SinusBradycardia => "Sinus Brady",
            Rhythm::PVCs => "PVCs",
            Rhythm::VentricularTachycardia => "VT",
            Rhythm::VentricularFibrillation => "VF",
            Rhythm::Asystole => "ASYSTOLE",
        };

        let chest_pain = self.get_chest_pain_level();
        let troponin = self.get_troponin_level();

        format!(
            "Heart: {} - HR={:.0} bpm, EF={:.0}%, BP={:.0}/{:.0} mmHg, Chest pain={:.1}/10, Troponin={:.2} ng/mL",
            rhythm_str,
            self.heart_rate_bpm,
            self.ejection_fraction_percent,
            self.aortic_pressure_systolic,
            self.aortic_pressure_diastolic,
            chest_pain,
            troponin
        )
    }

    fn get_id(&self) -> OrganId {
        self.id
    }

    fn get_type(&self) -> &'static str {
        "Heart"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
