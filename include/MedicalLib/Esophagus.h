#pragma once

#include "Organ.h"

/**
 * @brief Represents the Esophagus.
 * @note This is a placeholder with minimal logic for now.
 */
class MEDICAL_LIB_API Esophagus : public Organ {
public:
    Esophagus(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getMotility() const; // Peristalsis efficiency

private:
    double motility;
};
