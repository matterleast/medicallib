#pragma once

#include "Organ.h"
#include <string>

/**
 * @brief Represents the red pulp, responsible for filtering blood.
 */
struct RedPulp {
    double filtrationRate;      ///< The rate of blood filtration, in arbitrary units.
    double rbcBreakdownRate;    ///< The rate of old red blood cell removal.
};

/**
 * @brief Represents the white pulp, part of the immune system.
 */
struct WhitePulp {
    double lymphocyteCount;     ///< The number of lymphocytes, in millions.
    double macrophageCount;     ///< The number of macrophages, in millions.
};

/**
 * @brief Represents the Spleen, simulating its role in blood filtration and immunity.
 *
 * @section bio_sec Biological Overview
 * The spleen is an organ located in the upper left part of the abdomen, and it's a key
 * component of both the circulatory and lymphatic systems. It is not essential for life,
 * but it performs several important functions. The spleen is composed of two primary
 * types of tissue:
 *
 * - **Red Pulp**: This tissue acts as a blood filter. It removes old, malformed, or damaged
 *   red blood cells from circulation. It also serves as a reservoir for platelets and
 *   white blood cells, which can be released in an emergency.
 *
 * - **White Pulp**: This tissue is part of the immune system. It is rich in lymphocytes
 *   (B-cells and T-cells) and macrophages. It helps identify and fight off invading
 *   pathogens like bacteria and viruses found in the blood.
 *
 * @section model_sec Code Simulation
 * This `Spleen` class models the distinct functions of the red and white pulp.
 *
 * @subsection pulp_model_sec Pulp-Based Functional Model
 * The simulation separates the spleen's functions into two structs:
 * - `RedPulp`: Models the blood filtration process, including the rate at which old red blood
 *   cells are broken down (`getRbcBreakdownRate()`).
 * - `WhitePulp`: Models the immune component, tracking the population of key immune cells like
 *   lymphocytes (`getLymphocyteCount()`).
 *
 * The `update()` method simulates the continuous activity of both pulp types.
 *
 * @subsection spleen_flow_sec Spleen Function Diagram
 * This diagram illustrates how blood is processed by the two functional parts of the spleen.
 *
 * @dot
 * digraph SpleenFunctions {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     BloodIn [label="Blood Supply"];
 *     Spleen [shape=ellipse];
 *     BloodOut [label="Filtered Blood"];
 *
 *     subgraph cluster_RedPulp {
 *         label="Red Pulp";
 *         style=filled;
 *         color=lightpink;
 *         Filter [label="Filter Blood\n(Remove old RBCs)"];
 *     }
 *
 *     subgraph cluster_WhitePulp {
 *         label="White Pulp";
 *         style=filled;
 *         color=lightblue;
 *         Immunity [label="Immune Surveillance\n(Lymphocytes)"];
 *     }
 *
 *     BloodIn -> Spleen;
 *     Spleen -> Filter;
 *     Spleen -> Immunity;
 *     {Filter, Immunity} -> BloodOut;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to create a `Spleen` object and get a summary of its state.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Spleen.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Spleen object
 *     Spleen spleen(1);
 *
 *     // Simulate for a short period
 *     std::cout << "Simulating Spleen function..." << std::endl;
 *     spleen.update(patient, 10.0); // Simulate 10 seconds of activity
 *     std::cout << spleen.getSummary() << std::endl;
 *
 *     // Retrieve specific data
 *     std::cout << "\n--- Spleen Vitals ---" << std::endl;
 *     std::cout << "RBC Breakdown Rate: " << spleen.getRbcBreakdownRate() << std::endl;
 *     std::cout << "Lymphocyte Count: " << spleen.getLymphocyteCount() << " million" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
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
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

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
