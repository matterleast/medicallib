#include "MedicalLib/Brain.h"
#include "MedicalLib/Patient.h" // For Blood struct
#include "MedicalLib/Heart.h"   // For Heart data
#include "MedicalLib/Lungs.h"   // For setting respiration
#include "MedicalLib/SpinalCord.h" // For SpinalCord data
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
      gcsEye(4),
      gcsVerbal(5),
      gcsMotor(6),
      intracranialPressure_mmHg(10.0),
      cerebralPerfusionPressure_mmHg(80.0),
      meanArterialPressure_mmHg(90.0), // Placeholder value
      totalTime_s(0.0),
      targetRespirationRate_bpm(16.0), // Normal baseline
      targetHeartRate_bpm(75.0),       // Normal baseline
      eegHistorySize(200) {

    // Initialize Brain Regions
    frontalLobe = {"Frontal Lobe", 0.8, 50.0};
    temporalLobe = {"Temporal Lobe", 0.7, 50.0};
    parietalLobe = {"Parietal Lobe", 0.7, 50.0};
    occipitalLobe = {"Occipital Lobe", 0.8, 55.0};
    cerebellum = {"Cerebellum", 0.6, 60.0};
}

void Brain::update(Patient& patient, double deltaTime_s) {
    totalTime_s += deltaTime_s;

    // Get Mean Arterial Pressure from the Heart
    if (const Heart* heart = getOrgan<Heart>(patient)) {
        meanArterialPressure_mmHg = heart->getAorticPressure();
    } else {
        // If no heart, use a default stable value
        meanArterialPressure_mmHg += getFluctuation(0.1);
        meanArterialPressure_mmHg = std::clamp(meanArterialPressure_mmHg, 85.0, 95.0);
    }

    updateActivity(deltaTime_s);
    updatePressures(meanArterialPressure_mmHg);
    updateAutonomicControl(patient, deltaTime_s);

    // Update EEG data
    eegData.push_front(generateEegValue());
    if (eegData.size() > eegHistorySize) {
        eegData.pop_back();
    }

    // --- Blood Interaction ---
    Blood& blood = patient.blood;
    // Brain consumes O2 and produces CO2. Rate depends on activity.
    double totalActivity = (frontalLobe.activityLevel + temporalLobe.activityLevel +
                           parietalLobe.activityLevel + occipitalLobe.activityLevel +
                           cerebellum.activityLevel) / 5.0;

    // O2 consumption
    double o2_consumption = 0.1 * totalActivity * deltaTime_s; // % per second
    blood.oxygenSaturation -= o2_consumption;

    // CO2 production
    double co2_production = 0.08 * totalActivity * deltaTime_s; // mmHg per second
    blood.co2PartialPressure_mmHg += co2_production;

    // Update GCS based on physiological parameters
    updateGCS(patient);
}

void Brain::updateActivity(double deltaTime_s) {
    // Simulate minor fluctuations in brain activity
    frontalLobe.activityLevel += getFluctuation(0.005);
    frontalLobe.activityLevel = std::clamp(frontalLobe.activityLevel, 0.7, 0.9);
}

void Brain::updateGCS(const Patient& patient) {
    // This is a more detailed model for GCS, breaking it down into components.
    // The scores are based on physiological drivers like hypoxia, hypercapnia, and perfusion.
    const Blood& blood = patient.blood;

    // --- Eye Response (1-4) ---
    if (blood.oxygenSaturation > 94.0 && cerebralPerfusionPressure_mmHg > 60) {
        gcsEye = 4; // Spontaneous
    } else if (blood.oxygenSaturation > 90.0 && cerebralPerfusionPressure_mmHg > 55) {
        gcsEye = 3; // To sound
    } else if (blood.oxygenSaturation > 80.0 || cerebralPerfusionPressure_mmHg > 50) {
        gcsEye = 2; // To pain
    } else {
        gcsEye = 1; // None
    }

    // --- Verbal Response (1-5) ---
    if (blood.co2PartialPressure_mmHg < 45.0 && blood.oxygenSaturation > 94.0) {
        gcsVerbal = 5; // Orientated
    } else if (blood.co2PartialPressure_mmHg < 55.0 && blood.oxygenSaturation > 90.0) {
        gcsVerbal = 4; // Confused
    } else if (blood.co2PartialPressure_mmHg < 65.0 || blood.oxygenSaturation > 85.0) {
        gcsVerbal = 3; // Inappropriate words
    } else if (blood.co2PartialPressure_mmHg < 75.0 || blood.oxygenSaturation > 75.0) {
        gcsVerbal = 2; // Incomprehensible sounds
    } else {
        gcsVerbal = 1; // None
    }

    // --- Motor Response (1-6) ---
    if (cerebralPerfusionPressure_mmHg > 60 && blood.oxygenSaturation > 92.0) {
        gcsMotor = 6; // Obeys commands
    } else if (cerebralPerfusionPressure_mmHg > 55 && blood.oxygenSaturation > 88.0) {
        gcsMotor = 5; // Localizes to pain
    } else if (cerebralPerfusionPressure_mmHg > 50 || blood.oxygenSaturation > 80.0) {
        gcsMotor = 4; // Withdraws from pain
    } else if (cerebralPerfusionPressure_mmHg > 45 || blood.oxygenSaturation > 70.0) {
        gcsMotor = 3; // Flexion to pain (decorticate)
    } else if (cerebralPerfusionPressure_mmHg > 40 || blood.oxygenSaturation > 60.0) {
        gcsMotor = 2; // Extension to pain (decerebrate)
    } else {
        gcsMotor = 1; // None
    }

    // --- Confounding Factors ---
    // High toxin levels can decrease GCS
    if (blood.toxins_au > 50.0) {
        gcsEye = std::min(gcsEye, 2);
        gcsVerbal = std::min(gcsVerbal, 3);
        gcsMotor = std::min(gcsMotor, 4);
    }
    if (blood.toxins_au > 80.0) {
        gcsEye = 1;
        gcsVerbal = std::min(gcsVerbal, 2);
        gcsMotor = std::min(gcsMotor, 3);
    }

    // Check for spinal cord injury
    if (const SpinalCord* spinalCord = getOrgan<SpinalCord>(patient)) {
        if (spinalCord->getMotorPathwayStatus() != SignalStatus::NORMAL) {
            gcsMotor = 1; // No motor response if spinal cord is damaged
        }
    }

    // Check for intubation (inferred)
    if (const Lungs* lungs = getOrgan<Lungs>(patient)) {
        if (lungs->getPeakInspiratoryPressure() > 5.0) { // Threshold for mechanical ventilation
            gcsVerbal = 1; // Not testable
        }
    }


    // Sum the components for the total score
    gcsScore = gcsEye + gcsVerbal + gcsMotor;
}

void Brain::updatePressures(double meanArterialPressure) {
    // Autoregulation: Brain tries to maintain constant CPP
    // Simplified model: ICP drifts slowly
    intracranialPressure_mmHg += getFluctuation(0.01);
    intracranialPressure_mmHg = std::clamp(intracranialPressure_mmHg, 8.0, 12.0);

    cerebralPerfusionPressure_mmHg = meanArterialPressure - intracranialPressure_mmHg;
    cerebralPerfusionPressure_mmHg = std::max(0.0, cerebralPerfusionPressure_mmHg);
}

void Brain::updateAutonomicControl(Patient& patient, double deltaTime_s) {
    const double& co2 = patient.blood.co2PartialPressure_mmHg;
    const double& o2 = patient.blood.oxygenSaturation;

    // Chemoreceptor reflex: Adjust respiration based on blood gases
    double co2_error = co2 - 40.0; // Normal PaCO2 is 40 mmHg
    double o2_error = 98.0 - o2;   // Normal SpO2 is ~98%

    // Increase rate for high CO2 or low O2
    double co2_drive = std::max(0.0, co2_error) * 0.5; // Strong response to high CO2
    double o2_drive = std::max(0.0, o2_error) * 0.8;   // Stronger response to hypoxia

    double targetRate = 16.0 + co2_drive + o2_drive;

    // Smoothly adjust the current rate towards the target rate
    double adjustmentSpeed = 0.5; // How quickly the rate changes
    targetRespirationRate_bpm += (targetRate - targetRespirationRate_bpm) * adjustmentSpeed * deltaTime_s;

    // Clamp to a physiological range
    targetRespirationRate_bpm = std::clamp(targetRespirationRate_bpm, 8.0, 35.0);

    if (Lungs* lungs = getOrgan<Lungs>(patient)) {
        lungs->setRespirationRate(targetRespirationRate_bpm);
    }

    // Baroreceptor reflex: Adjust heart rate based on blood pressure
    const auto& bp = patient.blood.bloodPressure;
    double meanArterialPressure = bp.diastolic_mmHg + (bp.systolic_mmHg - bp.diastolic_mmHg) / 3.0;

    double bp_error = 90.0 - meanArterialPressure; // Target MAP is 90 mmHg

    // Change HR to correct the error.
    double hr_adjustment = bp_error * 0.4; // Proportional response

    double targetRate_hr = 75.0 + hr_adjustment;

    // Smoothly adjust the current rate towards the target rate
    double hrAdjustmentSpeed = 0.4;
    targetHeartRate_bpm += (targetRate_hr - targetHeartRate_bpm) * hrAdjustmentSpeed * deltaTime_s;

    // Clamp to a physiological range
    targetHeartRate_bpm = std::clamp(targetHeartRate_bpm, 50.0, 160.0);

    if (Heart* heart = getOrgan<Heart>(patient)) {
        heart->setHeartRate(targetHeartRate_bpm);
    }
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
int Brain::getGCSEye() const { return gcsEye; }
int Brain::getGCSVerbal() const { return gcsVerbal; }
int Brain::getGCSMotor() const { return gcsMotor; }
double Brain::getIntracranialPressure() const { return intracranialPressure_mmHg; }
double Brain::getCerebralPerfusionPressure() const { return cerebralPerfusionPressure_mmHg; }
const std::deque<double>& Brain::getEegWaveform() const { return eegData; }
