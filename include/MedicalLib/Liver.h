#pragma once

#include "Organ.h"
#include <string>
#include <vector>

/**
 * @brief Represents a functional unit of the liver.
 */
struct HepaticLobule {
    std::string id;             ///< The unique ID of the lobule.
    double metabolicActivity;   ///< A factor representing the metabolic activity [0.0, 1.0+].
    bool isDamaged;             ///< True if the lobule has sustained damage.
};

/**
 * @brief Represents the Liver organ with a more detailed physiological model.
 */
class MEDICAL_LIB_API Liver : public Organ {
public:
    /**
     * @brief Constructor for the Liver class.
     * @param id The ID of the organ.
     */
    Liver(int id);

    /**
     * @brief Updates the liver's state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the liver's vitals.
     * @return A string containing the liver's vital signs.
     */
    std::string getSummary() const override;

    // --- Getters for Key Metabolic Vitals ---

    /** @brief Gets the rate of bile production in mL/min. */
    double getBileProductionRate() const;

    /** @brief Gets the rate of glucose production (gluconeogenesis) in g/min. */
    double getGlucoseProductionRate() const;

    /** @brief Gets the Alanine Aminotransferase (ALT) level in U/L. */
    double getAltLevel() const;

    /** @brief Gets the Aspartate Aminotransferase (AST) level in U/L. */
    double getAstLevel() const;

    /** @brief Gets the total bilirubin level in mg/dL. */
    double getBilirubinLevel() const;

    /** @brief Gets the production rate of angiotensinogen. */
    double getAngiotensinogenRate() const;

private:
    // --- Physiological Parameters ---
    double angiotensinogen_production_rate; // In arbitrary units/s
    double bileProductionRate_ml_per_s;
    double glucoseProductionRate_g_per_s;
    double alt_U_per_L;      ///< Alanine Aminotransferase
    double ast_U_per_L;      ///< Aspartate Aminotransferase
    double bilirubin_mg_per_dL; ///< Total bilirubin

    // --- Anatomical Components ---
    std::vector<HepaticLobule> lobules;
    double totalMetabolicCapacity;
};
