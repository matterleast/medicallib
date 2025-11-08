//! Medical simulation example
//!
//! Demonstrates the medical library with a simulated patient

use medicallib::*;
use medicallib::organs::*;
use std::{thread, time::Duration};

fn main() {
    // Initialize a new patient with a 12-lead heart
    let mut patient = initialize_patient(1, 12);
    println!("Patient created with ID: {}", patient.id);

    // Introduce a toxin load to the blood for the liver to clear
    patient.blood.toxin_level_au = 100.0;
    println!("Initial toxin load of 100.0 a.u. introduced.\n");

    // Simulate time passing and print a live summary
    let simulation_time_s = 60.0; // Run for a longer time to see effects
    let delta_time_s = 0.1;
    let num_steps = (simulation_time_s / delta_time_s) as i32;

    // Get mutable access to organs we want to interact with
    if let Some(stomach) = patient.get_organ_mut::<stomach::Stomach>("Stomach") {
        stomach.add_substance(300.0, 7.0); // Simulate eating a meal (300mL, pH 7.0)
        println!("A 300mL meal has been consumed.");
    }

    println!("\n--- Simulating {} seconds... ---", simulation_time_s);

    for i in 0..num_steps {
        let current_time = i as f64 * delta_time_s;

        // Clear console on Unix systems
        #[cfg(unix)]
        print!("\x1B[2J\x1B[1;1H");

        // --- Event scripting ---
        if (current_time - 20.0).abs() < delta_time_s / 2.0 {
            if let Some(lungs) = patient.get_organ_mut::<lungs::Lungs>("Lungs") {
                println!("\n*** LUNG INJURY EVENT ***\n");
                lungs.inflict_damage(0, 0.8); // Damage lobe 0 by 80%
            }
        }

        update_patient(&mut patient, delta_time_s);

        println!("Time: {:.1}s / {}s\n", current_time, simulation_time_s);
        println!("--- Blood Chemistry ---");
        println!("SpO2: {:.1} %", patient.blood.oxygen_saturation_percent);
        println!("PaCO2: {:.1} mmHg", patient.blood.paco2_mmhg);
        println!("Glucose: {:.1} mg/dL", patient.blood.blood_glucose_mg_dl);
        println!("Toxins: {:.1} a.u.\n", patient.blood.toxin_level_au);

        // Print organ summaries
        for organ in patient.organs() {
            println!("{}", organ.get_summary());
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("\n--- Simulation Complete. Final State: ---\n");
    println!("{}", get_patient_summary(&patient));
}
