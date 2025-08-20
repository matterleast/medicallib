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
 * @brief Represents the Esophagus with a more detailed physiological model.
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
