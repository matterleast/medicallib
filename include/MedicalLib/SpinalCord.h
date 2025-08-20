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
    std::string name;
    SignalStatus status;
    double conductionVelocity_m_per_s;
};

/**
 * @brief Represents the Spinal Cord with a more detailed physiological model.
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
     * @param deltaTime_s The time elapsed in seconds.
     */
    void update(double deltaTime_s) override;

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
