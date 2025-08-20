#pragma once

#include "Organ.h"
#include <string>

/**
 * @brief Represents the mix of enzymes released by the pancreas for digestion.
 */
struct DigestiveEnzymes {
    double volume_mL = 0.0;         ///< The volume of the secreted fluid in mL.
    double amylase_U_per_L = 0.0;   ///< The concentration of amylase in Units/Liter.
    double lipase_U_per_L = 0.0;    ///< The concentration of lipase in Units/Liter.
};

/**
 * @brief Represents the Pancreas, simulating its dual endocrine and exocrine functions.
 *
 * @section bio_sec Biological Overview
 * The pancreas is a glandular organ that sits behind the stomach. It has two primary and distinct
 * functions, making it a "heterocrine" gland:
 *
 * - **Endocrine Function**: The pancreas contains clusters of cells called the islets of
 *   Langerhans. These cells produce and secrete hormones directly into the bloodstream to regulate
 *   blood sugar levels. The two main hormones are:
 *   - **Insulin**: Lowers blood sugar by promoting glucose uptake by cells.
 *   - **Glucagon**: Raises blood sugar by stimulating the liver to release stored glucose.
 *
 * - **Exocrine Function**: The pancreas produces powerful digestive enzymes that are secreted into
 *   the small intestine (duodenum) to help digest food. These enzymes include:
 *   - **Amylase**: Breaks down carbohydrates.
 *   - **Lipase**: Breaks down fats.
 *   - **Proteases**: Break down proteins.
 *
 * @section model_sec Code Simulation
 * This `Pancreas` class models both the endocrine and exocrine roles of the organ.
 *
 * @subsection endocrine_model_sec Endocrine Simulation
 * The endocrine function is simulated by tracking the secretion rates of key hormones. The `update()`
 * method adjusts these rates based on simulated blood glucose levels (from the `Patient` object).
 * Key outputs include:
 * - `getInsulinSecretion()`: The rate of insulin release.
 * - `getGlucagonSecretion()`: The rate of glucagon release.
 *
 * @subsection exocrine_model_sec Exocrine Simulation
 * The exocrine function is modeled by the `releaseEnzymes()` method. When called, this method
 * returns a `DigestiveEnzymes` struct, which contains the volume and concentration of secreted
 * amylase and lipase, ready to be passed to the `Intestines` model.
 *
 * @subsection pancreas_func_sec Pancreas Function Diagram
 * This diagram illustrates the two main functions of the pancreas.
 *
 * @dot
 * digraph PancreasFunctions {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     Pancreas [shape=ellipse];
 *
 *     subgraph cluster_Endocrine {
 *         label="Endocrine Function";
 *         style=filled;
 *         color=lightblue;
 *         Insulin [label="Insulin"];
 *         Glucagon [label="Glucagon"];
 *         Bloodstream [label="Bloodstream"];
 *         {Insulin, Glucagon} -> Bloodstream;
 *     }
 *
 *     subgraph cluster_Exocrine {
 *         label="Exocrine Function";
 *         style=filled;
 *         color=lightgreen;
 *         Enzymes [label="Digestive Enzymes\n(Amylase, Lipase)"];
 *         Duodenum [label="Small Intestine"];
 *         Enzymes -> Duodenum;
 *     }
 *
 *     Pancreas -> Insulin;
 *     Pancreas -> Glucagon;
 *     Pancreas -> Enzymes;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to simulate the pancreas and access both its endocrine and
 * exocrine function outputs.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Pancreas.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *     patient.setVital("BloodGlucose", 120.0); // Set a high blood glucose to trigger insulin
 *
 *     // Create a Pancreas object
 *     Pancreas pancreas(1);
 *
 *     // Simulate for a short period
 *     std::cout << "Simulating Pancreas function..." << std::endl;
 *     pancreas.update(patient, 10.0); // Simulate 10 seconds of activity
 *
 *     // Check endocrine output
 *     std::cout << "\n--- Endocrine Vitals ---" << std::endl;
 *     std::cout << "Insulin Secretion: " << pancreas.getInsulinSecretion() << " units/hr" << std::endl;
 *
 *     // Check exocrine output
 *     DigestiveEnzymes released = pancreas.releaseEnzymes(1.0);
 *     std::cout << "\n--- Exocrine Release (1s) ---" << std::endl;
 *     std::cout << "Volume: " << released.volume_mL << " mL" << std::endl;
 *     std::cout << "Amylase: " << released.amylase_U_per_L << " U/L" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
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

    // --- Exocrine Functions (Enzymes) ---

    /**
     * @brief Releases digestive enzymes when stimulated.
     * @param deltaTime_s The time step for this update.
     * @return A struct containing the released enzymes.
     */
    DigestiveEnzymes releaseEnzymes(double deltaTime_s);

private:
    // --- Endocrine Parameters ---
    double insulinSecretion_units_per_hr;
    double glucagonSecretion_ng_per_hr;

    // --- Exocrine Parameters ---
    double amylaseSecretion_U_per_L;
    double lipaseSecretion_U_per_L;
    double enzymeReleaseRate_ml_per_s;
};
