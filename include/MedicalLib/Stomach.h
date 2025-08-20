#pragma once

#include "Organ.h"

/**
 * @brief Represents the Stomach.
 */
class MEDICAL_LIB_API Stomach : public Organ {
public:
    Stomach(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getPhLevel() const;
    double getDigestionRate() const;

private:
    double phLevel; // Acidity
    double digestionRate; // in arbitrary units
};
