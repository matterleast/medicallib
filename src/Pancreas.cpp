#include "MedicalLib/Pancreas.h"
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

Pancreas::Pancreas(int id)
    : Organ(id, "Pancreas"),
      insulinSecretion_units_per_hr(1.0),
      glucagonSecretion_ng_per_hr(50.0),
      amylaseSecretion_U_per_L(80.0),
      lipaseSecretion_U_per_L(40.0) {}

void Pancreas::update(Patient& patient, double deltaTime_s) {
    // Hormone secretion is driven by blood glucose.
    const double glucose = patient.blood.glucose_mg_per_dL;
    const double highGlucoseThreshold = 120.0;
    const double lowGlucoseThreshold = 80.0;

    // Insulin response
    if (glucose > highGlucoseThreshold) {
        insulinSecretion_units_per_hr += (glucose - highGlucoseThreshold) * 0.1 * deltaTime_s;
    } else {
        insulinSecretion_units_per_hr -= 0.5 * deltaTime_s;
    }

    // Glucagon response
    if (glucose < lowGlucoseThreshold) {
        glucagonSecretion_ng_per_hr += (lowGlucoseThreshold - glucose) * 0.2 * deltaTime_s;
    } else {
        glucagonSecretion_ng_per_hr -= 1.0 * deltaTime_s;
    }

    // Enzyme secretion would be driven by food in the duodenum (not yet modeled).
    // For now, we just simulate minor fluctuations around a baseline.
    amylaseSecretion_U_per_L += getFluctuation(0.2);
    lipaseSecretion_U_per_L += getFluctuation(0.2);

    // Clamp to healthy/possible ranges
    insulinSecretion_units_per_hr = std::clamp(insulinSecretion_units_per_hr, 0.5, 10.0);
    glucagonSecretion_ng_per_hr = std::clamp(glucagonSecretion_ng_per_hr, 20.0, 100.0);
    amylaseSecretion_U_per_L = std::clamp(amylaseSecretion_U_per_L, 60.0, 100.0);
    lipaseSecretion_U_per_L = std::clamp(lipaseSecretion_U_per_L, 20.0, 60.0);
}

std::string Pancreas::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Pancreas Summary ---\n"
       << "--- Endocrine Function ---\n"
       << "Insulin Secretion: " << getInsulinSecretion() << " units/hr\n"
       << "Glucagon Secretion: " << getGlucagonSecretion() << " ng/hr\n"
       << "--- Exocrine Function ---\n"
       << "Amylase Secretion: " << getAmylaseSecretion() << " U/L\n"
       << "Lipase Secretion: " << getLipaseSecretion() << " U/L\n";
    return ss.str();
}

// --- Getters Implementation ---
double Pancreas::getInsulinSecretion() const { return insulinSecretion_units_per_hr; }
double Pancreas::getGlucagonSecretion() const { return glucagonSecretion_ng_per_hr; }
double Pancreas::getAmylaseSecretion() const { return amylaseSecretion_U_per_L; }
double Pancreas::getLipaseSecretion() const { return lipaseSecretion_U_per_L; }
