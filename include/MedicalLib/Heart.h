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
