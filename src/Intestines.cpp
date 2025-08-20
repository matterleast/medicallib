#include "MedicalLib/Intestines.h"
#include "MedicalLib/Patient.h"
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
      chymeVolume_mL(0.0) {

    // Initialize segments with typical values
    duodenum = {"Duodenum", 0.25, 1.0, 0.5, 0.1};
    jejunum = {"Jejunum", 2.5, 1.0, 1.0, 0.3};
    ileum = {"Ileum", 3.0, 1.0, 0.8, 0.5};
    colon = {"Colon", 1.5, 0.5, 0.1, 1.0}; // High water absorption
}

void Intestines::update(Patient& patient, double deltaTime_s) {
    if (chymeVolume_mL > 0) {
        // Simplified absorption model: total absorption is an average of all segments
        double totalNutrientAbsorption = (duodenum.nutrientAbsorptionRate + jejunum.nutrientAbsorptionRate + ileum.nutrientAbsorptionRate + colon.nutrientAbsorptionRate) / 4.0;
        double totalWaterAbsorption = (duodenum.waterAbsorptionRate + jejunum.waterAbsorptionRate + ileum.waterAbsorptionRate + colon.waterAbsorptionRate) / 4.0;

        // Absorb glucose into the blood
        double glucoseAbsorption = totalNutrientAbsorption * chymeVolume_mL * 0.001 * deltaTime_s;
        patient.blood.glucose_mg_per_dL += glucoseAbsorption;

        // Reduce chyme volume based on absorption
        double absorbedVolume = (totalNutrientAbsorption * 0.01 + totalWaterAbsorption * 0.1) * deltaTime_s;
        chymeVolume_mL -= absorbedVolume;
        chymeVolume_mL = std::max(0.0, chymeVolume_mL);
    }

    // Fluctuate motility slightly
    duodenum.motility += getFluctuation(0.01);
    duodenum.motility = std::clamp(duodenum.motility, 0.9, 1.1);
}

void Intestines::receiveChyme(double volume_mL) {
    chymeVolume_mL += volume_mL;
}

std::string Intestines::getSummary() const {
    std::stringstream ss;
    ss.precision(2);
    ss << std::fixed;
    ss << "--- Intestines Summary ---\n"
       << "Total Chyme Volume: " << getTotalChymeVolume() << " mL\n\n"
       << "--- Segments ---\n"
       << duodenum.name << ": Motility " << duodenum.motility << "\n"
       << jejunum.name << ": Nutrient Abs. " << jejunum.nutrientAbsorptionRate << "\n"
       << ileum.name << ": Water Abs. " << ileum.waterAbsorptionRate << "\n"
       << colon.name << ": Water Abs. " << colon.waterAbsorptionRate << "\n";
    return ss.str();
}

// --- Getters Implementation ---
double Intestines::getTotalChymeVolume() const { return chymeVolume_mL; }
