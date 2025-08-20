#include "MedicalLib/SpinalCord.h"
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

SpinalCord::SpinalCord(int id) : Organ(id, "SpinalCord"), signalConductionVelocity(70.0) {}

void SpinalCord::update(double deltaTime_s) {
    const double baseline_velocity = 70.0;
    const double theta = 0.01;
    const double velocity_stddev = 0.1;

    signalConductionVelocity += theta * (baseline_velocity - signalConductionVelocity) * deltaTime_s + getFluctuation(velocity_stddev * deltaTime_s);

    signalConductionVelocity = std::clamp(signalConductionVelocity, 60.0, 80.0);
}

std::string SpinalCord::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Signal Velocity: " << signalConductionVelocity << " m/s";
    return ss.str();
}

double SpinalCord::getSignalConductionVelocity() const { return signalConductionVelocity; }
