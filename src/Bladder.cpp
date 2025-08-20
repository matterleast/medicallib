#include "MedicalLib/Bladder.h"
#include "MedicalLib/Patient.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <iomanip>

Bladder::Bladder(int id)
    : Organ(id, "Bladder"),
      currentState(MicturitionState::FILLING),
      currentVolume_mL(50.0),
      pressure_cmH2O(5.0),
      internalSphincterClosed(true) {}

void Bladder::update(Patient& patient, double deltaTime_s) {
    // Simple pressure model: pressure increases with volume
    pressure_cmH2O = (currentVolume_mL / capacity_mL) * 60.0;

    // State machine
    switch (currentState) {
        case MicturitionState::FILLING:
            if (currentVolume_mL > capacity_mL * 0.8 || pressure_cmH2O > pressureThreshold_cmH2O) {
                currentState = MicturitionState::FULL;
            }
            break;

        case MicturitionState::FULL:
            // For demo, automatically start voiding after 10 seconds in FULL state
            static double timeInFullState = 0.0;
            timeInFullState += deltaTime_s;
            if (timeInFullState > 10.0) {
                currentState = MicturitionState::VOIDING;
                internalSphincterClosed = false;
                timeInFullState = 0.0;
            }
            break;

        case MicturitionState::VOIDING:
            double voidingRate_ml_per_s = 15.0;
            currentVolume_mL -= voidingRate_ml_per_s * deltaTime_s;
            if (currentVolume_mL <= 0) {
                currentVolume_mL = 0;
                currentState = MicturitionState::FILLING;
                internalSphincterClosed = true;
            }
            break;
    }
}

void Bladder::addUrine(double amount_ml) {
    if (currentState != MicturitionState::VOIDING) {
        currentVolume_mL += amount_ml;
        currentVolume_mL = std::clamp(currentVolume_mL, 0.0, capacity_mL);
    }
}

std::string Bladder::stateToString(MicturitionState state) const {
    switch (state) {
        case MicturitionState::FILLING: return "Filling";
        case MicturitionState::FULL: return "Full";
        case MicturitionState::VOIDING: return "Voiding";
        default: return "Unknown";
    }
}

std::string Bladder::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Bladder Summary ---\n"
       << "State: " << stateToString(currentState) << "\n"
       << "Volume: " << getVolume() << " / " << capacity_mL << " mL\n"
       << "Pressure: " << getPressure() << " cmH2O\n";
    return ss.str();
}

// --- Getters Implementation ---
double Bladder::getVolume() const { return currentVolume_mL; }
double Bladder::getPressure() const { return pressure_cmH2O; }
MicturitionState Bladder::getCurrentState() const { return currentState; }
