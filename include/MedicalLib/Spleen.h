#pragma once

#include "Organ.h"
#include <string>

/**
 * @brief Represents the red pulp, responsible for filtering blood.
 */
struct RedPulp {
    double filtrationRate; // Arbitrary units
    double rbcBreakdownRate; // Rate of old red blood cell removal
};

/**
 * @brief Represents the white pulp, part of the immune system.
 */
struct WhitePulp {
    double lymphocyteCount; // in millions
    double macrophageCount; // in millions
};

/**
 * @brief Represents the Spleen, involved in blood filtering and immunity.
 */
class MEDICAL_LIB_API Spleen : public Organ {
public:
    /**
     * @brief Constructor for the Spleen class.
     * @param id The ID of the organ.
     */
    Spleen(int id);

    /**
     * @brief Updates the spleen's state over a time interval.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the spleen's state.
     * @return A string containing the spleen's state.
     */
    std::string getSummary() const override;

    // --- Getters for Spleen Function ---

    /** @brief Gets the rate of red blood cell breakdown. */
    double getRbcBreakdownRate() const;

    /** @brief Gets the total lymphocyte count in the white pulp. */
    double getLymphocyteCount() const;

private:
    // --- Anatomical Components ---
    RedPulp redPulp;
    WhitePulp whitePulp;
};
