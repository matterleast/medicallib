//! Quick test to verify troponin timing is correct

use medicallib::*;
use medicallib::organs::*;

fn main() {
    let mut patient = initialize_patient(1, 12);

    // Induce severe LAD stenosis
    if let Some(vascular) = patient.get_organ_mut::<vascular::VascularSystem>("VascularSystem") {
        // First add baseline plaque
        vascular.add_plaque("LAD", 0.4);  // 40% baseline stenosis
        // Then rupture it to cause acute occlusion
        vascular.rupture_plaque("LAD");

        // Verify stenosis was applied
        if let Some(lad) = vascular.get_vessel("LAD") {
            println!("LAD Stenosis: {:.1}%", lad.plaque_buildup * 100.0);
            println!("LAD Flow: {:.2} mL/min\n", lad.blood_flow_rate_ml_per_min);
        }
    }

    println!("Testing troponin release timing...\n");
    println!("Time     | Troponin  | Tissue State | Total Ischemia");
    println!("---------|-----------|--------------|---------------");

    // Simulate for 6 hours to see troponin progression
    for i in 0..360 {
        let time_min = (i + 1) * 10;

        // Update patient (10 minutes at a time)
        for _ in 0..60 {
            update_patient(&mut patient, 10.0);
        }

        if let Some(heart) = patient.get_organ::<heart::Heart>("Heart") {
            let troponin = heart.get_troponin_level();
            let anterior = &heart.myocardial_segments[0];

            // Only print every hour + key milestones
            if time_min % 60 == 0 || time_min == 20 || time_min == 180 || time_min == 200 {
                let state_str = format!("{:?}", anterior.cellular_state).split('{').next().unwrap().trim().to_string();
                println!("{:>4} min | {:>8.3}  | {:<12} | {:>6.1}h | Flow: {:.2} mL/min",
                    time_min,
                    troponin,
                    state_str,
                    anterior.total_ischemia_time_s / 3600.0,
                    anterior.blood_flow_ml_per_min
                );
            }
        }
    }

    println!("\n=== TROPONIN TIMELINE ===");
    println!("Expected: Rises at ~3h, peaks at 24-48h");
    println!("Normal: <0.04 ng/mL");
    println!("Positive MI: >0.04 ng/mL");
}
