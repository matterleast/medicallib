#include "MedicalLib/Brain.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <cmath>

// Helper for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Brain::Brain(int id)
    : Organ(id, "Brain"),
      gcsScore(15),
      intracranialPressure_mmHg(10.0),
      cerebralPerfusionPressure_mmHg(80.0),
      meanArterialPressure_mmHg(90.0), // Placeholder value
      totalTime_s(0.0),
      eegHistorySize(200) {

    // Initialize Brain Regions
    frontalLobe = {"Frontal Lobe", 0.8, 50.0};
    temporalLobe = {"Temporal Lobe", 0.7, 50.0};
    parietalLobe = {"Parietal Lobe", 0.7, 50.0};
    occipitalLobe = {"Occipital Lobe", 0.8, 55.0};
    cerebellum = {"Cerebellum", 0.6, 60.0};
}

void Brain::update(double deltaTime_s) {
    totalTime_s += deltaTime_s;

    // In a real scenario, MAP would be provided by the circulatory system (Heart)
    // For now, we'll just keep it stable with minor fluctuations.
    meanArterialPressure_mmHg += getFluctuation(0.1);
    meanArterialPressure_mmHg = std::clamp(meanArterialPressure_mmHg, 85.0, 95.0);

    updateActivity(deltaTime_s);
    updatePressures(meanArterialPressure_mmHg);

    // Update EEG data
    eegData.push_front(generateEegValue());
    if (eegData.size() > eegHistorySize) {
        eegData.pop_back();
    }
}

void Brain::updateActivity(double deltaTime_s) {
    // Simulate minor fluctuations in brain activity
    frontalLobe.activityLevel += getFluctuation(0.005);
    frontalLobe.activityLevel = std::clamp(frontalLobe.activityLevel, 0.7, 0.9);

    // GCS is a clinical score, doesn't typically change second-to-second.
    // This is a placeholder for future, more complex pathology simulation.
    gcsScore = 15;
}

void Brain::updatePressures(double meanArterialPressure) {
    // Autoregulation: Brain tries to maintain constant CPP
    // Simplified model: ICP drifts slowly
    intracranialPressure_mmHg += getFluctuation(0.01);
    intracranialPressure_mmHg = std::clamp(intracranialPressure_mmHg, 8.0, 12.0);

    cerebralPerfusionPressure_mmHg = meanArterialPressure - intracranialPressure_mmHg;
    cerebralPerfusionPressure_mmHg = std::max(0.0, cerebralPerfusionPressure_mmHg);
}

double Brain::generateEegValue() {
    // Super simplified EEG: combination of a few sine waves (alpha, beta)
    double alpha_wave = 0.5 * sin(2 * M_PI * 10 * totalTime_s); // 10 Hz
    double beta_wave = 0.3 * sin(2 * M_PI * 20 * totalTime_s);  // 20 Hz
    double noise = getFluctuation(0.1);
    return (alpha_wave + beta_wave + noise) * 20; // Scaled to microvolts
}

std::string Brain::getSummary() const {
    std::stringstream ss;
    ss.precision(1);
    ss << std::fixed;
    ss << "--- Brain Summary ---\n"
       << "Glasgow Coma Scale (GCS): " << getGCS() << "\n"
       << "Intracranial Pressure (ICP): " << getIntracranialPressure() << " mmHg\n"
       << "Mean Arterial Pressure (MAP): " << meanArterialPressure_mmHg << " mmHg\n"
       << "Cerebral Perfusion (CPP): " << getCerebralPerfusionPressure() << " mmHg\n";
    return ss.str();
}

// --- Getters Implementation ---
int Brain::getGCS() const { return gcsScore; }
double Brain::getIntracranialPressure() const { return intracranialPressure_mmHg; }
double Brain::getCerebralPerfusionPressure() const { return cerebralPerfusionPressure_mmHg; }
const std::deque<double>& Brain::getEegWaveform() const { return eegData; }
