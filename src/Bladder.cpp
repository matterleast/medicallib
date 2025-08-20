#include "MedicalLib/Bladder.h"
#include <random>
#include <algorithm>
#include <sstream>

Bladder::Bladder(int id) : Organ(id, "Bladder"), currentVolume(50.0), capacity(500.0) {}

void Bladder::update(double deltaTime_s) {
    // In a more complex model, this would be driven by kidney output.
    // For now, simulate a constant fill rate.
    const double fillRate_ml_per_s = 0.02;
    currentVolume += fillRate_ml_per_s * deltaTime_s;
    currentVolume = std::clamp(currentVolume, 0.0, capacity);
}

void Bladder::addUrine(double amount_ml) {
    currentVolume += amount_ml;
    currentVolume = std::clamp(currentVolume, 0.0, capacity);
}

std::string Bladder::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Volume: " << currentVolume << " / " << capacity << " ml";
    return ss.str();
}

double Bladder::getCurrentVolume() const { return currentVolume; }
double Bladder::getCapacity() const { return capacity; }
