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
 * @brief Represents the Bladder, which stores urine.
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
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

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
