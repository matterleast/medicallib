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
      bileConcentrationFactor(5.0),
      bileReleaseRate_ml_per_s(2.0) {}

void Gallbladder::update(Patient& patient, double deltaTime_s) {
    // Get bile from the liver
    if (const Liver* liver = getOrgan<Liver>(patient)) {
        double bileProduced_mL = liver->getBileProductionRate() * deltaTime_s;
        storeBile(bileProduced_mL);
    }

    // A real model would also be driven by CCK hormone for contraction.
    // Contraction is now triggered externally by releaseBile().

    switch (currentState) {
        case GallbladderState::STORING:
            // Concentrate bile over time
            bileConcentrationFactor += 0.05 * deltaTime_s;
            bileConcentrationFactor = std::min(10.0, bileConcentrationFactor);
            break;

        case GallbladderState::CONTRACTING:
            // After contracting for a while, or if empty, go back to storing.
            static double contractionTime = 0.0;
            contractionTime += deltaTime_s;

            if (storedBile_mL < 5.0 || contractionTime > 15.0) { // Stop contracting when near empty or after 15s
                storedBile_mL = std::max(0.0, storedBile_mL);
                if (storedBile_mL == 0.0) {
                    bileConcentrationFactor = 1.0; // Bile is fresh if we're totally empty
                }
                currentState = GallbladderState::STORING;
                contractionTime = 0.0;
            }
            break;
    }
}

double Gallbladder::releaseBile(double deltaTime_s) {
    if (storedBile_mL <= 0) {
        return 0.0;
    }

    currentState = GallbladderState::CONTRACTING;
    double amountToRelease = bileReleaseRate_ml_per_s * deltaTime_s;
    amountToRelease = std::min(amountToRelease, storedBile_mL); // Don't release more than we have
    storedBile_mL -= amountToRelease;

    return amountToRelease;
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
