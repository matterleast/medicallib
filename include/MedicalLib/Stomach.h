#pragma once

#include "Organ.h"
#include <string>
#include <vector>

/**
 * @brief Represents the current digestive state of the stomach.
 */
enum class GastricState {
    EMPTY,
    FILLING,
    DIGESTING,
    EMPTYING
};

/**
 * @brief Represents the mixture of partially digested food and gastric juices.
 */
struct Chyme {
    double volume_mL;   ///< The volume of chyme in mL.
    double acidity_pH;  ///< The pH of the chyme.
};

/**
 * @brief Represents the Stomach, simulating the initial stages of digestion.
 *
 * @section bio_sec Biological Overview
 * The stomach is a muscular, J-shaped organ in the upper abdomen. It is a key part of the
 * digestive system. Its primary functions are:
 * - **Storage**: It acts as a temporary reservoir for food coming from the esophagus, allowing
 *   for a large meal to be consumed at once.
 * - **Digestion**: It secretes strong gastric juices, primarily hydrochloric acid and the enzyme
 *   pepsin, which begin the process of breaking down proteins. The stomach's muscular walls
 *   (rugae) contract to mix the food with these juices.
 * - **Controlled Release**: The stomach turns the food into a semi-liquid mixture called **chyme**.
 *   It then slowly releases this chyme into the small intestine (duodenum) for further
 *   digestion and nutrient absorption.
 *
 * The digestive process involves several states, from being empty, to filling, actively
 * digesting, and finally emptying its contents.
 *
 * @section model_sec Code Simulation
 * This `Stomach` class models the digestive cycle of the stomach.
 *
 * @subsection state_model_sec State-Based Digestive Model
 * The simulation is managed as a state machine, governed by the `GastricState` enum. The stomach
 * transitions through these states based on its contents and time:
 * - `EMPTY`: No contents.
 * - `FILLING`: Food is being added (via `addSubstance()`).
 * - `DIGESTING`: Contents are being mixed and broken down, and pH is lowered.
 * - `EMPTYING`: Chyme is being passed to the intestines.
 *
 * The contents themselves are modeled by the `Chyme` struct, which tracks volume and acidity.
 * The `update()` method drives the state transitions and simulates the secretion of gastric
 * juices and the emptying process.
 *
 * @subsection gastric_state_diagram_sec Gastric State Diagram
 * The following diagram illustrates the transitions between the stomach's digestive states.
 *
 * @dot
 * digraph GastricStates {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     EMPTY -> FILLING [label="addSubstance()"];
 *     FILLING -> DIGESTING [label="update()"];
 *     DIGESTING -> EMPTYING [label="update() over time"];
 *     EMPTYING -> EMPTY [label="volume = 0"];
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to add food to the stomach and observe the resulting
 * changes in its state and volume.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Stomach.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Stomach object
 *     Stomach stomach(1);
 *     std::cout << "Initial State: " << stomach.getSummary() << std::endl;
 *
 *     // Add a meal
 *     std::cout << "\nEating a 500mL meal..." << std::endl;
 *     stomach.addSubstance(500.0);
 *     std::cout << "State after eating: " << stomach.getSummary() << std::endl;
 *
 *     // Simulate digestion over time
 *     std::cout << "\nSimulating digestion for 30 minutes..." << std::endl;
 *     for (int i = 0; i < 30; ++i) {
 *         stomach.update(patient, 60.0); // Simulate 1 minute
 *     }
 *     std::cout << "State after 30 mins: " << stomach.getSummary() << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Stomach : public Organ {
public:
    /**
     * @brief Constructor for the Stomach class.
     * @param id The ID of the organ.
     */
    Stomach(int id);

    /**
     * @brief Updates the stomach state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the stomach's state.
     * @return A string containing the stomach's state.
     */
    std::string getSummary() const override;

    /**
     * @brief Adds a substance (e.g., a bolus from the esophagus) to the stomach.
     * @param volume_mL The volume of the substance.
     */
    void addSubstance(double volume_mL);

    // --- Getters for Gastric State ---

    /** @brief Gets the current gastric state. */
    GastricState getCurrentState() const;

    /** @brief Gets the current volume of contents in the stomach in mL. */
    double getVolume() const;

    /** @brief Gets the current pH of the stomach contents. */
    double getAcidity() const;

private:
    // --- Helper to convert enum to string ---
    std::string stateToString(GastricState state) const;

    // --- Physiological Parameters ---
    GastricState currentState;
    double currentVolume_mL;
    double currentPh;
    double gastricJuiceSecretionRate_ml_per_s;
    double emptyingRate_ml_per_s;

    // --- Simulation State ---
    const double capacity_mL = 1500.0;
};
