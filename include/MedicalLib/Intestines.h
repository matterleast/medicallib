#pragma once

#include "Organ.h"
#include <string>
#include <vector>

// Forward-declare for receiveEnzymes method
struct DigestiveEnzymes;

/**
 * @brief Represents a segment of the intestines.
 */
struct IntestinalSegment {
    std::string name;                   ///< The name of the segment (e.g., "Duodenum").
    double length_m;                    ///< The length of the segment in meters.
    double motility;                    ///< The rate of chyme movement.
    double nutrientAbsorptionRate;      ///< The rate at which nutrients are absorbed.
    double waterAbsorptionRate;         ///< The rate at which water is absorbed.
};

/**
 * @brief Represents the Intestines (Small and Large), simulating final digestion and absorption.
 *
 * @section bio_sec Biological Overview
 * The intestines are a crucial part of the digestive tract where most of the absorption of
 * nutrients and water happens. They are divided into two main parts:
 *
 * - **Small Intestine**: A long, coiled tube where the majority of digestion and nutrient
 *   absorption takes place. It receives chyme from the stomach, along with bile from the
 *   gallbladder and digestive enzymes from the pancreas. It has three segments:
 *   1.  **Duodenum**: The first section, where chyme is mixed with bile and enzymes.
 *   2.  **Jejunum**: The middle section, where most nutrient absorption occurs.
 *   3.  **Ileum**: The final section, which absorbs any remaining nutrients.
 *
 * - **Large Intestine (Colon)**: A shorter, wider tube that absorbs water and electrolytes from
 *   the remaining indigestible food matter and then passes useless waste material from the body.
 *
 * @section model_sec Code Simulation
 * This `Intestines` class models the passage of chyme through the various intestinal segments.
 *
 * @subsection segment_model_sec Segment-Based Model
 * The small and large intestines are modeled as a series of `IntestinalSegment` structs, each
 * with its own properties for motility and absorption rates. The `update()` method simulates
 * the movement and processing of chyme through these segments over time.
 *
 * @subsection interaction_sec Interaction with Other Organs
 * The model is designed to work with other digestive organs:
 * - `receiveChyme()`: Takes the output from the Stomach.
 * - `receiveBile()`: Takes bile from the Gallbladder to aid in fat digestion.
 * - `receiveEnzymes()`: Takes enzymes from the Pancreas to break down macromolecules.
 *
 * @subsection intestines_flow_sec Digestive Flow Diagram
 * The following diagram shows the path of chyme as it enters from the stomach and moves
 * through the different segments of the intestines.
 *
 * @dot
 * digraph IntestinesFlow {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     Stomach [label="Stomach"];
 *     Duodenum [label="Duodenum"];
 *     Jejunum [label="Jejunum"];
 *     Ileum [label="Ileum"];
 *     Colon [label="Colon (Large Intestine)"];
 *     Waste [label="Waste Exit"];
 *
 *     Pancreas [shape=ellipse, style=filled, color=lightgrey];
 *     Gallbladder [shape=ellipse, style=filled, color=lightgrey];
 *
 *     Stomach -> Duodenum [label="Chyme"];
 *     Pancreas -> Duodenum [label="Enzymes"];
 *     Gallbladder -> Duodenum [label="Bile"];
 *     Duodenum -> Jejunum;
 *     Jejunum -> Ileum;
 *     Ileum -> Colon;
 *     Colon -> Waste;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to simulate the intestines receiving chyme from the stomach.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Intestines.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create an Intestines object
 *     Intestines intestines(1);
 *     std::cout << "Initial State: " << intestines.getSummary() << std::endl;
 *
 *     // Receive chyme from the stomach
 *     std::cout << "\nReceiving 200mL of chyme..." << std::endl;
 *     intestines.receiveChyme(200.0);
 *     std::cout << "State after receiving chyme: " << intestines.getSummary() << std::endl;
 *
 *     // Simulate digestion and absorption over time
 *     std::cout << "\nSimulating for 1 hour..." << std::endl;
 *     for (int i = 0; i < 60; ++i) {
 *         intestines.update(patient, 60.0); // Simulate 1 minute
 *     }
 *     std::cout << "State after 1 hour: " << intestines.getSummary() << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Intestines : public Organ {
public:
    /**
     * @brief Constructor for the Intestines class.
     * @param id The ID of the organ.
     */
    Intestines(int id);

    /**
     * @brief Updates the intestines' state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the intestines' state.
     * @return A string containing the intestines' state.
     */
    std::string getSummary() const override;

    /**
     * @brief Adds chyme from the stomach to the duodenum.
     * @param volume_mL The volume of chyme.
     */
    void receiveChyme(double volume_mL);

    /**
     * @brief Adds bile from the gallbladder.
     * @param volume_mL The volume of bile.
     */
    void receiveBile(double volume_mL);

    /**
     * @brief Adds digestive enzymes from the pancreas.
     * @param enzymes A struct containing the enzyme information.
     */
    void receiveEnzymes(const DigestiveEnzymes& enzymes);

    // --- Getters for Intestinal State ---

    /** @brief Gets the total volume of chyme currently in the intestines. */
    double getTotalChymeVolume() const;

private:
    // --- Anatomical Components ---
    IntestinalSegment duodenum;
    IntestinalSegment jejunum;
    IntestinalSegment ileum;
    IntestinalSegment colon;

    // --- Simulation State ---
    double chymeVolume_mL; // Total volume in the whole system for now
    double bileVolume_mL;
    double enzymeVolume_mL;
    double amylase_U_per_L;
    double lipase_U_per_L;
};
