#include "MedicalLib/Lungs.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <cmath>
#include <numeric>

// Helper for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Lungs::Lungs(int id)
    : Organ(id, "Lungs"),
      respirationRate(16.0),
      oxygenSaturation(98.0),
      tidalVolume_mL(500.0),
      endTidalCO2_mmHg(40.0),
      peakInspiratoryPressure_cmH2O(0.0),
      totalLungCapacity_mL(6000.0),
      currentState(RespiratoryState::PAUSE),
      cyclePosition_s(0.0),
      totalTime_s(0.0),
      capnographyHistorySize(200) {

    // Initialize Lobes
    rightUpperLobe = {"Right Upper Lobe", 0, 0.1};
    rightMiddleLobe = {"Right Middle Lobe", 0, 0.07};
    rightLowerLobe = {"Right Lower Lobe", 0, 0.13};
    leftUpperLobe = {"Left Upper Lobe", 0, 0.1};
    leftLowerLobe = {"Left Lower Lobe", 0, 0.1};

    // Initialize Bronchi
    mainBronchus = {"Main Bronchus", 0.8};
}

void Lungs::update(double deltaTime_s) {
    totalTime_s += deltaTime_s;

    updateRespiratoryMechanics(deltaTime_s);
    updateGasLevels(deltaTime_s);

    // Update capnography data
    capnographyData.push_front(generateCapnographyValue());
    if (capnographyData.size() > capnographyHistorySize) {
        capnographyData.pop_back();
    }
}

void Lungs::updateRespiratoryMechanics(double deltaTime_s) {
    double cycleDuration_s = 60.0 / respirationRate;
    cyclePosition_s += deltaTime_s;

    // State transitions
    double inspirationDuration = cycleDuration_s * 0.4; // I:E ratio of 1:1.5
    double expirationDuration = cycleDuration_s * 0.6;

    if (cyclePosition_s <= inspirationDuration) {
        currentState = RespiratoryState::INSPIRATION;
    } else if (cyclePosition_s <= cycleDuration_s) {
        currentState = RespiratoryState::EXPIRATION;
    } else {
        cyclePosition_s -= cycleDuration_s;
        currentState = RespiratoryState::INSPIRATION;
    }

    // Pressure and Volume dynamics
    double flowRate_mL_s = 0;
    if (currentState == RespiratoryState::INSPIRATION) {
        // Simple sine wave for pressure generation
        double pressure_wave = sin(M_PI * (cyclePosition_s / inspirationDuration));
        peakInspiratoryPressure_cmH2O = 15.0 * pressure_wave; // 15 cmH2O peak
        flowRate_mL_s = (peakInspiratoryPressure_cmH2O / mainBronchus.resistance) * 100;
        tidalVolume_mL += flowRate_mL_s * deltaTime_s;
    } else { // EXPIRATION
        peakInspiratoryPressure_cmH2O = 0;
        // Passive recoil drives expiration
        double recoilPressure = (tidalVolume_mL / 500.0) * 5.0; // Simplified
        flowRate_mL_s = -(recoilPressure / mainBronchus.resistance) * 100;
        tidalVolume_mL += flowRate_mL_s * deltaTime_s;
    }

    // Clamp tidal volume
    tidalVolume_mL = std::clamp(tidalVolume_mL, 0.0, totalLungCapacity_mL / 2.0);
}

void Lungs::updateGasLevels(double deltaTime_s) {
    // Fluctuate base vitals slightly
    respirationRate += getFluctuation(0.01);
    respirationRate = std::clamp(respirationRate, 12.0, 20.0);

    // SpO2 is affected by how well we are breathing
    double ventilationFactor = (tidalVolume_mL / 500.0) * (respirationRate / 16.0);
    double targetSpo2 = 98.0 * std::clamp(ventilationFactor, 0.9, 1.0);
    oxygenSaturation += 0.1 * (targetSpo2 - oxygenSaturation) * deltaTime_s + getFluctuation(0.02);
    oxygenSaturation = std::clamp(oxygenSaturation, 94.0, 100.0);

    // etCO2 is inversely related to ventilation
    double targetEtCO2 = 40.0 / std::clamp(ventilationFactor, 0.8, 1.2);
    endTidalCO2_mmHg += 0.2 * (targetEtCO2 - endTidalCO2_mmHg) * deltaTime_s + getFluctuation(0.05);
    endTidalCO2_mmHg = std::clamp(endTidalCO2_mmHg, 35.0, 50.0);
}

double Lungs::generateCapnographyValue() {
    double cycleDuration_s = 60.0 / respirationRate;
    double timeInCycle = fmod(cyclePosition_s, cycleDuration_s);
    double inspirationEnd = cycleDuration_s * 0.4;
    double plateauStart = cycleDuration_s * 0.5;
    double plateauEnd = cycleDuration_s * 0.8;

    if (currentState == RespiratoryState::INSPIRATION) {
        return 0.0; // Phase I: Inspiratory baseline
    } else { // EXPIRATION
        if (timeInCycle < plateauStart) { // Phase II: Expiratory upstroke
            return endTidalCO2_mmHg * ((timeInCycle - inspirationEnd) / (plateauStart - inspirationEnd));
        } else if (timeInCycle < plateauEnd) { // Phase III: Alveolar plateau
            return endTidalCO2_mmHg + getFluctuation(0.1);
        } else { // Phase IV: Inspiratory downstroke (handled by next cycle's baseline)
            return endTidalCO2_mmHg * (1.0 - (timeInCycle - plateauEnd) / (cycleDuration_s - plateauEnd));
        }
    }
}

std::string Lungs::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Lungs Summary ---\n"
       << "Respiration Rate: " << getRespirationRate() << " breaths/min\n"
       << "Oxygen Saturation (SpO2): " << getOxygenSaturation() << " %\n"
       << "Tidal Volume: " << getTidalVolume() << " mL\n"
       << "End-Tidal CO2 (etCO2): " << getEndTidalCO2() << " mmHg\n"
       << "Peak Airway Pressure: " << getPeakInspiratoryPressure() << " cmH2O\n";
    return ss.str();
}

// --- Getters Implementation ---
double Lungs::getRespirationRate() const { return respirationRate; }
double Lungs::getOxygenSaturation() const { return oxygenSaturation; }
double Lungs::getTidalVolume() const { return tidalVolume_mL; }
double Lungs::getEndTidalCO2() const { return endTidalCO2_mmHg; }
double Lungs::getPeakInspiratoryPressure() const { return peakInspiratoryPressure_cmH2O; }
const std::deque<double>& Lungs::getCapnographyWaveform() const { return capnographyData; }
