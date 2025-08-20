#pragma once

#include "Organ.h"
#include <vector>
#include <string>
#include <deque>
#include <map>

/**
 * @brief Represents a specific region of the brain.
 */
struct BrainRegion {
    std::string name;                   ///< The name of the brain region.
    double activityLevel;               ///< Metabolic activity level, normalized [0.0, 1.0].
    double bloodFlow_ml_100g_min;       ///< Blood flow in mL per 100g of tissue per minute.
};

/**
 * @brief Represents the Brain organ, simulating neurological vitals and activity.
 *
 * @section bio_sec Biological Overview
 * The brain is the command center of the nervous system, controlling thought, memory, emotion,
 * touch, motor skills, vision, breathing, temperature, hunger, and every process that regulates
 * our body. It is a complex organ composed of several distinct regions, each with specialized
- * functions.
+ * functions. The major parts include:
 * - **Frontal Lobe**: Associated with reasoning, planning, parts of speech, movement, emotions, and problem-solving.
 * - **Parietal Lobe**: Manages perception of stimuli related to touch, pressure, temperature, and pain.
 * - **Temporal Lobe**: Involved in perception and recognition of auditory stimuli, memory, and speech.
 * - **Occipital Lobe**: Primarily responsible for vision.
 * - **Cerebellum**: Crucial for coordinating voluntary movements, posture, balance, and speech.
 *
 * Maintaining adequate blood flow is critical for brain health. Two key metrics are:
 * - **Intracranial Pressure (ICP)**: The pressure inside the skull.
 * - **Cerebral Perfusion Pressure (CPP)**: The net pressure gradient causing blood flow to the brain.
 *
 * @section model_sec Code Simulation
 * This `Brain` class models the brain's high-level physiological state and its influence on
 * the body's autonomic systems.
 *
 * @subsection neuro_vitals_sec Neurological Vitals
 * The simulation tracks several key neurological vitals:
 * - **Glasgow Coma Scale (GCS)**: A simplified score representing the level of consciousness,
 *   accessible via `getGCS()`.
 * - **Intracranial Pressure (ICP)**: A simulated pressure within the skull, retrieved with
 *   `getIntracranialPressure()`.
 * - **Cerebral Perfusion Pressure (CPP)**: Calculated based on ICP and Mean Arterial Pressure,
 *   available through `getCerebralPerfusionPressure()`.
 * - **EEG Waveform**: The class generates a simplified electroencephalogram (EEG) waveform,
 *   which can be accessed with `getEegWaveform()`.
 *
 * @subsection region_model_sec Regional Activity Model
 * The major lobes and the cerebellum are modeled as `BrainRegion` structs, each with its own
 * metabolic activity level and blood flow. The `update()` method adjusts these values over time.
 *
 * @subsection brain_diagram_sec Brain Regions Diagram
 * This diagram shows the major brain regions simulated by this class.
 *
 * @dot
 * digraph BrainRegions {
 *     node [shape=box, style=rounded];
 *
 *     Brain [label="Brain", shape=ellipse];
 *
 *     subgraph cluster_Cerebrum {
 *         label="Cerebrum (Lobes)";
 *         style=filled;
 *         color=lightblue;
 *         Frontal [label="Frontal Lobe"];
 *         Parietal [label="Parietal Lobe"];
 *         Temporal [label="Temporal Lobe"];
 *         Occipital [label="Occipital Lobe"];
 *     }
 *
 *     Cerebellum [label="Cerebellum", style=filled, color=lightpink];
 *
 *     Brain -> Frontal;
 *     Brain -> Parietal;
 *     Brain -> Temporal;
 *     Brain -> Occipital;
 *     Brain -> Cerebellum;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to create a `Brain` instance and monitor its state.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Brain.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Brain object
 *     Brain brain(1);
 *
 *     // Simulate for 5 seconds
 *     std::cout << "Simulating Brain for 5 seconds..." << std::endl;
 *     for (int i = 0; i < 5; ++i) {
 *         // In a real simulation, the patient's heart would update the MAP
 *         patient.setVital("MeanArterialPressure", 85.0); // Example MAP
 *         brain.update(patient, 1.0); // Update by 1.0 second
 *         std::cout << "Time: " << i + 1 << "s, " << brain.getSummary() << std::endl;
 *     }
 *
 *     // Retrieve specific final vitals
 *     std::cout << "\n--- Simulation Results ---" << std::endl;
 *     std::cout << "Final GCS: " << brain.getGCS() << std::endl;
 *     std::cout << "Final ICP: " << brain.getIntracranialPressure() << " mmHg" << std::endl;
 *     std::cout << "Final CPP: " << brain.getCerebralPerfusionPressure() << " mmHg" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Brain : public Organ {
public:
    /**
     * @brief Constructor for the Brain class.
     * @param id The ID of the organ.
     */
    Brain(int id);

    /**
     * @brief Updates the brain's state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the brain's vitals.
     * @return A string containing the brain's vital signs.
     */
    std::string getSummary() const override;

    // --- Getters for Key Neurological Vitals ---

    /** @brief Gets the Glasgow Coma Scale score (simplified). */
    int getGCS() const;

    /** @brief Gets the intracranial pressure in mmHg. */
    double getIntracranialPressure() const;

    /** @brief Gets the cerebral perfusion pressure in mmHg. */
    double getCerebralPerfusionPressure() const;

    /** @brief Gets the data for a simplified EEG waveform. */
    const std::deque<double>& getEegWaveform() const;

private:
    // --- Private Helper Methods ---
    void updateActivity(double deltaTime_s);
    void updatePressures(double meanArterialPressure);
    void updateAutonomicControl(Patient& patient, double deltaTime_s);
    double generateEegValue();

    // --- Physiological Parameters ---
    int gcsScore;                          ///< Glasgow Coma Scale (3-15)
    double intracranialPressure_mmHg;      ///< ICP
    double cerebralPerfusionPressure_mmHg; ///< CPP
    double meanArterialPressure_mmHg;      ///< MAP (placeholder, needs to be linked to Heart)

    // --- Simulation State ---
    double totalTime_s;

    // --- Anatomical Components ---
    BrainRegion frontalLobe;
    BrainRegion temporalLobe;
    BrainRegion parietalLobe;
    BrainRegion occipitalLobe;
    BrainRegion cerebellum;

    // --- Autonomic Control Targets ---
    double targetRespirationRate_bpm;
    double targetHeartRate_bpm;

    // --- Waveform Data ---
    std::deque<double> eegData;
    size_t eegHistorySize;
};
