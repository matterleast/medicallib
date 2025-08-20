#pragma once

#include "Organ.h"

/**
 * @brief Represents the Lungs organ.
 */
class MEDICAL_LIB_API Lungs : public Organ {
public:
    /**
     * @brief Constructor for the Lungs class.
     * @param id The ID of the organ.
     */
    Lungs(int id);

    /**
     * @brief Updates the lungs' state over time.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the lungs' vitals.
     * @return A string containing the lungs' vital signs.
     */
    std::string getSummary() const override;

    // Specific getters for Lungs properties
    double getRespirationRate() const;
    double getOxygenSaturation() const;

private:
    double respirationRate;
    double oxygenSaturation;
};
