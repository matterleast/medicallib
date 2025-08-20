#pragma once

#include "Organ.h"

/**
 * @brief Represents the Kidneys.
 */
class MEDICAL_LIB_API Kidneys : public Organ {
public:
    Kidneys(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getFiltrationRate() const;
    double getUrineProductionRate() const;

private:
    double filtrationRate; // in ml/min
    double urineProductionRate; // in ml/s
};
