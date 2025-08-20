#include "MedicalLib/Pancreas.h"
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

Pancreas::Pancreas(int id) : Organ(id, "Pancreas"), insulinProduction(1.0), glucagonProduction(50.0) {}

void Pancreas::update(double deltaTime_s) {
    const double baseline_insulin = 1.0;
    const double baseline_glucagon = 50.0;
    const double theta = 0.05;
    const double insulin_stddev = 0.01;
    const double glucagon_stddev = 1.0;

    insulinProduction += theta * (baseline_insulin - insulinProduction) * deltaTime_s + getFluctuation(insulin_stddev * deltaTime_s);
    glucagonProduction += theta * (baseline_glucagon - glucagonProduction) * deltaTime_s + getFluctuation(glucagon_stddev * deltaTime_s);

    insulinProduction = std::clamp(insulinProduction, 0.5, 2.0);
    glucagonProduction = std::clamp(glucagonProduction, 40.0, 60.0);
}

std::string Pancreas::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Insulin Production: " << insulinProduction << " units/hr\n"
       << "  Glucagon Production: " << glucagonProduction << " ng/hr";
    return ss.str();
}

double Pancreas::getInsulinProduction() const { return insulinProduction; }
double Pancreas::getGlucagonProduction() const { return glucagonProduction; }
