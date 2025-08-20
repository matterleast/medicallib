#include "MedicalLib/Spleen.h"
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

Spleen::Spleen(int id) : Organ(id, "Spleen"), redBloodCellCount(4.5), whiteBloodCellCount(7.5) {}

void Spleen::update(double deltaTime_s) {
    const double baseline_rbc = 4.5;
    const double baseline_wbc = 7.5;
    const double theta = 0.02;
    const double rbc_stddev = 0.01;
    const double wbc_stddev = 0.05;

    redBloodCellCount += theta * (baseline_rbc - redBloodCellCount) * deltaTime_s + getFluctuation(rbc_stddev * deltaTime_s);
    whiteBloodCellCount += theta * (baseline_wbc - whiteBloodCellCount) * deltaTime_s + getFluctuation(wbc_stddev * deltaTime_s);

    redBloodCellCount = std::clamp(redBloodCellCount, 4.0, 5.0);
    whiteBloodCellCount = std::clamp(whiteBloodCellCount, 5.0, 10.0);
}

std::string Spleen::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  RBC Count: " << redBloodCellCount << " million/uL\n"
       << "  WBC Count: " << whiteBloodCellCount << " thousand/uL";
    return ss.str();
}

double Spleen::getRedBloodCellCount() const { return redBloodCellCount; }
double Spleen::getWhiteBloodCellCount() const { return whiteBloodCellCount; }
