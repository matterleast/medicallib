#pragma once

#include "Organ.h"
#include <vector>
#include <string>
#include <map>
#include <deque>

/**
 * @brief Represents the state of a heart valve.
 */
enum class ValveStatus { OPEN, CLOSED };

/**
 * @brief Represents a single heart valve and its potential pathologies.
 */
struct Valve {
    std::string name;                           ///< The name of the valve (e.g., "Mitral").
    ValveStatus status = ValveStatus::CLOSED;   ///< The current status of the valve (OPEN or CLOSED).
    double stenosis = 0.0;                      ///< Degree of narrowing, normalized [0, 1].
    double regurgitation = 0.0;                 ///< Degree of leakage, normalized [0, 1].
};

/**
 * @brief Represents the state of a heart chamber.
 */
enum class ChamberState { SYSTOLE, DIASTOLE };

/**
 * @brief Represents a single chamber of the heart.
 */
struct Chamber {
    std::string name;                           ///< The name of the chamber (e.g., "Left Ventricle").
    ChamberState state = ChamberState::DIASTOLE;///< The current state of the chamber (SYSTOLE or DIASTOLE).
    double volume_mL = 0.0;                     ///< The current volume of blood in the chamber in mL.
    double pressure_mmHg = 0.0;                 ///< The current pressure in the chamber in mmHg.
    double endDiastolicVolume_mL = 120.0;       ///< The volume of blood at the end of diastole (filling).
    double endSystolicVolume_mL = 50.0;         ///< The volume of blood at the end of systole (contraction).
};

/**
 * @brief Represents the Heart organ, with detailed mechanical and electrical simulation.
 *
 * @section bio_sec Biological Overview
 * The heart is a muscular organ responsible for pumping blood throughout the circulatory system.
 * It is divided into four chambers: two upper atria and two lower ventricles. The right side
 * of the heart handles deoxygenated blood, while the left side handles oxygenated blood.
 *
 * - **Deoxygenated blood** from the body enters the **Right Atrium**.
 * - It is pumped to the **Right Ventricle** through the **Tricuspid Valve**.
 * - The Right Ventricle pumps the blood to the lungs through the **Pulmonary Valve**.
 * - **Oxygenated blood** from the lungs enters the **Left Atrium**.
 * - It is pumped to the **Left Ventricle** through the **Mitral Valve**.
 * - The Left Ventricle pumps the oxygenated blood to the rest of the body through the **Aortic Valve**.
 *
 * This entire sequence is known as the cardiac cycle, which involves two main phases:
 * 1.  **Diastole**: The relaxation phase, where chambers fill with blood.
 * 2.  **Systole**: The contraction phase, where chambers pump blood out.
 *
 * @section model_sec Code Simulation
 * This `Heart` class provides a detailed simulation of both the mechanical and electrical functions
 * of the human heart.
 *
 * @subsection mech_model_sec Mechanical Model
 * The four chambers and four primary valves are modeled using the `Chamber` and `Valve` structs.
 * The `update()` function drives the simulation forward in time, calculating changes in chamber
 * volume and pressure based on the current phase of the cardiac cycle (systole or diastole).
 * Key outputs of the mechanical simulation include:
 * - **Ejection Fraction**: The percentage of blood pumped out of the left ventricle with each beat.
 *   Calculated by `getEjectionFraction()`.
 * - **Aortic Pressure**: Represents the systemic blood pressure. Retrieved with `getAorticPressure()`.
 *
 * @subsection elec_model_sec Electrical Model
 * The class also simulates the heart's electrical activity, which is observable via an
 * electrocardiogram (EKG). The `simulateEkgWaveform()` function generates a realistic EKG signal
 * based on the cardiac cycle's timing. The number of EKG leads can be configured in the constructor.
 * The generated data can be accessed using `getEkgData()`.
 *
 * @subsection a_graph_sec Blood Flow Diagram
 * The following diagram illustrates the path of blood through the heart's chambers and valves.
 *
 * @dot
 * digraph BloodFlow {
 *     rankdir="TB";
 *     node [shape=box, style=rounded];
 *
 *     subgraph cluster_Deoxygenated {
 *         label="Deoxygenated Blood (Pulmonary Circuit)";
 *         style=filled;
 *         color=lightblue;
 *         Body [label="Vena Cava\n(from Body)"];
 *         RA [label="Right Atrium"];
 *         RV [label="Right Ventricle"];
 *         Lungs_In [label="Pulmonary Artery\n(to Lungs)"];
 *         Body -> RA [label=" Enters"];
 *         RA -> RV [label=" Tricuspid Valve"];
 *         RV -> Lungs_In [label=" Pulmonary Valve"];
 *     }
 *
 *     subgraph cluster_Oxygenated {
 *         label="Oxygenated Blood (Systemic Circuit)";
 *         style=filled;
 *         color=lightpink;
 *         Lungs_Out [label="Pulmonary Vein\n(from Lungs)"];
 *         LA [label="Left Atrium"];
 *         LV [label="Left Ventricle"];
 *         Aorta [label="Aorta\n(to Body)"];
 *         Lungs_Out -> LA [label=" Enters"];
 *         LA -> LV [label=" Mitral Valve"];
 *         LV -> Aorta [label=" Aortic Valve"];
 *     }
 * }
 * @enddot
 *
 * @section usage_sec Example Usage
 * Here is a simple example of how to create a `Heart` object, simulate it over time,
 * and retrieve data.
 *
 * @code{.cpp}
 * #include <iostream>
 * #include "MedicalLib/Heart.h"
 * #include "MedicalLib/Patient.h"
 *
 * int main() {
 *     // A patient object is needed for the update function
 *     Patient patient;
 *
 *     // Create a Heart with 12 EKG leads
 *     Heart heart(1, 12);
 *
 *     // Set a baseline heart rate
 *     heart.setHeartRate(75.0);
 *
 *     // Simulate the heart for 5 seconds
 *     std::cout << "Simulating Heart for 5 seconds..." << std::endl;
 *     for (int i = 0; i < 5; ++i) {
 *         heart.update(patient, 1.0); // Update by 1.0 second
 *         std::cout << "Time: " << i + 1 << "s, " << heart.getSummary() << std::endl;
 *     }
 *
 *     // Retrieve specific metrics from the simulation
 *     std::cout << "\n--- Simulation Results ---" << std::endl;
 *     std::cout << "Final Measured Heart Rate: " << heart.getHeartRate() << " bpm" << std::endl;
 *     std::cout << "Final Ejection Fraction: " << heart.getEjectionFraction() << "%" << std::endl;
 *     std::cout << "Final Aortic Pressure: " << heart.getAorticPressure() << " mmHg" << std::endl;
 *
 *     return 0;
 * }
 * @endcode
 */
class MEDICAL_LIB_API Heart : public Organ {
public:
    /**
     * @brief Constructor for the Heart class.
     * @param id The ID of the organ.
     * @param numLeads The number of EKG leads to simulate (e.g., 3, 5, or 12).
     */
    Heart(int id, int numLeads = 12);

    /**
     * @brief Updates the heart's state over time, simulating the cardiac cycle.
     * @param patient A reference to the patient object.
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(Patient& patient, double deltaTime_s) override;

    /**
     * @brief Gets a string summary of the heart's vitals, including EKG and mechanical data.
     * @return A string containing the heart's vital signs.
     */
    std::string getSummary() const override;

    // --- Electrical Properties ---
    /**
     * @brief Sets the baseline heart rate.
     * @param newRate_bpm The new heart rate in beats per minute.
     */
    void setHeartRate(double newRate_bpm);

    /**
     * @brief Gets the current measured heart rate.
     * @return The heart rate in beats per minute.
     */
    double getHeartRate() const;

    /**
     * @brief Gets the simulated EKG data for all leads.
     * @return A constant reference to the map of EKG data.
     */
    const std::map<std::string, std::deque<double>>& getEkgData() const;

    // --- Mechanical Properties ---
    /**
     * @brief Calculates and returns the left ventricular ejection fraction.
     * @return The ejection fraction as a percentage [0, 100].
     */
    double getEjectionFraction() const;

    /**
     * @brief Gets the simulated aortic pressure, representing systemic blood pressure.
     * @return The aortic pressure in mmHg.
     */
    double getAorticPressure() const; // Represents systemic blood pressure

private:
    // --- Electrical Simulation ---
    double simulateEkgWaveform(double timeInCycle);
    double heartRate;           // Underlying target heart rate (bpm)
    double measuredHeartRate;
    int numLeads;
    std::vector<std::string> leadNames;
    std::map<std::string, std::deque<double>> ekgData;
    size_t ekgHistorySize;
    // Cycle timing
    double totalTime_s;
    double cardiacCyclePosition_s;
    double lastRPeakTime_s;
    bool rPeakDetectedInCycle;

    // --- Mechanical Simulation ---
    Chamber leftAtrium, rightAtrium, leftVentricle, rightVentricle;
    Valve mitralValve, tricuspidValve, aorticValve, pulmonaryValve;
    double ejectionFraction; // Percentage
};
