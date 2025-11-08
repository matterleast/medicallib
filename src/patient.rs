//! Patient management and blood composition

use crate::blood::BloodComposition;
use crate::organ::Organ;
use crate::organs::*;
use std::collections::HashMap;

/// Patient structure containing all organ systems
pub struct Patient {
    /// Unique patient identifier
    pub id: i32,
    /// Blood composition and vital signs
    pub blood: BloodComposition,
    /// All organs in the patient
    organs: Vec<Box<dyn Organ>>,
    /// Organ lookup by type name
    organ_map: HashMap<&'static str, usize>,
}

impl Patient {
    /// Get a reference to an organ by type
    pub fn get_organ<T: 'static>(&self, type_name: &'static str) -> Option<&T> {
        self.organ_map.get(type_name).and_then(|&idx| {
            self.organs.get(idx).and_then(|organ| {
                organ.as_any().downcast_ref::<T>()
            })
        })
    }

    /// Get a mutable reference to an organ by type
    pub fn get_organ_mut<T: 'static>(&mut self, type_name: &'static str) -> Option<&mut T> {
        self.organ_map.get(type_name).and_then(|&idx| {
            self.organs.get_mut(idx).and_then(|organ| {
                organ.as_any_mut().downcast_mut::<T>()
            })
        })
    }

    /// Get all organs
    pub fn organs(&self) -> &[Box<dyn Organ>] {
        &self.organs
    }

    /// Get all organs mutably
    pub fn organs_mut(&mut self) -> &mut [Box<dyn Organ>] {
        &mut self.organs
    }
}

/// Initialize a new patient with all organ systems
///
/// # Arguments
/// * `patient_id` - Unique identifier for the patient
/// * `num_heart_leads` - Number of EKG leads (3, 5, or 12)
///
/// # Returns
/// A fully initialized Patient
pub fn initialize_patient(patient_id: i32, num_heart_leads: usize) -> Patient {
    let mut organs: Vec<Box<dyn Organ>> = Vec::new();
    let mut organ_map = HashMap::new();

    // Create all organs
    let heart = Box::new(heart::Heart::new(0, num_heart_leads));
    organ_map.insert("Heart", organs.len());
    organs.push(heart);

    let lungs = Box::new(lungs::Lungs::new(1));
    organ_map.insert("Lungs", organs.len());
    organs.push(lungs);

    let brain = Box::new(brain::Brain::new(2));
    organ_map.insert("Brain", organs.len());
    organs.push(brain);

    let spinal_cord = Box::new(spinal_cord::SpinalCord::new(3));
    organ_map.insert("SpinalCord", organs.len());
    organs.push(spinal_cord);

    let stomach = Box::new(stomach::Stomach::new(4));
    organ_map.insert("Stomach", organs.len());
    organs.push(stomach);

    let esophagus = Box::new(esophagus::Esophagus::new(5));
    organ_map.insert("Esophagus", organs.len());
    organs.push(esophagus);

    let intestines = Box::new(intestines::Intestines::new(6));
    organ_map.insert("Intestines", organs.len());
    organs.push(intestines);

    let pancreas = Box::new(pancreas::Pancreas::new(7));
    organ_map.insert("Pancreas", organs.len());
    organs.push(pancreas);

    let liver = Box::new(liver::Liver::new(8));
    organ_map.insert("Liver", organs.len());
    organs.push(liver);

    let gallbladder = Box::new(gallbladder::Gallbladder::new(9));
    organ_map.insert("Gallbladder", organs.len());
    organs.push(gallbladder);

    let kidneys = Box::new(kidneys::Kidneys::new(10));
    organ_map.insert("Kidneys", organs.len());
    organs.push(kidneys);

    let bladder = Box::new(bladder::Bladder::new(11));
    organ_map.insert("Bladder", organs.len());
    organs.push(bladder);

    let spleen = Box::new(spleen::Spleen::new(12));
    organ_map.insert("Spleen", organs.len());
    organs.push(spleen);

    Patient {
        id: patient_id,
        blood: BloodComposition::default(),
        organs,
        organ_map,
    }
}

/// Update patient state and all organ systems
///
/// # Arguments
/// * `patient` - Mutable reference to the patient
/// * `delta_time_s` - Time step in seconds
pub fn update_patient(patient: &mut Patient, delta_time_s: f64) {
    // Update all organs
    for i in 0..patient.organs.len() {
        // Split the borrows to allow organ to access patient
        let (_left, right) = patient.organs.split_at_mut(i);
        if let Some((organ, _)) = right.split_first_mut() {
            let mut temp_patient = Patient {
                id: patient.id,
                blood: patient.blood.clone(),
                organs: Vec::new(),
                organ_map: HashMap::new(),
            };
            organ.update(&mut temp_patient, delta_time_s);
            patient.blood = temp_patient.blood;
        }
    }

    // Simulate RAAS (Renin-Angiotensin-Aldosterone System)
    // Kidneys produce renin, liver produces angiotensinogen
    let renin_secretion = if let Some(kidneys) = patient.get_organ::<kidneys::Kidneys>("Kidneys") {
        kidneys.get_renin_secretion()
    } else {
        0.0
    };

    let angiotensinogen = if let Some(liver) = patient.get_organ::<liver::Liver>("Liver") {
        liver.get_angiotensinogen()
    } else {
        0.0
    };

    // Angiotensin II production
    let angiotensin_production = renin_secretion * angiotensinogen * 0.1;
    patient.blood.chemistry.angiotensin_ii_au += angiotensin_production * delta_time_s;

    // Angiotensin II decay
    patient.blood.chemistry.angiotensin_ii_au *= 0.95_f64.powf(delta_time_s);
}

/// Get a summary of all patient vitals
pub fn get_patient_summary(patient: &Patient) -> String {
    format!(
        "Patient {} - SpO2: {:.1}%, PaCO2: {:.1} mmHg, Glucose: {:.1} mg/dL, Toxins: {:.1} AU",
        patient.id,
        patient.blood.gases.sao2_percent,
        patient.blood.gases.paco2_mmhg,
        patient.blood.chemistry.glucose_mg_dl,
        patient.blood.chemistry.toxin_level_au
    )
}
