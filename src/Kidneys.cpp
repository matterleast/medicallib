#include "MedicalLib/Kidneys.h"
#include "MedicalLib/Patient.h"
#include "MedicalLib/Heart.h"
#include "MedicalLib/Bladder.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <iomanip>
#include <numeric>

// Helper function for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Kidneys::Kidneys(int id)
    : Organ(id, "Kidneys"),
      reninSecretionRate(1.0), // Baseline ng/mL/hr
      gfr_mL_per_min(125.0),
      urineOutput_mL_per_s(0.02),
      bloodSodium_mEq_per_L(140.0),
      bloodPotassium_mEq_per_L(4.0),
      totalFiltrationCapacity(1.0) {

    // Create a simplified representation of nephrons
    nephrons.resize(100); // 100 representative units
    for (int i = 0; i < nephrons.size(); ++i) {
        nephrons[i] = {"Nephron " + std::to_string(i), 1.0, false};
    }
}

void Kidneys::update(Patient& patient, double deltaTime_s) {
    // Recalculate total capacity based on nephron health
    totalFiltrationCapacity = 0.0;
    for (const auto& nephron : nephrons) {
        if (!nephron.isDamaged) {
            totalFiltrationCapacity += nephron.filtrationEfficiency;
        }
    }
    totalFiltrationCapacity /= nephrons.size();

    // GFR is dependent on blood pressure from the heart
    double perfusionPressure = 90.0; // Assume normal MAP if heart is not present
    if (const Heart* heart = getOrgan<Heart>(patient)) {
        perfusionPressure = heart->getAorticPressure();
    }
    double pressureModifier = std::clamp(perfusionPressure / 90.0, 0.5, 1.2);

    const double baseline_gfr = 125.0 * totalFiltrationCapacity * pressureModifier;
    gfr_mL_per_min += 0.1 * (baseline_gfr - gfr_mL_per_min) * deltaTime_s + getFluctuation(0.5);

    // Urine output is related to GFR
    urineOutput_mL_per_s = gfr_mL_per_min / 60.0 * 0.01; // Simplified relationship
    urineOutput_mL_per_s += getFluctuation(0.001);

    // Pass urine to the bladder
    if (Bladder* bladder = getOrgan<Bladder>(patient)) {
        bladder->addUrine(urineOutput_mL_per_s * deltaTime_s);
    }

    // Simulate electrolyte balance
    bloodSodium_mEq_per_L += getFluctuation(0.05);
    bloodPotassium_mEq_per_L += getFluctuation(0.01);

    // --- RAAS Regulation ---
    // Renin is released in response to low blood pressure.
    const auto& bp = patient.blood.bloodPressure;
    double meanArterialPressure = bp.diastolic_mmHg + (bp.systolic_mmHg - bp.diastolic_mmHg) / 3.0;

    if (meanArterialPressure < 85.0) { // Low pressure threshold
        reninSecretionRate += (85.0 - meanArterialPressure) * 0.1 * deltaTime_s;
    } else {
        // If pressure is normal, renin decays back to baseline
        reninSecretionRate -= (reninSecretionRate - 1.0) * 0.05 * deltaTime_s;
    }

    // Clamp to healthy ranges
    gfr_mL_per_min = std::clamp(gfr_mL_per_min, 90.0, 150.0);
    urineOutput_mL_per_s = std::clamp(urineOutput_mL_per_s, 0.01, 0.03);
    bloodSodium_mEq_per_L = std::clamp(bloodSodium_mEq_per_L, 135.0, 145.0);
    bloodPotassium_mEq_per_L = std::clamp(bloodPotassium_mEq_per_L, 3.5, 5.0);
    reninSecretionRate = std::clamp(reninSecretionRate, 0.5, 50.0);
}

std::string Kidneys::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Kidneys Summary ---\n"
       << "Glomerular Filtration Rate (GFR): " << getGfr() << " mL/min\n"
       << "Urine Output: " << getUrineOutputRate() * 3600 << " mL/hr\n"
       << "Renin Secretion: " << getReninSecretionRate() << " ng/mL/hr\n"
       << "Blood Sodium: " << getBloodSodium() << " mEq/L\n"
       << "Blood Potassium: " << getBloodPotassium() << " mEq/L\n";
    return ss.str();
}

// --- Getters Implementation ---
double Kidneys::getGfr() const { return gfr_mL_per_min; }
double Kidneys::getUrineOutputRate() const { return urineOutput_mL_per_s; }
double Kidneys::getBloodSodium() const { return bloodSodium_mEq_per_L; }
double Kidneys::getBloodPotassium() const { return bloodPotassium_mEq_per_L; }
double Kidneys::getReninSecretionRate() const { return reninSecretionRate; }
