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
 * @brief Represents the Kidneys, simulating blood filtration, urine production, and electrolyte balance.
 *
 * @section bio_sec Biological Overview
 * The kidneys are a pair of bean-shaped organs that perform several vital functions. Their primary
 * role is to filter waste products, excess water, and other impurities from the blood. These
 * waste products are converted into urine, which flows to the bladder to be excreted.
 *
 * Each kidney contains millions of tiny filtering units called **nephrons**. Blood enters the
 * nephron, where waste is filtered out and essential substances are reabsorbed back into the blood.
 *
 * Key functions of the kidneys include:
 * - **Waste Excretion**: Filtering out urea, creatinine, and other metabolic wastes.
 * - **Electrolyte Balance**: Regulating the levels of sodium, potassium, and other electrolytes.
 * - **Blood Pressure Regulation**: Producing the enzyme renin, which plays a key role in the
 *   renin-angiotensin-aldosterone system that controls blood pressure.
 * - **Acid-Base Balance**: Maintaining the pH of the blood within a narrow range.
 *
 * The **Glomerular Filtration Rate (GFR)** is a key measure of kidney function, representing the
 * volume of fluid filtered from the blood per unit time.
 *
 * @section model_sec Code Simulation
 * This `Kidneys` class models the filtration and regulatory functions of the kidneys.
 *
 * @subsection func_model_sec Functional Model
 * The kidney's filtering capacity is represented by a vector of `Nephron` structs. Each nephron
 * has a specific `filtrationEfficiency`. The `update()` method simulates the ongoing process of
 * blood filtration, calculating urine output and changes in blood chemistry.
 *
 * @subsection vitals_sec Simulated Renal Vitals
 * The simulation tracks several key indicators of renal function:
 * - **GFR**: The Glomerular Filtration Rate, a primary indicator of kidney health, accessible via `getGfr()`.
 * - **Urine Output**: The rate of urine production, retrieved with `getUrineOutputRate()`.
 * - **Electrolytes**: Simulated levels of blood sodium (`getBloodSodium()`) and potassium (`getBloodPotassium()`).
 * - **Renin**: The secretion rate of renin, a key hormone for blood pressure control, available
 *   through `getReninSecretionRate()`.
 *
 * @subsection kidney_flow_sec Kidney Filtration Flow
 * The following diagram illustrates the high-level flow of blood through the kidneys for filtration.
 *
 * @dot
 * digraph KidneyFlow {
 *     rankdir="TB";
 *     node [shape=record, style=rounded];
 *
 *     BloodIn [label="Renal Artery\n(Unfiltered Blood)"];
 *     Kidney [label="{<n>Nephrons|Glomerular Filtration}"];
 *     BloodOut [label="Renal Vein\n(Filtered Blood)"];
 *     UrineOut [label="Ureter\n(Urine to Bladder)"];
 *
 *     BloodIn -> Kidney:n;
 *     Kidney -> BloodOut;
 *     Kidney -> UrineOut;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to create a `Kidneys` object and monitor its function.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Kidneys.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Kidneys object
 *     Kidneys kidneys(1);
 *
 *     // Simulate for a short period
 *     std::cout << "Simulating Kidneys function for 1 minute..." << std::endl;
 *     kidneys.update(patient, 60.0); // Simulate 60 seconds of activity
 *     std::cout << kidneys.getSummary() << std::endl;
 *
 *     // Retrieve specific final vitals
 *     std::cout << "\n--- Renal Function Panel ---" << std::endl;
 *     std::cout << "GFR: " << kidneys.getGfr() << " mL/min" << std::endl;
 *     std::cout << "Urine Output Rate: " << kidneys.getUrineOutputRate() << " mL/s" << std::endl;
 *     std::cout << "Blood Sodium: " << kidneys.getBloodSodium() << " mEq/L" << std::endl;
 *     std::cout << "Blood Potassium: " << kidneys.getBloodPotassium() << " mEq/L" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
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
