#pragma once

#include "Organ.h"

/**
 * @brief Represents the Brain organ.
 */
class MEDICAL_LIB_API Brain : public Organ {
public:
    /**
     * @brief Constructor for the Brain class.
     * @param id The ID of the organ.
     */
    Brain(int id);

    /**
     * @brief Updates the brain's state over time.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the brain's vitals.
     * @return A string containing the brain's vital signs.
     */
    std::string getSummary() const override;

    // Specific getters for Brain properties
    double getConsciousnessLevel() const; // A simplified scale, e.g., 0.0 to 1.0
    double getCerebralBloodFlow() const; // in ml/100g/min

private:
    double consciousnessLevel;
    double cerebralBloodFlow;
};
