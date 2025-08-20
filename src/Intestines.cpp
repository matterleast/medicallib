#include "MedicalLib/Intestines.h"
#include "MedicalLib/Patient.h"
#include "MedicalLib/Gallbladder.h"
#include "MedicalLib/Pancreas.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <iomanip>

// Helper function for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Intestines::Intestines(int id)
    : Organ(id, "Intestines"),
      chymeVolume_mL(0.0),
      bileVolume_mL(0.0),
      enzymeVolume_mL(0.0),
      amylase_U_per_L(0.0),
      lipase_U_per_L(0.0) {

    // Initialize segments with typical values
    duodenum = {"Duodenum", 0.25, 1.0, 0.5, 0.1};
    jejunum = {"Jejunum", 2.5, 1.0, 1.0, 0.3};
    ileum = {"Ileum", 3.0, 1.0, 0.8, 0.5};
    colon = {"Colon", 1.5, 0.5, 0.1, 1.0}; // High water absorption
}

void Intestines::update(Patient& patient, double deltaTime_s) {
    if (chymeVolume_mL > 0) {
        // 1. Signal Gallbladder and Pancreas to release substances
        if (Gallbladder* gallbladder = getOrgan<Gallbladder>(patient)) {
            double bileReleased = gallbladder->releaseBile(deltaTime_s);
            receiveBile(bileReleased);
        }
        if (Pancreas* pancreas = getOrgan<Pancreas>(patient)) {
            DigestiveEnzymes enzymesReleased = pancreas->releaseEnzymes(deltaTime_s);
            receiveEnzymes(enzymesReleased);
        }

        // 2. Digestion and Absorption
        // Bile helps emulsify fats, enzymes break down nutrients.
        // We'll model this as a "digestion efficiency" multiplier.
        double digestionEfficiency = 1.0;
        if (bileVolume_mL > 0 && enzymeVolume_mL > 0) {
            digestionEfficiency = 5.0; // 5x more effective with bile and enzymes
        }

        // Simplified absorption model
        double totalNutrientAbsorptionRate = (duodenum.nutrientAbsorptionRate + jejunum.nutrientAbsorptionRate + ileum.nutrientAbsorptionRate) * digestionEfficiency;
        double totalWaterAbsorptionRate = (duodenum.waterAbsorptionRate + jejunum.waterAbsorptionRate + ileum.waterAbsorptionRate + colon.waterAbsorptionRate);

        // Absorb glucose into the blood
        double glucoseAbsorption = totalNutrientAbsorptionRate * chymeVolume_mL * 0.001 * deltaTime_s;
        patient.blood.glucose_mg_per_dL += glucoseAbsorption;

        // Reduce chyme/bile/enzyme volume based on absorption and processing
        double absorbedVolume = (totalNutrientAbsorptionRate * 0.01 + totalWaterAbsorptionRate * 0.1) * deltaTime_s;
        chymeVolume_mL -= absorbedVolume;

        // As chyme is processed, bile and enzymes are used up
        bileVolume_mL -= 0.1 * bileVolume_mL * deltaTime_s;
        enzymeVolume_mL -= 0.1 * enzymeVolume_mL * deltaTime_s;

        // Clamp volumes to zero
        chymeVolume_mL = std::max(0.0, chymeVolume_mL);
        bileVolume_mL = std::max(0.0, bileVolume_mL);
        enzymeVolume_mL = std::max(0.0, enzymeVolume_mL);

        if (enzymeVolume_mL == 0.0) {
            amylase_U_per_L = 0.0;
            lipase_U_per_L = 0.0;
        }
    }

    // Fluctuate motility slightly
    duodenum.motility += getFluctuation(0.01);
    duodenum.motility = std::clamp(duodenum.motility, 0.9, 1.1);
}

void Intestines::receiveChyme(double volume_mL) {
    chymeVolume_mL += volume_mL;
}

void Intestines::receiveBile(double volume_mL) {
    bileVolume_mL += volume_mL;
}

void Intestines::receiveEnzymes(const DigestiveEnzymes& enzymes) {
    if (enzymes.volume_mL <= 0) return;

    // Calculate new weighted average of enzyme concentration
    double total_enzyme_vol = enzymeVolume_mL + enzymes.volume_mL;
    amylase_U_per_L = (amylase_U_per_L * enzymeVolume_mL + enzymes.amylase_U_per_L * enzymes.volume_mL) / total_enzyme_vol;
    lipase_U_per_L = (lipase_U_per_L * enzymeVolume_mL + enzymes.lipase_U_per_L * enzymes.volume_mL) / total_enzyme_vol;
    enzymeVolume_mL = total_enzyme_vol;
}

std::string Intestines::getSummary() const {
    std::stringstream ss;
    ss.precision(2);
    ss << std::fixed;
    ss << "--- Intestines Summary ---\n"
       << "Chyme Volume: " << getTotalChymeVolume() << " mL\n"
       << "Bile Volume: " << bileVolume_mL << " mL\n"
       << "Enzyme Volume: " << enzymeVolume_mL << " mL\n"
       << "Amylase: " << amylase_U_per_L << " U/L\n"
       << "Lipase: " << lipase_U_per_L << " U/L\n\n"
       << "--- Segments ---\n"
       << duodenum.name << ": Motility " << duodenum.motility << "\n"
       << jejunum.name << ": Nutrient Abs. " << jejunum.nutrientAbsorptionRate << "\n"
       << ileum.name << ": Water Abs. " << ileum.waterAbsorptionRate << "\n"
       << colon.name << ": Water Abs. " << colon.waterAbsorptionRate << "\n";
    return ss.str();
}

// --- Getters Implementation ---
double Intestines::getTotalChymeVolume() const { return chymeVolume_mL; }
