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
    std::string name;
    double length_m;
    double motility; // Rate of chyme movement
    double nutrientAbsorptionRate;
    double waterAbsorptionRate;
};

/**
 * @brief Represents the Intestines (Small and Large) with a more detailed model.
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
