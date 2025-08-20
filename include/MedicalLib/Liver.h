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
 * @brief Represents the Liver organ, simulating metabolic and detoxification functions.
 *
 * @section bio_sec Biological Overview
 * The liver is a large, essential organ located in the upper right quadrant of the abdomen. It
 * performs a vast number of critical functions, including:
 * - **Metabolism**: It metabolizes carbohydrates, fats, and proteins. It plays a central role in
 *   maintaining blood glucose levels, for example through gluconeogenesis (creating glucose).
 * - **Detoxification**: It filters the blood to remove toxins, drugs, and metabolic byproducts
 *   like bilirubin.
 * - **Synthesis**: It produces essential proteins, such as albumin and clotting factors. It also
 *   produces angiotensinogen, a key hormone in blood pressure regulation.
 * - **Bile Production**: It produces bile, a fluid that is stored in the gallbladder and is
 *   essential for the digestion of fats in the small intestine.
 *
 * Liver health is often assessed by measuring the levels of liver enzymes (like ALT and AST) and
 * bilirubin in the blood. Elevated levels can indicate liver damage or disease.
 *
 * @section model_sec Code Simulation
 * This `Liver` class models the liver's key metabolic and synthetic functions.
 *
 * @subsection func_model_sec Functional Model
 * The liver's vast processing capacity is represented by a collection of `HepaticLobule` structs,
 * which are the functional units of the organ. The `update()` method simulates ongoing metabolic
 * activity, calculating the production rates of various substances.
 *
 * @subsection vitals_sec Simulated Vitals and Outputs
 * The simulation tracks several key indicators of liver function:
 * - **Bile Production**: The rate at which bile is produced, accessible via `getBileProductionRate()`.
 * - **Glucose Production**: The rate of gluconeogenesis, retrieved with `getGlucoseProductionRate()`.
 * - **Liver Enzymes**: Simulated levels of ALT (`getAltLevel()`) and AST (`getAstLevel()`), which
 *   rise in response to liver damage.
 * - **Bilirubin**: The level of bilirubin in the blood, a marker for detoxification efficiency,
 *   available through `getBilirubinLevel()`.
 *
 * @subsection liver_flow_sec Liver Blood and Bile Flow
 * The following diagram illustrates the high-level flow of substances into and out of the liver.
 *
 * @dot
 * digraph LiverFlow {
 *     rankdir="LR";
 *     node [shape=record, style=rounded];
 *
 *     Input [label="{<pv>Portal Vein (Nutrient-rich Blood)|<ha>Hepatic Artery (Oxygen-rich Blood)}"];
 *     Liver [label="<in>Liver Processing|{Metabolism | Detoxification | Synthesis}"];
 *     Output [label="{<hv>Hepatic Vein (Cleaned Blood to Body)|<bd>Bile Duct (Bile to Gallbladder)}"];
 *
 *     Input:pv -> Liver:in;
 *     Input:ha -> Liver:in;
 *     Liver -> Output:hv;
 *     Liver -> Output:bd;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to create a `Liver` object and check its functional outputs.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Liver.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Liver object
 *     Liver liver(1);
 *
 *     // Simulate for a short period
 *     std::cout << "Simulating Liver function..." << std::endl;
 *     liver.update(patient, 10.0); // Simulate 10 seconds of activity
 *     std::cout << liver.getSummary() << std::endl;
 *
 *     // Retrieve specific final vitals
 *     std::cout << "\n--- Liver Function Tests ---" << std::endl;
 *     std::cout << "Bile Production Rate: " << liver.getBileProductionRate() << " mL/min" << std::endl;
 *     std::cout << "ALT Level: " << liver.getAltLevel() << " U/L" << std::endl;
 *     std::cout << "AST Level: " << liver.getAstLevel() << " U/L" << std::endl;
 *     std::cout << "Bilirubin Level: " << liver.getBilirubinLevel() << " mg/dL" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
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
