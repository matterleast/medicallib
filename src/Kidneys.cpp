#include "MedicalLib/Kidneys.h"
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

Kidneys::Kidneys(int id) : Organ(id, "Kidneys"), filtrationRate(125.0), urineProductionRate(0.02) {}

void Kidneys::update(double deltaTime_s) {
    const double baseline_filtration = 125.0;
    const double baseline_urine_prod = 0.02;
    const double theta = 0.03;
    const double filtration_stddev = 1.0;
    const double urine_prod_stddev = 0.001;

    filtrationRate += theta * (baseline_filtration - filtrationRate) * deltaTime_s + getFluctuation(filtration_stddev * deltaTime_s);
    urineProductionRate += theta * (baseline_urine_prod - urineProductionRate) * deltaTime_s + getFluctuation(urine_prod_stddev * deltaTime_s);

    filtrationRate = std::clamp(filtrationRate, 100.0, 150.0);
    urineProductionRate = std::clamp(urineProductionRate, 0.01, 0.03);
}

std::string Kidneys::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Filtration Rate: " << filtrationRate << " ml/min\n"
       << "  Urine Production: " << urineProductionRate << " ml/s";
    return ss.str();
}

double Kidneys::getFiltrationRate() const { return filtrationRate; }
double Kidneys::getUrineProductionRate() const { return urineProductionRate; }
