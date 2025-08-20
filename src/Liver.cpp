#include "MedicalLib/Liver.h"
#include "MedicalLib/Patient.h" // For Blood struct
#include <random>
#include <algorithm>
#include <sstream>
#include <numeric>

// Helper function for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Liver::Liver(int id)
    : Organ(id, "Liver"),
      angiotensinogen_production_rate(10.0), // Constant production
      bileProductionRate_ml_per_s(0.0069),
      glucoseProductionRate_g_per_s(0.001),
      alt_U_per_L(25.0),
      ast_U_per_L(25.0),
      bilirubin_mg_per_dL(0.8),
      totalMetabolicCapacity(1.0) {

    // Create a simplified representation of lobules
    lobules.resize(100); // 100 representative units
    for (int i = 0; i < lobules.size(); ++i) {
        lobules[i] = {"Lobule " + std::to_string(i), 1.0, false};
    }
}

void Liver::update(Patient& patient, double deltaTime_s) {
    // Recalculate total capacity based on lobule health (for future use)
    totalMetabolicCapacity = 0.0;
    for (const auto& lobule : lobules) {
        if (!lobule.isDamaged) {
            totalMetabolicCapacity += lobule.metabolicActivity;
        }
    }
    totalMetabolicCapacity /= lobules.size();

    // Baseline production rates are modulated by liver health
    const double baseline_bile_rate = 0.0069 * totalMetabolicCapacity;
    const double baseline_glucose_rate = 0.001 * totalMetabolicCapacity;

    // Update production rates with minor fluctuations
    bileProductionRate_ml_per_s += 0.02 * (baseline_bile_rate - bileProductionRate_ml_per_s) * deltaTime_s + getFluctuation(0.0001);
    glucoseProductionRate_g_per_s += 0.02 * (baseline_glucose_rate - glucoseProductionRate_g_per_s) * deltaTime_s + getFluctuation(0.00005);

    // Update enzyme and bilirubin levels
    // In a healthy state, they hover around a normal baseline
    alt_U_per_L += getFluctuation(0.1);
    ast_U_per_L += getFluctuation(0.1);
    bilirubin_mg_per_dL += getFluctuation(0.01);

    // Clamp to normal healthy ranges
    bileProductionRate_ml_per_s = std::clamp(bileProductionRate_ml_per_s, 0.005, 0.009);
    glucoseProductionRate_g_per_s = std::clamp(glucoseProductionRate_g_per_s, 0.0008, 0.0012);
    alt_U_per_L = std::clamp(alt_U_per_L, 10.0, 40.0);
    ast_U_per_L = std::clamp(ast_U_per_L, 10.0, 40.0);
    bilirubin_mg_per_dL = std::clamp(bilirubin_mg_per_dL, 0.3, 1.2);

    // --- Blood Interaction ---
    Blood& blood = patient.blood;
    // 1. Toxin Filtration
    double toxinFiltrationRate = 0.1 * totalMetabolicCapacity * deltaTime_s; // a.u. per second
    double toxinsRemoved = blood.toxins_au * toxinFiltrationRate;
    blood.toxins_au -= toxinsRemoved;
    blood.toxins_au = std::max(0.0, blood.toxins_au);

    // 2. Glucose regulation (Gluconeogenesis / Glycogenolysis)
    const double highGlucose = 120.0;
    const double lowGlucose = 80.0;
    if (patient.blood.glucose_mg_per_dL > highGlucose) {
        patient.blood.glucose_mg_per_dL -= (patient.blood.glucose_mg_per_dL - highGlucose) * 0.1 * totalMetabolicCapacity * deltaTime_s;
    } else if (patient.blood.glucose_mg_per_dL < lowGlucose) {
        patient.blood.glucose_mg_per_dL += (lowGlucose - patient.blood.glucose_mg_per_dL) * 0.1 * totalMetabolicCapacity * deltaTime_s;
    }
}

std::string Liver::getSummary() const {
    std::stringstream ss;
    ss.precision(3);
    ss << std::fixed;
    ss << "--- Liver Summary ---\n"
       << "Bile Production: " << getBileProductionRate() * 60.0 << " mL/min\n"
       << "Glucose Production: " << getGlucoseProductionRate() * 60.0 << " g/min\n"
       << "ALT Level: " << getAltLevel() << " U/L\n"
       << "AST Level: " << getAstLevel() << " U/L\n"
       << "Bilirubin: " << getBilirubinLevel() << " mg/dL\n";
    return ss.str();
}

// --- Getters Implementation ---
double Liver::getBileProductionRate() const { return bileProductionRate_ml_per_s; }
double Liver::getGlucoseProductionRate() const { return glucoseProductionRate_g_per_s; }
double Liver::getAltLevel() const { return alt_U_per_L; }
double Liver::getAstLevel() const { return ast_U_per_L; }
double Liver::getBilirubinLevel() const { return bilirubin_mg_per_dL; }
double Liver::getAngiotensinogenRate() const { return angiotensinogen_production_rate; }
