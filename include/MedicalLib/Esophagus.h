#pragma once

#include "Organ.h"
#include <string>
#include <vector>

/**
 * @brief Represents the state of esophageal muscle contraction.
 */
enum class PeristalsisState {
    IDLE,
    CONTRACTING,
    RELAXING
};

/**
 * @brief Represents a small mass of chewed food.
 */
struct Bolus {
    double volume_mL;     ///< The volume of the bolus in mL.
    double position_cm;   ///< Position from the top of the esophagus in cm.
};

/**
 * @brief Represents the Esophagus, simulating the transport of food via peristalsis.
 *
 * @section bio_sec Biological Overview
 * The esophagus is a muscular tube that connects the pharynx (throat) to the stomach. Its sole
 * function in the digestive system is to transport food and liquid from the mouth to the stomach.
 *
 * This transport is achieved through a process called **peristalsis**, which is a series of
 * wave-like muscle contractions that move food along the digestive tract. When a person swallows,
 * the esophagus muscles contract behind the chewed food (bolus) and relax in front of it, pushing
 * it downward. At the bottom, the lower esophageal sphincter (LES) relaxes to allow the food to
 * enter the stomach and then closes to prevent stomach contents from flowing back up.
 *
 * @section model_sec Code Simulation
 * This `Esophagus` class models the process of swallowing and peristalsis.
 *
 * @subsection state_model_sec State-Based Model
 * The simulation is managed by the `PeristalsisState` enum, which tracks the current muscle activity:
 * - `IDLE`: No swallowing is in progress.
 * - `CONTRACTING`: The muscle is actively contracting to push a bolus.
 * - `RELAXING`: The muscle is relaxing after a contraction.
 *
 * A mass of food is represented by the `Bolus` struct. The process begins when `initiateSwallow()`
 * is called, which creates a new bolus. The `update()` method then simulates the movement of this
 * bolus down the esophagus's length until it is delivered to the stomach.
 *
 * @subsection peristalsis_flow_sec Peristalsis Flow Diagram
 * This diagram shows the journey of a food bolus from the throat to the stomach.
 *
 * @dot
 * digraph PeristalsisFlow {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     Pharynx [label="Pharynx (Throat)"];
 *     Esophagus [label="Esophagus\n(Peristaltic Wave)"];
 *     Stomach [label="Stomach"];
 *
 *     Pharynx -> Esophagus [label="swallow()"];
 *     Esophagus -> Stomach [label="LES opens"];
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to simulate swallowing a bolus of food.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Esophagus.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create an Esophagus object
 *     Esophagus esophagus(1);
 *     std::cout << "Initial State: " << esophagus.getSummary() << std::endl;
 *
 *     // Initiate a swallow
 *     std::cout << "\nSwallowing a 15mL bolus..." << std::endl;
 *     esophagus.initiateSwallow(15.0);
 *
 *     // Simulate the bolus moving down the esophagus
 *     for (int i = 0; i < 10; ++i) {
 *         esophagus.update(patient, 1.0); // Simulate 1 second
 *         std::cout << "Time: " << i + 1 << "s, " << esophagus.getSummary() << std::endl;
 *         if (!esophagus.isSwallowing()) {
 *             std::cout << "Bolus has reached the stomach." << std::endl;
 *             break;
 *         }
 *     }
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Esophagus : public Organ {
public:
    /**
     * @brief Constructor for the Esophagus class.
     * @param id The ID of the organ.
     */
    Esophagus(int id);

    /**
     * @brief Updates the esophagus state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the esophagus's state.
     * @return A string containing the esophagus's state.
     */
    std::string getSummary() const override;

    /**
     * @brief Initiates a swallow.
     * @param bolusVolume_mL The volume of the bolus being swallowed.
     */
    void initiateSwallow(double bolusVolume_mL);

    // --- Getters for Esophageal State ---

    /** @brief Gets the current state of peristalsis. */
    PeristalsisState getCurrentState() const;

    /** @brief Checks if a bolus is currently being swallowed. */
    bool isSwallowing() const;

private:
    // --- Helper to convert enum to string ---
    std::string stateToString(PeristalsisState state) const;

    // --- Physiological Parameters ---
    PeristalsisState currentState;
    double lowerEsophagealSphincterTone; // Pressure in mmHg

    // --- Simulation State ---
    std::vector<Bolus> activeBoli;
    const double length_cm = 25.0; // Average length of esophagus
};
