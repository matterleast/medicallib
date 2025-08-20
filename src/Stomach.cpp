#include "MedicalLib/Stomach.h"
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

Stomach::Stomach(int id) : Organ(id, "Stomach"), phLevel(2.0), digestionRate(1.0) {}

void Stomach::update(double deltaTime_s) {
    // For now, these values just fluctuate around a baseline.
    const double baseline_ph = 2.0;
    const double baseline_digestion = 1.0;
    const double theta = 0.1;
    const double ph_stddev = 0.05;
    const double digestion_stddev = 0.02;

    phLevel += theta * (baseline_ph - phLevel) * deltaTime_s + getFluctuation(ph_stddev * deltaTime_s);
    digestionRate += theta * (baseline_digestion - digestionRate) * deltaTime_s + getFluctuation(digestion_stddev * deltaTime_s);

    phLevel = std::clamp(phLevel, 1.5, 3.5);
    digestionRate = std::clamp(digestionRate, 0.5, 1.5);
}

std::string Stomach::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  pH Level: " << phLevel << "\n"
       << "  Digestion Rate: " << digestionRate;
    return ss.str();
}

double Stomach::getPhLevel() const { return phLevel; }
double Stomach::getDigestionRate() const { return digestionRate; }
