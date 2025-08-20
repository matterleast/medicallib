#pragma once

#include "Organ.h"

/**
 * @brief Represents the Intestines (Small and Large combined).
 */
class MEDICAL_LIB_API Intestines : public Organ {
public:
    Intestines(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getNutrientAbsorptionRate() const;
    double getWaterAbsorptionRate() const;

private:
    double nutrientAbsorptionRate; // in arbitrary units
    double waterAbsorptionRate; // in ml/s
};
