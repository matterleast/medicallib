#pragma once

#include "Organ.h"

/**
 * @brief Represents the Spleen.
 */
class MEDICAL_LIB_API Spleen : public Organ {
public:
    Spleen(int id);
    void update(double deltaTime_s) override;
    std::string getSummary() const override;

    double getRedBloodCellCount() const;
    double getWhiteBloodCellCount() const;

private:
    double redBloodCellCount; // in millions/uL
    double whiteBloodCellCount; // in thousands/uL
};
