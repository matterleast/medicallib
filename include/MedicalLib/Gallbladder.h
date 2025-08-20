#pragma once

#include "Organ.h"
#include <string>

/**
 * @brief Represents the contraction state of the gallbladder.
 */
enum class GallbladderState {
    STORING,
    CONTRACTING
};

/**
 * @brief Represents the Gallbladder, which stores, concentrates, and releases bile.
 *
 * @section bio_sec Biological Overview
 * The gallbladder is a small, pear-shaped organ located beneath the liver. Its main purpose is
 * to store and concentrate bile, a digestive fluid produced by the liver. Bile is essential
 * for the digestion of fats.
 *
 * After being produced by the liver, bile travels to the gallbladder where it is stored. The
 * gallbladder lining absorbs water from the bile, making it more concentrated. When fatty food
 * enters the small intestine, a hormone called cholecystokinin (CCK) is released, signaling the
 * gallbladder to contract and release the concentrated bile into the duodenum (the first part
 * of the small intestine).
 *
 * @section model_sec Code Simulation
 * This `Gallbladder` class models the storage and release cycle of bile.
 *
 * @subsection state_model_sec State-Based Model
 * The simulation is managed by the `GallbladderState` enum, which has two states:
 * - `STORING`: The default state, where the gallbladder is passively filling with bile from the liver.
 * - `CONTRACTING`: Triggered by a simulated signal (e.g., presence of fats), causing the
 *   gallbladder to release bile.
 *
 * The `storeBile()` method is used to add bile from the liver, and `releaseBile()` simulates the
 * contraction, returning the volume of bile ejected. The model also tracks the `bileConcentrationFactor`.
 *
 * @subsection bile_flow_sec Bile Flow Diagram
 * This diagram shows the flow of bile from the liver to the gallbladder and then to the intestine.
 *
 * @dot
 * digraph BileFlow {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     Liver [label="Liver\n(Produces Bile)"];
 *     Gallbladder [label="Gallbladder\n(Stores & Concentrates Bile)"];
 *     Intestine [label="Small Intestine\n(Duodenum)"];
 *
 *     Liver -> Gallbladder [label=" stores"];
 *     Gallbladder -> Intestine [label=" releases upon signal"];
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to simulate the gallbladder storing and then releasing bile.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Gallbladder.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Gallbladder object
 *     Gallbladder gallbladder(1);
 *     std::cout << "Initial State: " << gallbladder.getSummary() << std::endl;
 *
 *     // Store some bile from the liver
 *     std::cout << "\nStoring 10mL of bile..." << std::endl;
 *     gallbladder.storeBile(10.0);
 *     std::cout << "State after storing: " << gallbladder.getSummary() << std::endl;
 *
 *     // Simulate the gallbladder contracting and releasing bile
 *     std::cout << "\nFood detected! Releasing bile..." << std::endl;
 *     // In a full simulation, the update() method would trigger the state change.
 *     // We'll call releaseBile() directly for this example.
 *     double released = gallbladder.releaseBile(5.0); // Simulate 5s of release
 *     std::cout << "Released " << released << " mL of bile." << std::endl;
 *     std::cout << "Final State: " << gallbladder.getSummary() << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Gallbladder : public Organ {
public:
    /**
     * @brief Constructor for the Gallbladder class.
     * @param id The ID of the organ.
     */
    Gallbladder(int id);

    /**
     * @brief Updates the gallbladder's state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the gallbladder's state.
     * @return A string containing the gallbladder's state.
     */
    std::string getSummary() const override;

    /**
     * @brief Adds bile from the liver.
     * @param volume_mL The volume of bile to add.
     */
    void storeBile(double volume_mL);

    /**
     * @brief Releases bile when stimulated (e.g., by chyme in duodenum).
     * @param deltaTime_s The time step for this update.
     * @return The amount of bile released in mL.
     */
    double releaseBile(double deltaTime_s);

    // --- Getters for Gallbladder State ---

    /** @brief Gets the current volume of stored bile in mL. */
    double getStoredBileVolume() const;

    /** @brief Gets the concentration factor of the stored bile. */
    double getBileConcentration() const;

    /** @brief Gets the current state of the gallbladder. */
    GallbladderState getCurrentState() const;

private:
    // --- Helper to convert enum to string ---
    std::string stateToString(GallbladderState state) const;

    // --- Physiological Parameters ---
    GallbladderState currentState;
    double storedBile_mL;
    double bileConcentrationFactor; // How concentrated the bile is (1x, 5x, etc.)
    double bileReleaseRate_ml_per_s; // Rate of bile release when contracting
    const double capacity_mL = 50.0;
};
