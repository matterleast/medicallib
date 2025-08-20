#pragma once

#include "Organ.h"

/**
 * @brief Represents the Spinal Cord.
 */
class MEDICAL_LIB_API SpinalCord : public Organ {
public:
    SpinalCord(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getSignalConductionVelocity() const; // in m/s

private:
    double signalConductionVelocity;
};
