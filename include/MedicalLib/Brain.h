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
    std::string name;
    double activityLevel; // 0.0 to 1.0
    double bloodFlow_ml_100g_min;
};

/**
 * @brief Represents the Brain organ with a more detailed physiological model.
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
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

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

    // --- Waveform Data ---
    std::deque<double> eegData;
    size_t eegHistorySize;
};
