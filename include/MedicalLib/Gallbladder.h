#pragma once

#include "Organ.h"

/**
 * @brief Represents the Gallbladder.
 */
class MEDICAL_LIB_API Gallbladder : public Organ {
public:
    Gallbladder(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getBileStored() const;

private:
    double bileStored; // in ml
};
