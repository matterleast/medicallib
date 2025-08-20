#include "MedicalLib/Liver.h"
#include <random>
#include <algorithm>
#include <sstream>

// Helper function for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

// Healthy baseline rates converted to per-second
// Bile: 400-800 ml/day -> ~0.0069 ml/s
// Glucose: ~90 g/day -> ~0.001 g/s
Liver::Liver(int id) : Organ(id, "Liver"), bileProductionRate(0.0069), glucoseProductionRate(0.001) {}

void Liver::update(double deltaTime_s) {
    const double baseline_bile_rate = 0.0069;
    const double baseline_glucose_rate = 0.001;
    const double theta = 0.02; // Slow reversion for metabolic rates
    const double bile_stddev = 0.0001;
    const double glucose_stddev = 0.00005;

    bileProductionRate += theta * (baseline_bile_rate - bileProductionRate) * deltaTime_s + getFluctuation(bile_stddev * deltaTime_s);
    glucoseProductionRate += theta * (baseline_glucose_rate - glucoseProductionRate) * deltaTime_s + getFluctuation(glucose_stddev * deltaTime_s);

    bileProductionRate = std::clamp(bileProductionRate, 0.005, 0.009);
    glucoseProductionRate = std::clamp(glucoseProductionRate, 0.0008, 0.0012);
}

std::string Liver::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Bile Production Rate: " << bileProductionRate * 60 << " ml/min\n"
       << "  Glucose Production Rate: " << glucoseProductionRate * 60 << " g/min";
    return ss.str();
}

double Liver::getBileProductionRate() const { return bileProductionRate; }
double Liver::getGlucoseProductionRate() const { return glucoseProductionRate; }
