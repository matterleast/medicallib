#pragma once

#include "Organ.h"
#include <string>
#include <vector>

/**
 * @brief Represents the functional status of a neural pathway.
 */
enum class SignalStatus {
    NORMAL,
    IMPAIRED,
    SEVERED
};

/**
 * @brief Represents a major bundle of nerve fibers in the spinal cord.
 */
struct SpinalTract {
    std::string name;                       ///< The name of the tract (e.g., "Corticospinal").
    SignalStatus status;                    ///< The functional status of the neural pathway.
    double conductionVelocity_m_per_s;      ///< The speed of signal travel in meters/sec.
};

/**
 * @brief Represents the Spinal Cord, simulating the transmission of neural signals.
 *
 * @section bio_sec Biological Overview
 * The spinal cord is a long, fragile, tube-like structure that begins at the end of the
 * brain stem and continues down almost to the bottom of the spine. It is a vital link
 * between the brain and the rest of the body, forming the central nervous system (CNS)
 * along with the brain.
 *
 * The spinal cord has two primary functions:
 * - **Signal Pathway**: It contains neural pathways that transmit information.
 *   - **Ascending Tracts**: Carry sensory information (touch, pain, temperature) from the
 *     peripheral nervous system up to the brain.
 *   - **Descending Tracts**: Carry motor commands from the brain down to the muscles and glands.
 * - **Reflex Coordination**: It can independently mediate simple reflexes, which are rapid,
 *   involuntary responses to stimuli, without input from the brain.
 *
 * @section model_sec Code Simulation
 * This `SpinalCord` class models the integrity and function of the major neural pathways.
 *
 * @subsection pathway_model_sec Pathway Model
 * The simulation represents the major pathways as `SpinalTract` structs for the ascending
 * (sensory) and descending (motor) signals. The integrity of these tracts is represented
 * by the `SignalStatus` enum (`NORMAL`, `IMPAIRED`, `SEVERED`), allowing for the simulation
 * of spinal cord injuries.
 *
 * Key functions to check the status include `getMotorPathwayStatus()` and `getSensoryPathwayStatus()`.
 * A simplified `isReflexArcIntact()` function represents the status of a basic reflex.
 *
 * @subsection signal_flow_sec Neural Signal Flow Diagram
 * This diagram shows the flow of motor and sensory signals between the brain, spinal cord,
 * and the peripheral nervous system (the body).
 *
 * @dot
 * digraph SignalFlow {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     Brain [shape=ellipse];
 *     PNS [label="Peripheral Nervous System\n(Body)"];
 *
 *     subgraph cluster_CNS {
 *         label="Central Nervous System";
 *         style=filled;
 *         color=lightblue;
 *         SpinalCord [label="Spinal Cord"];
 *     }
 *
 *     Brain -> SpinalCord [label=" Motor Commands\n(Descending Tract)"];
 *     SpinalCord -> PNS [label=" Motor Output"];
 *     PNS -> SpinalCord [label=" Sensory Input"];
 *     SpinalCord -> Brain [label=" Sensory Information\n(Ascending Tract)"];
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code shows how to create a `SpinalCord` object and check its pathway status.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/SpinalCord.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a SpinalCord object
 *     SpinalCord spinalCord(1);
 *
 *     // Check the initial status
 *     std::cout << "Initial State: " << spinalCord.getSummary() << std::endl;
 *
 *     // In a larger simulation, an injury event might change the pathway status.
 *     // For this example, we just check the default state.
 *     if (spinalCord.getMotorPathwayStatus() == SignalStatus::NORMAL) {
 *         std::cout << "Motor pathways are intact." << std::endl;
 *     }
 *
 *     if (spinalCord.isReflexArcIntact()) {
 *         std::cout << "Basic reflex arcs are functional." << std::endl;
 *     }
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API SpinalCord : public Organ {
public:
    /**
     * @brief Constructor for the SpinalCord class.
     * @param id The ID of the organ.
     */
    SpinalCord(int id);

    /**
     * @brief Updates the spinal cord's state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the spinal cord's vitals.
     * @return A string containing the spinal cord's vital signs.
     */
    std::string getSummary() const override;

    // --- Getters for Key Neurological Pathways ---

    /** @brief Gets the status of the primary motor pathways. */
    SignalStatus getMotorPathwayStatus() const;

    /** @brief Gets the status of the primary sensory pathways. */
    SignalStatus getSensoryPathwayStatus() const;

    /** @brief Gets the status of a basic reflex arc. */
    bool isReflexArcIntact() const;

private:
    // --- Helper to convert enum to string ---
    std::string statusToString(SignalStatus status) const;

    // --- Anatomical Components ---
    SpinalTract descendingMotorTract;
    SpinalTract ascendingSensoryTract;

    // --- Physiological Parameters ---
    bool reflexArcIntact; // Simplified representation of a reflex (e.g., patellar)
};
