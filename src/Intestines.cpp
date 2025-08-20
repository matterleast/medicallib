#include "MedicalLib/Intestines.h"
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

Intestines::Intestines(int id) : Organ(id, "Intestines"), nutrientAbsorptionRate(1.0), waterAbsorptionRate(0.1) {}

void Intestines::update(double deltaTime_s) {
    const double baseline_nutrient_abs = 1.0;
    const double baseline_water_abs = 0.1;
    const double theta = 0.05;
    const double nutrient_abs_stddev = 0.02;
    const double water_abs_stddev = 0.005;

    nutrientAbsorptionRate += theta * (baseline_nutrient_abs - nutrientAbsorptionRate) * deltaTime_s + getFluctuation(nutrient_abs_stddev * deltaTime_s);
    waterAbsorptionRate += theta * (baseline_water_abs - waterAbsorptionRate) * deltaTime_s + getFluctuation(water_abs_stddev * deltaTime_s);

    nutrientAbsorptionRate = std::clamp(nutrientAbsorptionRate, 0.8, 1.2);
    waterAbsorptionRate = std::clamp(waterAbsorptionRate, 0.08, 0.12);
}

std::string Intestines::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Nutrient Absorption: " << nutrientAbsorptionRate << "\n"
       << "  Water Absorption: " << waterAbsorptionRate << " ml/s";
    return ss.str();
}

double Intestines::getNutrientAbsorptionRate() const { return nutrientAbsorptionRate; }
double Intestines::getWaterAbsorptionRate() const { return waterAbsorptionRate; }
