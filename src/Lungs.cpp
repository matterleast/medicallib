#include "MedicalLib/Lungs.h"
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

Lungs::Lungs(int id) : Organ(id, "Lungs"), respirationRate(16.0), oxygenSaturation(98.0) {}

void Lungs::update(double deltaTime_s) {
    const double baseline_rr = 16.0;
    const double baseline_spo2 = 98.0;
    const double theta = 0.1; // Mean reversion speed
    const double rr_stddev = 0.05;
    const double spo2_stddev = 0.02;

    respirationRate += theta * (baseline_rr - respirationRate) * deltaTime_s + getFluctuation(rr_stddev * deltaTime_s);
    oxygenSaturation += theta * (baseline_spo2 - oxygenSaturation) * deltaTime_s + getFluctuation(spo2_stddev * deltaTime_s);

    respirationRate = std::clamp(respirationRate, 12.0, 20.0);
    oxygenSaturation = std::clamp(oxygenSaturation, 96.0, 100.0);
}

std::string Lungs::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Respiration Rate: " << respirationRate << " breaths/min\n"
       << "  Oxygen Saturation: " << oxygenSaturation << " %";
    return ss.str();
}

double Lungs::getRespirationRate() const { return respirationRate; }
double Lungs::getOxygenSaturation() const { return oxygenSaturation; }
