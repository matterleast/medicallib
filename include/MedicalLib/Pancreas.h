#pragma once

#include "Organ.h"

/**
 * @brief Represents the Pancreas, with both endocrine and exocrine functions.
 */
class MEDICAL_LIB_API Pancreas : public Organ {
public:
    Pancreas(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getInsulinProduction() const;
    double getGlucagonProduction() const;

private:
    double insulinProduction; // in units/hr
    double glucagonProduction; // in ng/hr
};
