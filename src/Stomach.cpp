#include "MedicalLib/Stomach.h"
#include "MedicalLib/Patient.h"
#include "MedicalLib/Intestines.h"
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

Stomach::Stomach(int id)
    : Organ(id, "Stomach"),
      currentState(GastricState::EMPTY),
      currentVolume_mL(0.0),
      currentPh(4.5), // pH of an empty stomach is higher
      gastricJuiceSecretionRate_ml_per_s(0.1),
      emptyingRate_ml_per_s(0.5) {}

void Stomach::update(Patient& patient, double deltaTime_s) {
    // --- State Machine Logic ---
    switch (currentState) {
        case GastricState::EMPTY:
            // In a real simulation, addSubstance would be called externally (e.g., from Esophagus)
            break;

        case GastricState::FILLING:
            // Transition to DIGESTING after a short period
            static double fillTime = 0.0;
            fillTime += deltaTime_s;
            if (fillTime > 2.0) {
                currentState = GastricState::DIGESTING;
                fillTime = 0.0;
            }
            break;

        case GastricState::DIGESTING:
            // Secrete acid, lowering pH
            currentPh -= 0.5 * deltaTime_s;
            currentPh = std::max(1.5, currentPh);

            // After some time, start emptying
            static double digestionTime = 0.0;
            digestionTime += deltaTime_s;
            if (digestionTime > 30.0) { // Digest for 30 seconds
                currentState = GastricState::EMPTYING;
                digestionTime = 0.0;
            }
            break;

        case GastricState::EMPTYING:
            // Empty chyme into intestines
            double amountToEmpty = emptyingRate_ml_per_s * deltaTime_s;

            if (Intestines* intestines = getOrgan<Intestines>(patient)) {
                intestines->receiveChyme(amountToEmpty);
            }

            currentVolume_mL -= amountToEmpty;

            if (currentVolume_mL <= 0) {
                currentVolume_mL = 0;
                currentState = GastricState::EMPTY;
                currentPh = 4.5; // pH returns to baseline
            }
            break;
    }

    // Secrete a baseline level of gastric juice, increasing during digestion
    double currentSecretionRate = (currentState == GastricState::DIGESTING) ? 2.0 : 0.1;
    currentVolume_mL += currentSecretionRate * deltaTime_s;
    currentVolume_mL = std::clamp(currentVolume_mL, 0.0, capacity_mL);
}

void Stomach::addSubstance(double volume_mL) {
    currentVolume_mL += volume_mL;
    // Food buffers the acid initially
    currentPh = std::min(4.0, currentPh + 0.5);
    currentState = GastricState::FILLING;
}

std::string Stomach::stateToString(GastricState state) const {
    switch (state) {
        case GastricState::EMPTY: return "Empty";
        case GastricState::FILLING: return "Filling";
        case GastricState::DIGESTING: return "Digesting";
        case GastricState::EMPTYING: return "Emptying";
        default: return "Unknown";
    }
}

std::string Stomach::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Stomach Summary ---\n"
       << "State: " << stateToString(currentState) << "\n"
       << "Volume: " << currentVolume_mL << " / " << capacity_mL << " mL\n"
       << "Acidity (pH): " << getAcidity() << "\n";
    return ss.str();
}

// --- Getters Implementation ---
GastricState Stomach::getCurrentState() const { return currentState; }
double Stomach::getVolume() const { return currentVolume_mL; }
double Stomach::getAcidity() const { return currentPh; }
