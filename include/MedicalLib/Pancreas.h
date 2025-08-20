#pragma once

#include "Organ.h"
#include <string>

/**
 * @brief Represents the Pancreas, with both endocrine and exocrine functions.
 */
class MEDICAL_LIB_API Pancreas : public Organ {
public:
    /**
     * @brief Constructor for the Pancreas class.
     * @param id The ID of the organ.
     */
    Pancreas(int id);

    /**
     * @brief Updates the pancreas's state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the pancreas's state.
     * @return A string containing the pancreas's state.
     */
    std::string getSummary() const override;

    // --- Getters for Endocrine Functions (Hormones) ---

    /** @brief Gets the current insulin secretion rate in units/hr. */
    double getInsulinSecretion() const;

    /** @brief Gets the current glucagon secretion rate in ng/hr. */
    double getGlucagonSecretion() const;

    // --- Getters for Exocrine Functions (Enzymes) ---

    /** @brief Gets the current amylase secretion rate in U/L. */
    double getAmylaseSecretion() const;

    /** @brief Gets the current lipase secretion rate in U/L. */
    double getLipaseSecretion() const;

private:
    // --- Endocrine Parameters ---
    double insulinSecretion_units_per_hr;
    double glucagonSecretion_ng_per_hr;

    // --- Exocrine Parameters ---
    double amylaseSecretion_U_per_L;
    double lipaseSecretion_U_per_L;
};
