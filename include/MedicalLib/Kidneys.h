#pragma once

#include "Organ.h"
#include <string>
#include <vector>

/**
 * @brief Represents a functional unit of the kidney.
 */
struct Nephron {
    std::string id;                 ///< The unique ID of the nephron.
    double filtrationEfficiency;    ///< The filtration efficiency, normalized [0.0, 1.0].
    bool isDamaged;                 ///< True if the nephron has sustained damage.
};

/**
 * @brief Represents the Kidneys, responsible for filtering blood and producing urine.
 */
class MEDICAL_LIB_API Kidneys : public Organ {
public:
    /**
     * @brief Constructor for the Kidneys class.
     * @param id The ID of the organ.
     */
    Kidneys(int id);

    /**
     * @brief Updates the kidneys' state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the kidneys' state.
     * @return A string containing the kidneys' state.
     */
    std::string getSummary() const override;

    // --- Getters for Renal Function ---

    /** @brief Gets the Glomerular Filtration Rate (GFR) in mL/min. */
    double getGfr() const;

    /** @brief Gets the rate of urine production in mL/s. */
    double getUrineOutputRate() const;

    /** @brief Gets the simulated blood sodium level in mEq/L. */
    double getBloodSodium() const;

    /** @brief Gets the simulated blood potassium level in mEq/L. */
    double getBloodPotassium() const;

    /** @brief Gets the current rate of renin secretion. */
    double getReninSecretionRate() const;

private:
    // --- Physiological Parameters ---
    double reninSecretionRate; // In ng/mL/hr
    double gfr_mL_per_min;
    double urineOutput_mL_per_s;
    double bloodSodium_mEq_per_L;
    double bloodPotassium_mEq_per_L;

    // --- Anatomical Components ---
    std::vector<Nephron> nephrons;
    double totalFiltrationCapacity;
};
