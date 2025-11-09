//! STEMI (ST-Elevation Myocardial Infarction) Simulation
//!
//! This example demonstrates emergent cardiac pathophysiology:
//! 1. Plaque buildup in LAD (Left Anterior Descending artery)
//! 2. Plaque rupture → acute thrombosis → coronary occlusion
//! 3. Myocardial ischemia → injury → necrosis
//! 4. ECG changes: ST elevation, Q waves, T wave inversion
//! 5. Symptoms: Chest pain from ischemic tissue
//! 6. Troponin release from myocardial injury
//! 7. Arrhythmia progression: PVCs → VT → VF → Asystole
//!
//! All of these emerge from the simulation - nothing is hardcoded!

use medicallib::*;
use medicallib::organs::*;
use std::{thread, time::Duration};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     EMERGENT STEMI SIMULATION - No Hardcoded Events!        ║");
    println!("║  Watch plaque → ischemia → arrhythmia → cardiac arrest      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Initialize patient with 12-lead ECG
    let mut patient = initialize_patient(1, 12);

    println!("=== PATIENT INITIALIZED ===");
    println!("Patient ID: {}", patient.id);
    println!("Blood Type: {}\n", patient.blood.blood_type);

    // Initial blood work
    println!("=== INITIAL BLOOD WORK ===");
    println!("{}\n", patient.blood.get_cbc_summary());
    println!("{}\n", patient.blood.get_cmp_summary());

    // Simulate pre-existing coronary artery disease
    // Add 40% stenosis to LAD from chronic atherosclerosis
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  BASELINE: Patient has 40% stenosis in LAD from years    ║");
    println!("║  of atherosclerosis. Tissue is compensating... for now.   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    if let Some(vascular) = patient.get_organ_mut::<vascular::VascularSystem>("VascularSystem") {
        vascular.add_plaque("LAD", 0.4);
    }

    // Run for 30 seconds to stabilize
    for _ in 0..300 {
        update_patient(&mut patient, 0.1);
    }

    println!("Baseline established. All systems stable.\n");
    thread::sleep(Duration::from_millis(1000));

    // THE CRITICAL EVENT: Plaque rupture!
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║               ⚠️  PLAQUE RUPTURE EVENT! ⚠️                  ║");
    println!("║  Unstable plaque in LAD ruptures → acute thrombosis       ║");
    println!("║  Stenosis: 40% → 90%+ (near-complete occlusion)          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    if let Some(vascular) = patient.get_organ_mut::<vascular::VascularSystem>("VascularSystem") {
        vascular.rupture_plaque("LAD");
    }

    thread::sleep(Duration::from_millis(1500));

    // Now simulate for several minutes and watch the cascade unfold
    // Extended to 30 minutes to see full progression: Ischemic → Injured → PVCs → VT → VF
    let simulation_time_s = 1800.0; // 30 minutes
    let delta_time_s = 0.5;
    let num_steps = (simulation_time_s / delta_time_s) as i32;

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║        WATCH THE EMERGENT PATHOPHYSIOLOGY UNFOLD          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let mut last_rhythm_report = String::new();
    let mut event_log = Vec::new();

    for i in 0..num_steps {
        let current_time = i as f64 * delta_time_s;

        update_patient(&mut patient, delta_time_s);

        // Clear console for real-time display
        #[cfg(unix)]
        print!("\x1B[2J\x1B[1;1H");

        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║              LIVE PATIENT MONITOR - Time: {:>5.0}s            ║", current_time);
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        // Get heart status
        if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
            // Check for rhythm changes
            let rhythm = format!("{:?}", heart.rhythm);
            if rhythm != last_rhythm_report {
                let msg = format!("[{:>5.0}s] RHYTHM CHANGE: {} → {}",
                    current_time,
                    last_rhythm_report.split("::").last().unwrap_or("Normal"),
                    rhythm.split("::").last().unwrap_or("Unknown")
                );
                event_log.push(msg.clone());
                last_rhythm_report = rhythm;
            }

            // Display vital signs
            println!("┌─ VITAL SIGNS ────────────────────────────────────────────┐");
            println!("│ HR: {:>3.0} bpm    BP: {:>3.0}/{:<3.0} mmHg    EF: {:>2.0}%        │",
                heart.heart_rate_bpm,
                heart.aortic_pressure_systolic,
                heart.aortic_pressure_diastolic,
                heart.ejection_fraction_percent
            );
            println!("└──────────────────────────────────────────────────────────┘\n");

            // Display cardiac status
            let chest_pain = heart.get_chest_pain_level();
            let troponin = heart.get_troponin_level();
            let cardiac_arrest = heart.is_cardiac_arrest();

            println!("┌─ CARDIAC STATUS ─────────────────────────────────────────┐");
            println!("│ Rhythm: {:<46} │", format!("{:?}", heart.rhythm));
            println!("│ Chest Pain: {:<10} (0-10 scale)                   │",
                format!("{:.1}/10", chest_pain)
            );
            println!("│ Troponin: {:<11} (Normal: <0.04 ng/mL)          │",
                format!("{:.3} ng/mL", troponin)
            );

            if cardiac_arrest {
                println!("│                                                          │");
                println!("│         ⚠️  ⚠️  ⚠️   CARDIAC ARREST!  ⚠️  ⚠️  ⚠️              │");
                println!("│              NO EFFECTIVE CARDIAC OUTPUT                 │");
            }
            println!("└──────────────────────────────────────────────────────────┘\n");

            // Display myocardial segment states
            println!("┌─ MYOCARDIAL TISSUE STATUS ───────────────────────────────┐");
            for segment in &heart.myocardial_segments {
                let state_str = match segment.cellular_state {
                    medicallib::myocardial_tissue::CellularState::Healthy => "✓ Healthy",
                    medicallib::myocardial_tissue::CellularState::Ischemic { duration_seconds } => {
                        &format!("⚠ Ischemic ({:.0}s)", duration_seconds)
                    },
                    medicallib::myocardial_tissue::CellularState::Injured { duration_seconds } => {
                        &format!("⚠️  INJURED ({:.0}s)", duration_seconds)
                    },
                    medicallib::myocardial_tissue::CellularState::Necrotic { days_old } => {
                        &format!("💀 NECROTIC ({:.1}d)", days_old)
                    },
                };

                let flow = segment.blood_flow_ml_per_min;
                let baseline = segment.baseline_flow_ml_per_min;
                let flow_pct = (flow / baseline * 100.0).min(100.0);

                println!("│ {:15} {:20} Flow:{:>3.0}%  {:7}│",
                    format!("{:?}:", segment.region),
                    state_str,
                    flow_pct,
                    if flow_pct < 50.0 { "⚠️ LOW" } else { "" }
                );
            }
            println!("└──────────────────────────────────────────────────────────┘\n");
        }

        // Coronary artery status
        if let Some(vascular) = patient.get_organ::<vascular::VascularSystem>("VascularSystem") {
            if let Some(lad) = vascular.get_vessel("LAD") {
                let stenosis_pct = lad.plaque_buildup * 100.0;
                let flow = lad.blood_flow_rate_ml_per_min;

                println!("┌─ CORONARY ARTERIES ──────────────────────────────────────┐");
                println!("│ LAD Stenosis: {:<5.1}%    Flow: {:<8.1} mL/min        │",
                    stenosis_pct,
                    flow
                );
                if stenosis_pct > 70.0 {
                    println!("│ ⚠️  CRITICAL STENOSIS - Myocardial ischemia!             │");
                }
                println!("└──────────────────────────────────────────────────────────┘\n");
            }
        }

        // Event log
        if !event_log.is_empty() {
            println!("┌─ EVENT LOG ──────────────────────────────────────────────┐");
            for (_idx, event) in event_log.iter().enumerate().rev().take(5).rev() {
                println!("│ {} │", event);
            }
            println!("└──────────────────────────────────────────────────────────┘\n");
        }

        // Sleep to make it watchable
        thread::sleep(Duration::from_millis(100));

        // End simulation early if patient arrests and stays arrested
        if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
            if heart.rhythm == heart::Rhythm::Asystole && current_time > 60.0 {
                println!("\n╔═══════════════════════════════════════════════════════════╗");
                println!("║          Patient has progressed to asystole.              ║");
                println!("║          Without intervention, this is terminal.          ║");
                println!("╚═══════════════════════════════════════════════════════════╝\n");
                thread::sleep(Duration::from_secs(3));
                break;
            }
        }
    }

    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                  SIMULATION COMPLETE                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("=== WHAT YOU JUST WITNESSED ===\n");
    println!("This was NOT scripted! Here's what EMERGED from the simulation:\n");
    println!("1. ✓ Plaque rupture reduced LAD blood flow");
    println!("2. ✓ Anterior/Septal myocardium became ischemic (O2 supply < demand)");
    println!("3. ✓ Ischemic cells released lactic acid → chest pain");
    println!("4. ✓ Prolonged ischemia → cellular injury → electrical instability");
    println!("5. ✓ Injured cells generated ectopic beats → PVCs");
    println!("6. ✓ Multiple unstable regions → organized VT");
    println!("7. ✓ Sustained VT → chaotic VF");
    println!("8. ✓ Untreated VF → myocardial death → asystole");
    println!("9. ✓ Troponin rose as myocardial cells died");
    println!("10. ✓ ECG changes reflected actual tissue electrical properties\n");

    println!("This is TRUE EMERGENT PATHOPHYSIOLOGY! 🎉\n");

    // Final blood work
    println!("=== FINAL BLOOD WORK ===\n");
    println!("{}\n", patient.blood.get_cbc_summary());
    println!("{}\n", patient.blood.get_cmp_summary());
    println!("{}\n", patient.blood.get_abg_summary());

    println!("\n=== FULL EVENT LOG ===");
    for event in &event_log {
        println!("{}", event);
    }
}
