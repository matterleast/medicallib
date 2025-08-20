#pragma once

#include "Organ.h"
#include <vector>
#include <string>
#include <deque>

/**
 * @brief Enum for the current state of the respiratory cycle.
 */
enum class RespiratoryState {
    INSPIRATION,
    EXPIRATION,
    PAUSE
};

/**
 * @brief Represents a single lobe of the lung.
 */
struct Lobe {
    std::string name;   ///< The name of the lobe.
    double volume_mL;   ///< The current volume of air in the lobe in mL.
    double compliance;  ///< The measure of the lung's ability to stretch and expand.
};

/**
 * @brief Represents a major airway.
 */
struct Bronchus {
    std::string name;   ///< The name of the bronchus.
    double resistance;  ///< The resistance to airflow.
};

/**
 * @brief Represents the Lungs organ, simulating respiratory mechanics and gas exchange.
 *
 * @section bio_sec Biological Overview
 * The lungs are the primary organs of the respiratory system, responsible for gas exchange.
 * Air is inhaled through the trachea, which branches into two main bronchi, one for each lung.
 * These bronchi further divide into smaller bronchioles, eventually leading to tiny air sacs
 * called alveoli, where oxygen from the inhaled air passes into the blood and carbon dioxide
 * from the blood is released into the air to be exhaled.
 *
 * The process of breathing consists of two phases:
 * 1.  **Inspiration**: The diaphragm and intercostal muscles contract, expanding the chest cavity
 *     and drawing air into the lungs.
 * 2.  **Expiration**: The muscles relax, the chest cavity shrinks, and air is forced out of the lungs.
 *
 * The human lungs are divided into lobes; the right lung has three (upper, middle, lower) and the
 * left lung has two (upper, lower).
 *
 * @section model_sec Code Simulation
 * This `Lungs` class models the physiological functions of the lungs, including the mechanics of
 * breathing and the resulting changes in vital signs.
 *
 * @subsection resp_mech_sec Respiratory Mechanics
 * The anatomical structure is simplified into five `Lobe` structs and a main `Bronchus` struct.
 * The `update()` method drives the respiratory cycle, which is governed by the `RespiratoryState`
 * enum (`INSPIRATION`, `EXPIRATION`, `PAUSE`). The simulation calculates changes in lobe volume
 * based on physiological parameters like `respirationRate` and `tidalVolume_mL`.
 *
 * @subsection gas_exchange_sec Gas Exchange and Vitals
 * The simulation produces several key respiratory vital signs:
 * - **Oxygen Saturation (SpO2)**: A measure of the oxygen level in the blood, accessible via `getOxygenSaturation()`.
 * - **End-Tidal CO2 (etCO2)**: The concentration of carbon dioxide at the end of an exhaled breath,
 *   retrieved with `getEndTidalCO2()`. The class also generates a continuous capnography waveform
 *   (`getCapnographyWaveform()`).
 * - **Tidal Volume**: The volume of air moved in a single breath, available through `getTidalVolume()`.
 *
 * @subsection airflow_graph_sec Airflow Diagram
 * The following diagram shows the simplified path of air from the main bronchus to the five lobes.
 *
 * @dot
 * digraph Airflow {
 *     rankdir="LR";
 *     node [shape=box, style=rounded];
 *
 *     Trachea [label="Trachea / Main Bronchus"];
 *
 *     subgraph cluster_RightLung {
 *         label="Right Lung";
 *         style=filled;
 *         color=lightblue;
 *         RUL [label="Right Upper Lobe"];
 *         RML [label="Right Middle Lobe"];
 *         RLL [label="Right Lower Lobe"];
 *     }
 *
 *     subgraph cluster_LeftLung {
 *         label="Left Lung";
 *         style=filled;
 *         color=lightpink;
 *         LUL [label="Left Upper Lobe"];
 *         LLL [label="Left Lower Lobe"];
 *     }
 *
 *     Trachea -> RUL;
 *     Trachea -> RML;
 *     Trachea -> RLL;
 *     Trachea -> LUL;
 *     Trachea -> LLL;
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * The following C++ code demonstrates how to create a `Lungs` object, set a
 * respiration rate, and monitor its vitals over a short period.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include <vector>
 * #include "MedicalLib/Lungs.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Lungs object
 *     Lungs lungs(1);
 *
 *     // Set a custom respiration rate
 *     lungs.setRespirationRate(16.0); // 16 breaths per minute
 *
 *     // Simulate for 10 seconds
 *     std::cout << "Simulating Lungs for 10 seconds..." << std::endl;
 *     for (int i = 0; i < 10; ++i) {
 *         lungs.update(patient, 1.0); // Update by 1.0 second
 *         std::cout << "Time: " << i + 1 << "s, " << lungs.getSummary() << std::endl;
 *     }
 *
 *     // Retrieve specific final vitals
 *     std::cout << "\n--- Simulation Results ---" << std::endl;
 *     std::cout << "Final SpO2: " << lungs.getOxygenSaturation() << "%" << std::endl;
 *     std::cout << "Final etCO2: " << lungs.getEndTidalCO2() << " mmHg" << std::endl;
 *     std::cout << "Final Tidal Volume: " << lungs.getTidalVolume() << " mL" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Lungs : public Organ {
public:
    /**
     * @brief Constructor for the Lungs class.
     * @param id The ID of the organ.
     */
    Lungs(int id);

    /**
     * @brief Updates the lungs' state over a time interval.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the lungs' vitals.
     * @return A string containing the lungs' vital signs.
     */
    std::string getSummary() const override;

    /**
     * @brief Inflicts damage on the lungs, reducing their compliance.
     * @param damage The amount of damage to inflict (0-1).
     */
    void inflictDamage(double damage);

    // --- Setters for External Control ---

    /**
     * @brief Sets the respiration rate.
     * @param newRate_bpm The new rate in breaths per minute.
     */
    void setRespirationRate(double newRate_bpm);

    // --- Getters for Key Respiratory Vitals ---

    /** @brief Gets the current respiration rate in breaths per minute. */
    double getRespirationRate() const;

    /** @brief Gets the current oxygen saturation (SpO2) as a percentage. */
    double getOxygenSaturation() const;

    /** @brief Gets the volume of air in a normal breath (tidal volume) in mL. */
    double getTidalVolume() const;

    /** @brief Gets the end-tidal CO2 (etCO2) value in mmHg. */
    double getEndTidalCO2() const;

    /** @brief Gets the peak airway pressure during inspiration in cmH2O. */
    double getPeakInspiratoryPressure() const;

    /** @brief Gets the data for the capnography waveform (etCO2 over time). */
    const std::deque<double>& getCapnographyWaveform() const;

private:
    // --- Private Helper Methods ---
    void updateRespiratoryMechanics(double deltaTime_s);
    void updateGasLevels(double deltaTime_s);
    double generateCapnographyValue();

    // --- Physiological Parameters ---
    double respirationRate;      ///< Breaths per minute
    double oxygenSaturation;     ///< SpO2 percentage
    double tidalVolume_mL;       ///< Volume of air per breath
    double endTidalCO2_mmHg;     ///< End-tidal CO2
    double peakInspiratoryPressure_cmH2O; ///< Peak airway pressure
    double totalLungCapacity_mL; ///< Total lung capacity

    // --- Simulation State ---
    RespiratoryState currentState;
    double cyclePosition_s;      ///< Current position in the respiratory cycle (seconds)
    double totalTime_s;          ///< Total simulation time

    // --- Anatomical Components ---
    Lobe rightUpperLobe;
    Lobe rightMiddleLobe;
    Lobe rightLowerLobe;
    Lobe leftUpperLobe;
    Lobe leftLowerLobe;
    Bronchus mainBronchus;

    // --- Waveform Data ---
    std::deque<double> capnographyData;
    size_t capnographyHistorySize;
};
