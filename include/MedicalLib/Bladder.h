#pragma once

#include "Organ.h"

/**
 * @brief Represents the Bladder.
 */
class MEDICAL_LIB_API Bladder : public Organ {
public:
    Bladder(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    // Note: A real model would get urine input from kidneys.
    // For now, it will just fill at a constant rate.
    void addUrine(double amount_ml);

    double getCurrentVolume() const;
    double getCapacity() const;

private:
    double currentVolume; // in ml
    double capacity; // in ml
};
