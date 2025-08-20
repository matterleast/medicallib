#include "MedicalLib/Esophagus.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <iomanip>

Esophagus::Esophagus(int id)
    : Organ(id, "Esophagus"),
      currentState(PeristalsisState::IDLE),
      lowerEsophagealSphincterTone(20.0) {}

void Esophagus::update(double deltaTime_s) {
    // Simulate a swallow every 15 seconds for demonstration
    static double timeSinceLastSwallow = 0.0;
    timeSinceLastSwallow += deltaTime_s;
    if (timeSinceLastSwallow > 15.0) {
        initiateSwallow(15.0); // Swallow a 15mL bolus
        timeSinceLastSwallow = 0.0;
    }

    if (!activeBoli.empty()) {
        currentState = PeristalsisState::CONTRACTING;
        const double peristalsisSpeed_cm_per_s = 3.0;

        for (auto& bolus : activeBoli) {
            bolus.position_cm += peristalsisSpeed_cm_per_s * deltaTime_s;
        }

        // Remove boli that have passed into the stomach
        activeBoli.erase(std::remove_if(activeBoli.begin(), activeBoli.end(),
            [this](const Bolus& b) { return b.position_cm >= this->length_cm; }),
            activeBoli.end());

        if (activeBoli.empty()) {
            currentState = PeristalsisState::IDLE;
        }
    } else {
        currentState = PeristalsisState::IDLE;
    }

    // Fluctuate LES tone slightly
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, 0.1);
    lowerEsophagealSphincterTone += d(gen) * deltaTime_s;
    lowerEsophagealSphincterTone = std::clamp(lowerEsophagealSphincterTone, 18.0, 25.0);
}

void Esophagus::initiateSwallow(double bolusVolume_mL) {
    activeBoli.push_back({bolusVolume_mL, 0.0});
}

std::string Esophagus::stateToString(PeristalsisState state) const {
    switch (state) {
        case PeristalsisState::IDLE: return "Idle";
        case PeristalsisState::CONTRACTING: return "Contracting";
        case PeristalsisState::RELAXING: return "Relaxing";
        default: return "Unknown";
    }
}

std::string Esophagus::getSummary() const {
    std::stringstream ss;
    ss << "--- Esophagus Summary ---\n"
       << "State: " << stateToString(currentState) << "\n"
       << "LES Tone: " << std::fixed << std::setprecision(1) << lowerEsophagealSphincterTone << " mmHg\n"
       << "Boluses in transit: " << activeBoli.size() << "\n";
    if (!activeBoli.empty()) {
        ss << "  - Top bolus position: " << activeBoli.front().position_cm << " / " << length_cm << " cm\n";
    }
    return ss.str();
}

PeristalsisState Esophagus::getCurrentState() const { return currentState; }
bool Esophagus::isSwallowing() const { return !activeBoli.empty(); }
