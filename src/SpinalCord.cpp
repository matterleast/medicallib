#include "MedicalLib/SpinalCord.h"
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

SpinalCord::SpinalCord(int id)
    : Organ(id, "SpinalCord"),
      reflexArcIntact(true) {

    // Initialize major pathways
    descendingMotorTract = {"Descending Motor Tract", SignalStatus::NORMAL, 75.0};
    ascendingSensoryTract = {"Ascending Sensory Tract", SignalStatus::NORMAL, 65.0};
}

void SpinalCord::update(Patient& patient, double deltaTime_s) {
    // In a healthy state, status doesn't change.
    // Pathology models would alter these values.
    // For now, we just simulate minor fluctuations in a healthy velocity.
    descendingMotorTract.conductionVelocity_m_per_s += getFluctuation(0.1);
    descendingMotorTract.conductionVelocity_m_per_s = std::clamp(descendingMotorTract.conductionVelocity_m_per_s, 70.0, 80.0);

    ascendingSensoryTract.conductionVelocity_m_per_s += getFluctuation(0.1);
    ascendingSensoryTract.conductionVelocity_m_per_s = std::clamp(ascendingSensoryTract.conductionVelocity_m_per_s, 60.0, 70.0);

    // Reflex arc is a simple boolean for now.
    reflexArcIntact = (descendingMotorTract.status == SignalStatus::NORMAL && ascendingSensoryTract.status == SignalStatus::NORMAL);
}

std::string SpinalCord::statusToString(SignalStatus status) const {
    switch (status) {
        case SignalStatus::NORMAL: return "Normal";
        case SignalStatus::IMPAIRED: return "Impaired";
        case SignalStatus::SEVERED: return "Severed";
        default: return "Unknown";
    }
}

std::string SpinalCord::getSummary() const {
    std::stringstream ss;
    ss << "--- Spinal Cord Summary ---\n"
       << "Motor Pathway (" << descendingMotorTract.name << "): "
       << statusToString(descendingMotorTract.status) << " ("
       << std::fixed << std::setprecision(1) << descendingMotorTract.conductionVelocity_m_per_s << " m/s)\n"
       << "Sensory Pathway (" << ascendingSensoryTract.name << "): "
       << statusToString(ascendingSensoryTract.status) << " ("
       << std::fixed << std::setprecision(1) << ascendingSensoryTract.conductionVelocity_m_per_s << " m/s)\n"
       << "Reflex Arc Intact: " << (isReflexArcIntact() ? "Yes" : "No") << "\n";
    return ss.str();
}

// --- Getters Implementation ---
SignalStatus SpinalCord::getMotorPathwayStatus() const { return descendingMotorTract.status; }
SignalStatus SpinalCord::getSensoryPathwayStatus() const { return ascendingSensoryTract.status; }
bool SpinalCord::isReflexArcIntact() const { return reflexArcIntact; }
