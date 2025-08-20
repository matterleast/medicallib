#pragma once

#include "Organ.h"

/**
 * @brief Represents the Liver organ.
 */
class MEDICAL_LIB_API Liver : public Organ {
public:
    /**
     * @brief Constructor for the Liver class.
     * @param id The ID of the organ.
     */
    Liver(int id);

    /**
     * @brief Updates the liver's state over time.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the liver's vitals.
     * @return A string containing the liver's vital signs.
     */
    std::string getSummary() const override;

    // Specific getters for Liver properties
    double getBileProductionRate() const; // in ml/s
    double getGlucoseProductionRate() const; // in g/s

private:
    double bileProductionRate;
    double glucoseProductionRate;
};
