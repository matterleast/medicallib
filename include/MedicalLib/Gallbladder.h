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
 * @brief Represents the Gallbladder, which stores and concentrates bile.
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
