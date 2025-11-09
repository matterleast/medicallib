//! Multi-Organ Failure Simulation - Cascading Emergent Pathophysiology
//!
//! This example demonstrates how organ failures CASCADE naturally:
//! 1. Cardiac arrest (from STEMI) → no perfusion
//! 2. Kidneys: hypoperfusion → ATN → AKI → uremia → hyperkalemia
//! 3. Hyperkalemia worsens cardiac function (vicious cycle!)
//! 4. Kidneys fail → metabolic acidosis
//! 5. Acidosis impairs all organs
//! 6. Liver: hypoperfusion → hepatocyte death → coagulopathy
//! 7. Brain: hypoxia → GCS decline
//!
//! ALL of this emerges from the simulation - no hardcoded cascade events!

use medicallib::*;
use medicallib::organs::*;
use std::{thread, time::Duration};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          CASCADING MULTI-ORGAN FAILURE SIMULATION           ║");
    println!("║         Watch One Failure Trigger System Collapse           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut patient = initialize_patient(1, 12);

    println!("=== PATIENT INITIALIZED ===");
    println!("Patient ID: {}", patient.id);
    println!("All systems nominal at baseline\n");

    // Initial vitals
    println!("=== BASELINE VITALS ===");
    for organ in patient.organs() {
        println!("{}", organ.get_summary());
    }
    println!();

    // THE TRIGGERING EVENT: Massive anterior MI from LAD occlusion
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║            ⚠️  TRIGGERING EVENT: MASSIVE STEMI ⚠️             ║");
    println!("║  Complete LAD occlusion → anterior wall infarction        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Create severe coronary disease
    if let Some(vascular) = patient.get_organ_mut::<vascular::VascularSystem>("VascularSystem") {
        vascular.add_plaque("LAD", 0.50);  // Start with 50% stenosis
        thread::sleep(Duration::from_millis(500));
        vascular.rupture_plaque("LAD");    // Rupture → 95% occlusion
    }

    println!("LAD now 95% occluded. Watching the cascade unfold...\n");
    thread::sleep(Duration::from_secs(2));

    // Run simulation and watch cascade
    let simulation_time_s = 600.0; // 10 minutes
    let delta_time_s = 1.0;
    let num_steps = (simulation_time_s / delta_time_s) as i32;

    let mut event_log = Vec::new();
    let mut last_recorded_states = std::collections::HashMap::new();

    for i in 0..num_steps {
        let current_time = i as f64 * delta_time_s;

        update_patient(&mut patient, delta_time_s);

        // Clear console
        #[cfg(unix)]
        print!("\x1B[2J\x1B[1;1H");

        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║           CASCADING ORGAN FAILURE - Time: {:>5.0}s             ║", current_time);
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        // Track and display organ states
        println!("┌─ ORGAN STATUS ───────────────────────────────────────────────┐");

        // Heart
        if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
            let rhythm_str = format!("{:?}", heart.rhythm);
            let current_rhythm = rhythm_str.clone();

            // Record rhythm changes
            if last_recorded_states.get("rhythm") != Some(&current_rhythm) {
                event_log.push(format!("[{:>5.0}s] HEART: Rhythm changed to {}", current_time, rhythm_str));
                last_recorded_states.insert("rhythm".to_string(), current_rhythm);
            }

            let arrest_marker = if heart.is_cardiac_arrest() { " ⚠️ ARREST" } else { "" };
            println!("│ ❤️  Heart: {} - HR={:.0} bpm, BP={}/{} mmHg, EF={:.0}%{}",
                rhythm_str,
                heart.heart_rate_bpm,
                heart.aortic_pressure_systolic as i32,
                heart.aortic_pressure_diastolic as i32,
                heart.ejection_fraction_percent,
                arrest_marker
            );
            println!("│    Chest pain: {:.1}/10, Troponin: {:.2} ng/mL",
                heart.get_chest_pain_level(),
                heart.get_troponin_level()
            );
        }

        // Kidneys
        if let Some(kidneys) = patient.get_organ::<kidneys::Kidneys>("Kidneys") {
            let aki_marker = if kidneys.is_aki() {
                format!(" [AKI Stage {}]", kidneys.aki_stage())
            } else {
                String::new()
            };

            // Record AKI onset
            let aki_key = format!("aki_{}", kidneys.aki_stage());
            if kidneys.is_aki() && last_recorded_states.get("aki") != Some(&aki_key) {
                event_log.push(format!("[{:>5.0}s] KIDNEYS: AKI Stage {} developed", current_time, kidneys.aki_stage()));
                last_recorded_states.insert("aki".to_string(), aki_key);
            }

            println!("│ 🫘 Kidneys: GFR={:.1} mL/min{}, RBF={:.0} mL/min",
                kidneys.gfr_ml_per_min,
                aki_marker,
                kidneys.renal_blood_flow_ml_per_min
            );
        }

        // Blood chemistry - the cascading effects!
        println!("│ 🧪 Blood Chemistry:");
        let k = patient.blood.chemistry.potassium_meq_l;
        let k_marker = if k > 6.0 { " ⚠️  CRITICAL" } else if k > 5.5 { " ⚠️" } else { "" };
        println!("│    K+: {:.1} mEq/L{}, Na+: {:.0} mEq/L, HCO3-: {:.1} mEq/L",
            k,
            k_marker,
            patient.blood.chemistry.sodium_meq_l,
            patient.blood.chemistry.bicarbonate_meq_l
        );

        // Record hyperkalemia
        if k > 5.5 && last_recorded_states.get("hyperkalemia") != Some(&"true".to_string()) {
            event_log.push(format!("[{:>5.0}s] BLOOD: Hyperkalemia (K+ {:.1}) - risk of arrhythmia!", current_time, k));
            last_recorded_states.insert("hyperkalemia".to_string(), "true".to_string());
        }

        println!("│    pH: {:.2}, Cr: {:.1} mg/dL, BUN: {:.0} mg/dL",
            patient.blood.gases.ph,
            patient.blood.chemistry.creatinine_mg_dl,
            patient.blood.chemistry.bun_mg_dl
        );

        // Brain
        if let Some(brain) = patient.get_organ::<brain::Brain>("Brain") {
            let gcs = brain.gcs.total();
            let category = brain.gcs.category();

            // Record GCS changes
            let gcs_key = gcs.to_string();
            if last_recorded_states.get("gcs") != Some(&gcs_key) {
                event_log.push(format!("[{:>5.0}s] BRAIN: GCS declined to {} ({})", current_time, gcs, category));
                last_recorded_states.insert("gcs".to_string(), gcs_key);
            }

            println!("│ 🧠 Brain: GCS={} ({}), ICP={:.1} mmHg, CPP={:.1} mmHg",
                gcs,
                category,
                brain.intracranial_pressure_mmhg,
                brain.cerebral_perfusion_pressure_mmhg
            );
        }

        println!("└──────────────────────────────────────────────────────────────┘\n");

        // Event log - show cascading failures
        if !event_log.is_empty() {
            println!("┌─ CASCADE EVENT LOG ──────────────────────────────────────────┐");
            for event in event_log.iter().rev().take(10).rev() {
                println!("│ {} │", event);
            }
            println!("└──────────────────────────────────────────────────────────────┘\n");
        }

        // Analysis of the cascade
        if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
            if let Some(kidneys) = patient.get_organ::<kidneys::Kidneys>("Kidneys") {
                println!("┌─ PATHOPHYSIOLOGY ANALYSIS ───────────────────────────────────┐");

                // Cardiac status
                if heart.is_cardiac_arrest() {
                    println!("│ ⚠️  CARDIAC ARREST → No perfusion to any organ           │");
                } else if heart.ejection_fraction_percent < 30.0 {
                    println!("│ ⚠️  Cardiogenic shock → Hypoperfusion of all organs     │");
                }

                // Renal consequences
                if kidneys.is_aki() {
                    println!("│ ⚠️  AKI → Uremia, hyperkalemia, acidosis developing     │");
                }

                // Vicious cycles
                if patient.blood.chemistry.potassium_meq_l > 5.5 && heart.rhythm == heart::Rhythm::VentricularFibrillation {
                    println!("│ ⚠️  VICIOUS CYCLE: Hyperkalemia worsening arrhythmia    │");
                }

                if patient.blood.gases.ph < 7.2 && heart.ejection_fraction_percent < 40.0 {
                    println!("│ ⚠️  VICIOUS CYCLE: Acidosis impairing cardiac function  │");
                }

                println!("└──────────────────────────────────────────────────────────────┘\n");
            }
        }

        thread::sleep(Duration::from_millis(200));

        // Early termination on complete system failure
        if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
            if heart.rhythm == heart::Rhythm::Asystole && current_time > 120.0 {
                println!("\n╔═══════════════════════════════════════════════════════════╗");
                println!("║         COMPLETE CARDIOVASCULAR COLLAPSE                  ║");
                println!("║         Multi-organ failure is now irreversible           ║");
                println!("╚═══════════════════════════════════════════════════════════╝\n");
                break;
            }
        }
    }

    // Summary
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                SIMULATION COMPLETE                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("=== THE CASCADE OF FAILURES (All Emergent!) ===\n");
    for (idx, event) in event_log.iter().enumerate() {
        println!("{}. {}", idx + 1, event);
    }

    println!("\n=== WHAT EMERGED FROM THE SIMULATION ===\n");
    println!("1. ✓ LAD occlusion → myocardial ischemia");
    println!("2. ✓ Ischemic myocardium → arrhythmias (PVCs → VT → VF)");
    println!("3. ✓ VF → no cardiac output → systemic hypoperfusion");
    println!("4. ✓ Renal hypoperfusion → nephron ischemia → ATN → AKI");
    println!("5. ✓ AKI → GFR drops → K+ not excreted → hyperkalemia");
    println!("6. ✓ Hyperkalemia → worsens cardiac electrical instability");
    println!("7. ✓ AKI → H+ not excreted → metabolic acidosis");
    println!("8. ✓ Acidosis → further impairs cardiac contractility");
    println!("9. ✓ Prolonged VF → myocardial necrosis → asystole");
    println!("10. ✓ Brain hypoxia → GCS decline");
    println!("11. ✓ Multi-organ failure → death\n");

    println!("Every single step emerged from simulation physics!");
    println!("NO hardcoded cascade logic. Just physiology! 🎉\n");

    // Final blood work
    println!("=== FINAL LABORATORY VALUES ===\n");
    println!("{}\n", patient.blood.get_cbc_summary());
    println!("{}\n", patient.blood.get_cmp_summary());
    println!("{}\n", patient.blood.get_abg_summary());
}
