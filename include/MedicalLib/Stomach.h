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
 * @brief Represents the Stomach with a more detailed physiological model.
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
