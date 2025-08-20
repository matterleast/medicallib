#pragma once

#include "Organ.h"
#include <vector>
#include <string>
#include <deque>

/**
 * @brief Enum for the current state of the respiratory cycle.
 */
enum class RespiratoryState {
    INSPIRATION,
    EXPIRATION,
    PAUSE
};

/**
 * @brief Represents a single lobe of the lung.
 */
struct Lobe {
    std::string name;
    double volume_mL;
    double compliance; // How easily it inflates
};

/**
 * @brief Represents a major airway.
 */
struct Bronchus {
    std::string name;
    double resistance; // Airflow resistance
};

/**
 * @brief Represents the Lungs organ with a more detailed physiological model.
 */
class MEDICAL_LIB_API Lungs : public Organ {
public:
    /**
     * @brief Constructor for the Lungs class.
     * @param id The ID of the organ.
     */
    Lungs(int id);

    /**
     * @brief Updates the lungs' state over a time interval.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the lungs' vitals.
     * @return A string containing the lungs' vital signs.
     */
    std::string getSummary() const override;

    // --- Getters for Key Respiratory Vitals ---

    /** @brief Gets the current respiration rate in breaths per minute. */
    double getRespirationRate() const;

    /** @brief Gets the current oxygen saturation (SpO2) as a percentage. */
    double getOxygenSaturation() const;

    /** @brief Gets the volume of air in a normal breath (tidal volume) in mL. */
    double getTidalVolume() const;

    /** @brief Gets the end-tidal CO2 (etCO2) value in mmHg. */
    double getEndTidalCO2() const;

    /** @brief Gets the peak airway pressure during inspiration in cmH2O. */
    double getPeakInspiratoryPressure() const;

    /** @brief Gets the data for the capnography waveform (etCO2 over time). */
    const std::deque<double>& getCapnographyWaveform() const;

private:
    // --- Private Helper Methods ---
    void updateRespiratoryMechanics(double deltaTime_s);
    void updateGasLevels(double deltaTime_s);
    double generateCapnographyValue();

    // --- Physiological Parameters ---
    double respirationRate;      ///< Breaths per minute
    double oxygenSaturation;     ///< SpO2 percentage
    double tidalVolume_mL;       ///< Volume of air per breath
    double endTidalCO2_mmHg;     ///< End-tidal CO2
    double peakInspiratoryPressure_cmH2O; ///< Peak airway pressure
    double totalLungCapacity_mL; ///< Total lung capacity

    // --- Simulation State ---
    RespiratoryState currentState;
    double cyclePosition_s;      ///< Current position in the respiratory cycle (seconds)
    double totalTime_s;          ///< Total simulation time

    // --- Anatomical Components ---
    Lobe rightUpperLobe;
    Lobe rightMiddleLobe;
    Lobe rightLowerLobe;
    Lobe leftUpperLobe;
    Lobe leftLowerLobe;
    Bronchus mainBronchus;

    // --- Waveform Data ---
    std::deque<double> capnographyData;
    size_t capnographyHistorySize;
};
