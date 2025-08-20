#include "MedicalLib/Gallbladder.h"
#include <random>
#include <algorithm>
#include <sstream>

Gallbladder::Gallbladder(int id) : Organ(id, "Gallbladder"), bileStored(30.0) {}

void Gallbladder::update(double deltaTime_s) {
    // A real model would be affected by liver production and digestion.
    // For now, it just sits there with a small fluctuation.
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, 0.01);

    bileStored += d(gen) * deltaTime_s;
    bileStored = std::clamp(bileStored, 10.0, 50.0);
}

std::string Gallbladder::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Bile Stored: " << bileStored << " ml";
    return ss.str();
}

double Gallbladder::getBileStored() const { return bileStored; }
