#include "MedicalLib/Kidneys.h"
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

void Kidneys::update(double deltaTime_s) {
    // Recalculate total capacity based on nephron health
    totalFiltrationCapacity = 0.0;
    for (const auto& nephron : nephrons) {
        if (!nephron.isDamaged) {
            totalFiltrationCapacity += nephron.filtrationEfficiency;
        }
    }
    totalFiltrationCapacity /= nephrons.size();

    // GFR is dependent on overall health
    const double baseline_gfr = 125.0 * totalFiltrationCapacity;
    gfr_mL_per_min += 0.1 * (baseline_gfr - gfr_mL_per_min) * deltaTime_s + getFluctuation(0.5);

    // Urine output is related to GFR but also hydration status (not modeled yet)
    urineOutput_mL_per_s = gfr_mL_per_min / 60.0 * 0.01; // Simplified relationship
    urineOutput_mL_per_s += getFluctuation(0.001);

    // Simulate electrolyte balance
    bloodSodium_mEq_per_L += getFluctuation(0.05);
    bloodPotassium_mEq_per_L += getFluctuation(0.01);

    // Clamp to healthy ranges
    gfr_mL_per_min = std::clamp(gfr_mL_per_min, 90.0, 150.0);
    urineOutput_mL_per_s = std::clamp(urineOutput_mL_per_s, 0.01, 0.03);
    bloodSodium_mEq_per_L = std::clamp(bloodSodium_mEq_per_L, 135.0, 145.0);
    bloodPotassium_mEq_per_L = std::clamp(bloodPotassium_mEq_per_L, 3.5, 5.0);
}

std::string Kidneys::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Kidneys Summary ---\n"
       << "Glomerular Filtration Rate (GFR): " << getGfr() << " mL/min\n"
       << "Urine Output: " << getUrineOutputRate() * 3600 << " mL/hr\n"
       << "Blood Sodium: " << getBloodSodium() << " mEq/L\n"
       << "Blood Potassium: " << getBloodPotassium() << " mEq/L\n";
    return ss.str();
}

// --- Getters Implementation ---
double Kidneys::getGfr() const { return gfr_mL_per_min; }
double Kidneys::getUrineOutputRate() const { return urineOutput_mL_per_s; }
double Kidneys::getBloodSodium() const { return bloodSodium_mEq_per_L; }
double Kidneys::getBloodPotassium() const { return bloodPotassium_mEq_per_L; }
