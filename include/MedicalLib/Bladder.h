#pragma once

#include "Organ.h"
#include <string>

/**
 * @brief Represents the state of the bladder muscles and sphincters.
 */
enum class MicturitionState {
    FILLING,
    FULL,
    VOIDING
};

/**
 * @brief Represents the Bladder, simulating the storage and expulsion of urine.
 *
 * @section bio_sec Biological Overview
 * The urinary bladder is a muscular, hollow organ that sits in the pelvic floor. Its primary
 * function is to collect and store urine produced by the kidneys. Urine enters the bladder
 * via two tubes called ureters.
 *
 * The wall of the bladder contains a muscle called the detrusor muscle, which can relax to
 * allow the bladder to stretch and fill, and contract to expel urine. The exit from the
 * bladder is controlled by two sphincters. The process of urinating is called micturition.
 * As the bladder fills, stretch receptors in its wall send signals to the brain, creating
 * the urge to urinate.
 *
 * @section model_sec Code Simulation
 * This `Bladder` class models the fill-void cycle of the urinary bladder.
 *
 * @subsection state_model_sec State-Based Model
 * The simulation is managed by the `MicturitionState` enum, which tracks the current phase:
 * - `FILLING`: The bladder is passively collecting urine from the kidneys.
 * - `FULL`: The bladder has reached a capacity where the urge to void is strong.
 * - `VOIDING`: The bladder is contracting to expel urine.
 *
 * The `addUrine()` method allows the `Kidneys` model to add to the bladder's volume. The `update()`
 * method simulates the increase in pressure as volume increases and handles the state transitions.
 *
 * @subsection micturition_state_sec Micturition Cycle Diagram
 * This diagram shows the state transitions of the bladder as it fills and voids.
 *
 * @dot
 * digraph MicturitionCycle {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     FILLING -> FULL [label="volume > threshold"];
 *     FULL -> VOIDING [label="voiding signal"];
 *     VOIDING -> FILLING [label="volume = 0"];
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to simulate urine being added to the bladder and observe
 * the change in its state.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Bladder.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Bladder object
 *     Bladder bladder(1);
 *     std::cout << "Initial State: " << bladder.getSummary() << std::endl;
 *
 *     // Add urine from the kidneys over time
 *     std::cout << "\nKidneys are producing urine..." << std::endl;
 *     for (int i = 0; i < 300; ++i) {
 *         bladder.addUrine(1.0); // Add 1mL of urine
 *         bladder.update(patient, 1.0);
 *     }
 *     std::cout << "State after adding 300mL: " << bladder.getSummary() << std::endl;
 *
 *     // Add more urine to reach the 'FULL' state
 *     for (int i = 0; i < 200; ++i) {
 *         bladder.addUrine(1.0);
 *         bladder.update(patient, 1.0);
 *     }
 *     std::cout << "State after adding another 200mL: " << bladder.getSummary() << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Bladder : public Organ {
public:
    /**
     * @brief Constructor for the Bladder class.
     * @param id The ID of the organ.
     */
    Bladder(int id);

    /**
     * @brief Updates the bladder's state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the bladder's state.
     * @return A string containing the bladder's state.
     */
    std::string getSummary() const override;

    /**
     * @brief Adds urine from the kidneys.
     * @param amount_ml The volume of urine to add.
     */
    void addUrine(double amount_ml);

    // --- Getters for Bladder State ---

    /** @brief Gets the current volume of urine in the bladder in mL. */
    double getVolume() const;

    /** @brief Gets the current pressure inside the bladder in cmH2O. */
    double getPressure() const;

    /** @brief Gets the current state of the micturition cycle. */
    MicturitionState getCurrentState() const;

private:
    // --- Helper to convert enum to string ---
    std::string stateToString(MicturitionState state) const;

    // --- Physiological Parameters ---
    MicturitionState currentState;
    double currentVolume_mL;
    double pressure_cmH2O;
    bool internalSphincterClosed;

    const double capacity_mL = 500.0;
    const double pressureThreshold_cmH2O = 40.0; // Pressure at which voiding reflex is strong
};
