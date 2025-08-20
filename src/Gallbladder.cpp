#include "MedicalLib/Gallbladder.h"
#include "MedicalLib/Patient.h"
#include "MedicalLib/Liver.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <iomanip>

Gallbladder::Gallbladder(int id)
    : Organ(id, "Gallbladder"),
      currentState(GallbladderState::STORING),
      storedBile_mL(30.0),
      bileConcentrationFactor(5.0) {}

void Gallbladder::update(Patient& patient, double deltaTime_s) {
    // Get bile from the liver
    if (const Liver* liver = getOrgan<Liver>(patient)) {
        double bileProduced_mL = liver->getBileProductionRate() * deltaTime_s;
        storeBile(bileProduced_mL);
    }

    // A real model would also be driven by CCK hormone for contraction.
    // For now, simulate slow storage and concentration, with a periodic contraction.

    switch (currentState) {
        case GallbladderState::STORING:
            // Concentrate bile over time
            bileConcentrationFactor += 0.05 * deltaTime_s;
            bileConcentrationFactor = std::min(10.0, bileConcentrationFactor);

            // For demo, contract every 40 seconds
            static double timeSinceContraction = 0.0;
            timeSinceContraction += deltaTime_s;
            if (timeSinceContraction > 40.0) {
                currentState = GallbladderState::CONTRACTING;
                timeSinceContraction = 0.0;
            }
            break;

        case GallbladderState::CONTRACTING:
            // Release bile
            double releasedBile = 2.0 * deltaTime_s;
            storedBile_mL -= releasedBile;

            if (storedBile_mL < 5.0) { // Stop contracting when near empty
                storedBile_mL = std::max(0.0, storedBile_mL);
                bileConcentrationFactor = 1.0; // Bile is fresh
                currentState = GallbladderState::STORING;
            }
            break;
    }
}

void Gallbladder::storeBile(double volume_mL) {
    if (currentState == GallbladderState::STORING) {
        storedBile_mL += volume_mL;
        storedBile_mL = std::clamp(storedBile_mL, 0.0, capacity_mL);
    }
}

std::string Gallbladder::stateToString(GallbladderState state) const {
    switch (state) {
        case GallbladderState::STORING: return "Storing/Concentrating";
        case GallbladderState::CONTRACTING: return "Contracting (Releasing)";
        default: return "Unknown";
    }
}

std::string Gallbladder::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Gallbladder Summary ---\n"
       << "State: " << stateToString(currentState) << "\n"
       << "Volume: " << getStoredBileVolume() << " / " << capacity_mL << " mL\n"
       << "Concentration: " << getBileConcentration() << "x\n";
    return ss.str();
}

// --- Getters Implementation ---
double Gallbladder::getStoredBileVolume() const { return storedBile_mL; }
double Gallbladder::getBileConcentration() const { return bileConcentrationFactor; }
GallbladderState Gallbladder::getCurrentState() const { return currentState; }
