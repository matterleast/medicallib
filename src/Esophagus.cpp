#include "MedicalLib/Esophagus.h"
#include <random>
#include <algorithm>
#include <sstream>

Esophagus::Esophagus(int id) : Organ(id, "Esophagus"), motility(1.0) {}

void Esophagus::update(double deltaTime_s) {
    // Placeholder, no real logic yet.
    // Fluctuate motility slightly.
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, 0.001);

    motility += d(gen) * deltaTime_s;
    motility = std::clamp(motility, 0.95, 1.05);
}

std::string Esophagus::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Motility: " << motility * 100 << "%";
    return ss.str();
}

double Esophagus::getMotility() const { return motility; }
