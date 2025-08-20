#pragma once

#include "Organ.h"

/**
 * @brief Represents the Heart organ.
 */
class MEDICAL_LIB_API Heart : public Organ {
public:
    /**
     * @brief Constructor for the Heart class.
     * @param id The ID of the organ.
     */
    Heart(int id);

    /**
     * @brief Updates the heart's state over time.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the heart's vitals.
     * @return A string containing the heart's vital signs.
     */
    std::string getSummary() const override;

    // Specific getters for Heart properties
    double getHeartRate() const;
    double getBloodPressureSystolic() const;
    double getBloodPressureDiastolic() const;

private:
    double heartRate;
    double bloodPressureSystolic;
    double bloodPressureDiastolic;
};
