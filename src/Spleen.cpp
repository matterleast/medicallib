#include "MedicalLib/Spleen.h"
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

Spleen::Spleen(int id) : Organ(id, "Spleen") {
    // Initialize pulp components
    redPulp = {1.0, 0.5};
    whitePulp = {1500.0, 500.0};
}

void Spleen::update(Patient& patient, double deltaTime_s) {
    // In a real model, these values would change in response to infection or disease.
    // For now, we just simulate minor fluctuations around a healthy baseline.

    // Red pulp fluctuations
    redPulp.filtrationRate += getFluctuation(0.01);
    redPulp.rbcBreakdownRate += getFluctuation(0.005);

    // White pulp fluctuations
    whitePulp.lymphocyteCount += getFluctuation(1.0);
    whitePulp.macrophageCount += getFluctuation(0.5);

    // Clamp to healthy ranges
    redPulp.filtrationRate = std::clamp(redPulp.filtrationRate, 0.9, 1.1);
    redPulp.rbcBreakdownRate = std::clamp(redPulp.rbcBreakdownRate, 0.45, 0.55);
    whitePulp.lymphocyteCount = std::clamp(whitePulp.lymphocyteCount, 1400.0, 1600.0);
    whitePulp.macrophageCount = std::clamp(whitePulp.macrophageCount, 450.0, 550.0);
}

std::string Spleen::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Spleen Summary ---\n"
       << "--- Red Pulp ---\n"
       << "Filtration Rate: " << redPulp.filtrationRate << "\n"
       << "RBC Breakdown Rate: " << getRbcBreakdownRate() << "\n"
       << "--- White Pulp ---\n"
       << "Lymphocyte Count: " << getLymphocyteCount() << " million\n"
       << "Macrophage Count: " << whitePulp.macrophageCount << " million\n";
    return ss.str();
}

// --- Getters Implementation ---
double Spleen::getRbcBreakdownRate() const { return redPulp.rbcBreakdownRate; }
double Spleen::getLymphocyteCount() const { return whitePulp.lymphocyteCount; }
