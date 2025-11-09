//! Debug test for coronary stenosis flow calculation
//!
//! This test verifies that:
//! 1. Coronary stenosis increases resistance
//! 2. Increased resistance reduces flow
//! 3. Reduced flow causes myocardial ischemia

use medicallib::*;
use medicallib::organs::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         CORONARY STENOSIS FLOW DEBUG TEST                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut patient = initialize_patient(1, 12);

    println!("=== STEP 1: BASELINE (No Stenosis) ===\n");

    // Run a few updates to stabilize
    for _ in 0..10 {
        update_patient(&mut patient, 0.1);
    }

    // Check baseline coronary flow
    if let Some(vascular) = patient.get_organ::<vascular::VascularSystem>("VascularSystem") {
        if let Some(lad) = vascular.get_vessel("LAD") {
            println!("LAD Vessel:");
            println!("  Diameter: {:.2} mm (baseline: {:.2} mm)", lad.diameter_mm, lad.baseline_diameter_mm);
            println!("  Effective diameter: {:.2} mm", lad.effective_diameter());
            println!("  Plaque buildup: {:.1}%", lad.plaque_buildup * 100.0);
            println!("  Resistance: {:.6}", lad.flow_resistance());
            println!("  Blood flow: {:.2} mL/min", lad.blood_flow_rate_ml_per_min);
        }
    }

    if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
        println!("\nMyocardial Segments:");
        for segment in &heart.myocardial_segments {
            if matches!(segment.region, myocardial_tissue::MyocardialRegion::Anterior | myocardial_tissue::MyocardialRegion::Septal) {
                println!("  {:?}:", segment.region);
                println!("    Blood flow: {:.2} mL/min (baseline: {:.2})",
                    segment.blood_flow_ml_per_min,
                    segment.baseline_flow_ml_per_min);
                println!("    O2 delivery: {:.2} mL/min", segment.oxygen_delivery_ml_per_min);
                println!("    O2 consumption: {:.2} mL/min", segment.oxygen_consumption_ml_per_min);
                println!("    State: {:?}", segment.cellular_state);
            }
        }
    }

    println!("\n=== STEP 2: ADD 40% STENOSIS ===\n");

    if let Some(vascular) = patient.get_organ_mut::<vascular::VascularSystem>("VascularSystem") {
        vascular.add_plaque("LAD", 0.4);
    }

    // Update to recalculate flow
    for _ in 0..5 {
        update_patient(&mut patient, 0.1);
    }

    if let Some(vascular) = patient.get_organ::<vascular::VascularSystem>("VascularSystem") {
        if let Some(lad) = vascular.get_vessel("LAD") {
            println!("LAD Vessel:");
            println!("  Diameter: {:.2} mm (baseline: {:.2} mm)", lad.diameter_mm, lad.baseline_diameter_mm);
            println!("  Effective diameter: {:.2} mm", lad.effective_diameter());
            println!("  Plaque buildup: {:.1}%", lad.plaque_buildup * 100.0);
            println!("  Resistance: {:.6} (expected: ~10× baseline)", lad.flow_resistance());
            println!("  Blood flow: {:.2} mL/min (expected: ~10% of baseline)", lad.blood_flow_rate_ml_per_min);
        }
    }

    if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
        println!("\nMyocardial Segments:");
        for segment in &heart.myocardial_segments {
            if matches!(segment.region, myocardial_tissue::MyocardialRegion::Anterior | myocardial_tissue::MyocardialRegion::Septal) {
                println!("  {:?}:", segment.region);
                println!("    Blood flow: {:.2} mL/min (baseline: {:.2})",
                    segment.blood_flow_ml_per_min,
                    segment.baseline_flow_ml_per_min);
                println!("    O2 delivery: {:.2} mL/min", segment.oxygen_delivery_ml_per_min);
                println!("    O2 consumption: {:.2} mL/min", segment.oxygen_consumption_ml_per_min);
                let ratio = segment.oxygen_delivery_ml_per_min / segment.oxygen_consumption_ml_per_min;
                println!("    Supply/Demand ratio: {:.2} (< 1.0 = ischemia)", ratio);
                println!("    State: {:?}", segment.cellular_state);
            }
        }
    }

    println!("\n=== STEP 3: RUPTURE PLAQUE (90% Stenosis) ===\n");

    if let Some(vascular) = patient.get_organ_mut::<vascular::VascularSystem>("VascularSystem") {
        vascular.rupture_plaque("LAD");
    }

    // Update to recalculate flow
    for _ in 0..5 {
        update_patient(&mut patient, 0.1);
    }

    if let Some(vascular) = patient.get_organ::<vascular::VascularSystem>("VascularSystem") {
        if let Some(lad) = vascular.get_vessel("LAD") {
            println!("LAD Vessel:");
            println!("  Diameter: {:.2} mm (baseline: {:.2} mm)", lad.diameter_mm, lad.baseline_diameter_mm);
            println!("  Effective diameter: {:.2} mm", lad.effective_diameter());
            println!("  Plaque buildup: {:.1}%", lad.plaque_buildup * 100.0);
            println!("  Resistance: {:.6} (expected: ~260× baseline)", lad.flow_resistance());
            println!("  Blood flow: {:.2} mL/min (expected: near 0)", lad.blood_flow_rate_ml_per_min);
        }

        // Check all arteries to see flow distribution
        println!("\nAll arterial flows:");
        let total_conductance: f64 = vascular.vessels
            .iter()
            .filter(|v| matches!(v.vessel_type, vascular::VesselType::Artery))
            .map(|v| {
                let r = v.flow_resistance();
                if r > 0.0 { 1.0 / r } else { 0.0 }
            })
            .sum();

        println!("  Total arterial conductance: {:.6}", total_conductance);

        for vessel in &vascular.vessels {
            if matches!(vessel.vessel_type, vascular::VesselType::Artery) {
                let r = vessel.flow_resistance();
                let conductance = if r > 0.0 { 1.0 / r } else { 0.0 };
                let fraction = if total_conductance > 0.0 { conductance / total_conductance } else { 0.0 };
                println!("  {}: R={:.2}, G={:.6}, Fraction={:.4}, Flow={:.2} mL/min",
                    vessel.name,
                    r,
                    conductance,
                    fraction,
                    vessel.blood_flow_rate_ml_per_min
                );
            }
        }
    }

    if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
        println!("\nMyocardial Segments (should be ISCHEMIC now):");
        for segment in &heart.myocardial_segments {
            if matches!(segment.region, myocardial_tissue::MyocardialRegion::Anterior | myocardial_tissue::MyocardialRegion::Septal) {
                println!("  {:?}:", segment.region);
                println!("    Blood flow: {:.2} mL/min (baseline: {:.2})",
                    segment.blood_flow_ml_per_min,
                    segment.baseline_flow_ml_per_min);
                println!("    O2 delivery: {:.2} mL/min", segment.oxygen_delivery_ml_per_min);
                println!("    O2 consumption: {:.2} mL/min", segment.oxygen_consumption_ml_per_min);
                let ratio = segment.oxygen_delivery_ml_per_min / segment.oxygen_consumption_ml_per_min;
                println!("    Supply/Demand ratio: {:.2} (< 1.0 = ischemia)", ratio);
                println!("    State: {:?}", segment.cellular_state);

                if ratio < 1.0 {
                    println!("    ✓ ISCHEMIA DETECTED");
                } else {
                    println!("    ✗ ERROR: Should be ischemic but isn't!");
                }
            }
        }
    }

    println!("\n=== STEP 4: RUN FOR 60 SECONDS TO SEE PROGRESSION ===\n");

    for i in 0..600 {
        update_patient(&mut patient, 0.1);

        if i % 100 == 99 {
            let time_s = (i + 1) as f64 * 0.1;
            println!("\n--- Time: {:.0}s ---", time_s);

            if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
                let total_chest_pain = heart.get_chest_pain_level();
                println!("  Heart:");
                println!("    Heart rate: {:.0} bpm", heart.heart_rate_bpm);
                println!("    Rhythm: {:?}", heart.rhythm);
                println!("    Chest pain: {:.1}/10", total_chest_pain);

                for segment in &heart.myocardial_segments {
                    if matches!(segment.region, myocardial_tissue::MyocardialRegion::Anterior) {
                        println!("  Anterior segment:");
                        println!("    Flow: {:.2} mL/min", segment.blood_flow_ml_per_min);
                        println!("    State: {:?}", segment.cellular_state);
                    }
                }
            }
        }
    }

    println!("\n=== FINAL ASSESSMENT ===\n");

    if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
        let mut test_passed = true;

        println!("Expected outcomes:");
        println!("  ✓ LAD should have near-zero flow");
        println!("  ✓ Anterior/Septal segments should be ischemic or injured");
        println!("  ✓ Chest pain should be present");
        println!("  ✓ Arrhythmias may have developed\n");

        println!("Actual outcomes:");

        for segment in &heart.myocardial_segments {
            if matches!(segment.region, myocardial_tissue::MyocardialRegion::Anterior | myocardial_tissue::MyocardialRegion::Septal) {
                println!("  {:?}: {:?}", segment.region, segment.cellular_state);

                if matches!(segment.cellular_state, myocardial_tissue::CellularState::Healthy) {
                    println!("    ✗ FAIL: Should not be healthy with 90% LAD stenosis!");
                    test_passed = false;
                } else {
                    println!("    ✓ PASS: Tissue shows pathology");
                }
            }
        }

        println!("  Rhythm: {:?}", heart.rhythm);
        println!("  Chest pain: {:.1}/10", heart.get_chest_pain_level());

        if test_passed {
            println!("\n✓✓✓ TEST PASSED: Emergent pathophysiology working! ✓✓✓");
        } else {
            println!("\n✗✗✗ TEST FAILED: Stenosis not affecting tissue! ✗✗✗");
        }
    }
}
