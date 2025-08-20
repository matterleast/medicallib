#include "MedicalLib/Brain.h"
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

Brain::Brain(int id) : Organ(id, "Brain"), consciousnessLevel(1.0), cerebralBloodFlow(50.0) {}

void Brain::update(double deltaTime_s) {
    const double baseline_consciousness = 1.0;
    const double baseline_cbf = 50.0;
    const double theta = 0.05; // Slower reversion for brain metrics
    const double consciousness_stddev = 0.001;
    const double cbf_stddev = 0.5;

    consciousnessLevel += theta * (baseline_consciousness - consciousnessLevel) * deltaTime_s + getFluctuation(consciousness_stddev * deltaTime_s);
    cerebralBloodFlow += theta * (baseline_cbf - cerebralBloodFlow) * deltaTime_s + getFluctuation(cbf_stddev * deltaTime_s);

    consciousnessLevel = std::clamp(consciousnessLevel, 0.0, 1.0);
    cerebralBloodFlow = std::clamp(cerebralBloodFlow, 40.0, 60.0);
}

std::string Brain::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Consciousness Level: " << consciousnessLevel * 100 << "%\n"
       << "  Cerebral Blood Flow: " << cerebralBloodFlow << " ml/100g/min";
    return ss.str();
}

double Brain::getConsciousnessLevel() const { return consciousnessLevel; }
double Brain::getCerebralBloodFlow() const { return cerebralBloodFlow; }
