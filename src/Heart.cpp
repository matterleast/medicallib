#include "MedicalLib/Heart.h"
#include "MedicalLib/Patient.h"
#include <random>
#include <algorithm>
#include <sstream>
#include <vector>
#include <string>
#include <cmath>
#include <numeric>
#include <deque>

// Helper for Gaussian function to model EKG waves
static double gaussian(double x, double mu, double sigma) {
    return exp(-0.5 * pow((x - mu) / sigma, 2));
}

// Helper function for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Heart::Heart(int id, int numLeads)
    : Organ(id, "Heart"),
      heartRate(75.0),
      measuredHeartRate(75.0),
      numLeads(numLeads),
      ekgHistorySize(200),
      totalTime_s(0.0),
      cardiacCyclePosition_s(0.0),
      lastRPeakTime_s(-1.0),
      rPeakDetectedInCycle(false),
      ejectionFraction(0.55) {

    // Initialize Chambers
    leftAtrium.name = "Left Atrium";
    rightAtrium.name = "Right Atrium";
    leftVentricle.name = "Left Ventricle";
    rightVentricle.name = "Right Ventricle";

    // Initialize Valves
    mitralValve.name = "Mitral Valve";
    tricuspidValve.name = "Tricuspid Valve";
    aorticValve.name = "Aortic Valve";
    pulmonaryValve.name = "Pulmonary Valve";

    const std::vector<std::string> allLeadNames = {
        "I", "II", "III", "aVR", "aVL", "aVF",
        "V1", "V2", "V3", "V4", "V5", "V6"
    };

    int leadsToCreate = std::min((int)allLeadNames.size(), numLeads);
    for (int i = 0; i < leadsToCreate; ++i) {
        leadNames.push_back(allLeadNames[i]);
        ekgData[allLeadNames[i]] = std::deque<double>();
    }
}

void Heart::update(Patient& patient, double deltaTime_s) {
    // --- Electrical Simulation Update ---
    totalTime_s += deltaTime_s;

    // The underlying heartRate is now set externally by the Brain.
    // We just add a little natural variation.
    heartRate += getFluctuation(0.01);
    double cycleDuration_s = 60.0 / heartRate;

    double oldCyclePosition = cardiacCyclePosition_s;
    cardiacCyclePosition_s += deltaTime_s;

    // R-peak detection for measured heart rate
    double rPeakCycleTime = 0.22 * cycleDuration_s;
    if (oldCyclePosition < rPeakCycleTime && cardiacCyclePosition_s >= rPeakCycleTime && !rPeakDetectedInCycle) {
        if (lastRPeakTime_s > 0) {
            measuredHeartRate = 60.0 / (totalTime_s - lastRPeakTime_s);
        }
        lastRPeakTime_s = totalTime_s;
        rPeakDetectedInCycle = true;
    }

    if (cardiacCyclePosition_s > cycleDuration_s) {
        cardiacCyclePosition_s -= cycleDuration_s;
        rPeakDetectedInCycle = false;
    }

    double timeInCycle = cardiacCyclePosition_s / cycleDuration_s;
    double baseVoltage = simulateEkgWaveform(timeInCycle);
    for(const auto& leadName : leadNames) {
        double leadModifier = 1.0 - (std::distance(leadNames.begin(), std::find(leadNames.begin(), leadNames.end(), leadName)) * 0.1);
        ekgData[leadName].push_front(baseVoltage * leadModifier);
        if (ekgData[leadName].size() > ekgHistorySize) ekgData[leadName].pop_back();
    }

    // --- Mechanical Simulation Update ---
    // Define pressures outside the heart
    double venousPressure = 5.0; // CVP
    double pulmonaryArteryPressure = 20.0;
    double aorticPressure = getAorticPressure();

    // Determine chamber states based on EKG-timed cycle
    if (timeInCycle >= 0.0 && timeInCycle < 0.15) { // Atrial Systole
        leftAtrium.state = ChamberState::SYSTOLE;
        rightAtrium.state = ChamberState::SYSTOLE;
        // End of ventricular filling, capture EDV
        if(oldCyclePosition < 0.15 && timeInCycle >= 0.15) {
            leftVentricle.endDiastolicVolume_mL = leftVentricle.volume_mL;
        }
    } else {
        leftAtrium.state = ChamberState::DIASTOLE;
        rightAtrium.state = ChamberState::DIASTOLE;
    }

    if (timeInCycle >= 0.20 && timeInCycle < 0.5) { // Ventricular Systole
        leftVentricle.state = ChamberState::SYSTOLE;
        rightVentricle.state = ChamberState::SYSTOLE;
        // End of ventricular ejection, capture ESV and calculate EF
        if(oldCyclePosition < 0.5 && timeInCycle >= 0.5) {
            leftVentricle.endSystolicVolume_mL = leftVentricle.volume_mL;
            if (leftVentricle.endDiastolicVolume_mL > 0) {
                ejectionFraction = (leftVentricle.endDiastolicVolume_mL - leftVentricle.endSystolicVolume_mL) / leftVentricle.endDiastolicVolume_mL;
            }
        }
    } else {
        leftVentricle.state = ChamberState::DIASTOLE;
        rightVentricle.state = ChamberState::DIASTOLE;
    }

    // Update chamber pressures based on state (very simplified model)
    leftAtrium.pressure_mmHg = (leftAtrium.state == ChamberState::SYSTOLE) ? 10.0 : 5.0;
    rightAtrium.pressure_mmHg = (rightAtrium.state == ChamberState::SYSTOLE) ? 7.0 : 2.0;
    leftVentricle.pressure_mmHg = (leftVentricle.state == ChamberState::SYSTOLE) ? 125.0 * sin((timeInCycle - 0.2) / 0.3 * M_PI) : 5.0;
    rightVentricle.pressure_mmHg = (rightVentricle.state == ChamberState::SYSTOLE) ? 25.0 * sin((timeInCycle - 0.2) / 0.3 * M_PI) : 2.0;

    // Update Valve Status
    tricuspidValve.status = (rightAtrium.pressure_mmHg > rightVentricle.pressure_mmHg) ? ValveStatus::OPEN : ValveStatus::CLOSED;
    mitralValve.status = (leftAtrium.pressure_mmHg > leftVentricle.pressure_mmHg) ? ValveStatus::OPEN : ValveStatus::CLOSED;
    pulmonaryValve.status = (rightVentricle.pressure_mmHg > pulmonaryArteryPressure) ? ValveStatus::OPEN : ValveStatus::CLOSED;
    aorticValve.status = (leftVentricle.pressure_mmHg > aorticPressure) ? ValveStatus::OPEN : ValveStatus::CLOSED;

    // Update Chamber Volumes (simplified blood flow)
    double flowRate = 500.0 * deltaTime_s; // mL/s
    if (mitralValve.status == ValveStatus::OPEN) leftVentricle.volume_mL += flowRate;
    if (tricuspidValve.status == ValveStatus::OPEN) rightVentricle.volume_mL += flowRate;
    if (aorticValve.status == ValveStatus::OPEN) leftVentricle.volume_mL -= flowRate * 1.5;
    if (pulmonaryValve.status == ValveStatus::OPEN) rightVentricle.volume_mL -= flowRate * 1.5;

    // Clamp volumes to realistic values
    leftVentricle.volume_mL = std::max(40.0, std::min(leftVentricle.volume_mL, 130.0));
    rightVentricle.volume_mL = std::max(40.0, std::min(rightVentricle.volume_mL, 130.0));

    // --- Update Blood Pressure ---
    // Simplified model: BP is influenced by heart rate and RAAS.
    double angiotensinEffect = patient.blood.angiotensin_au * 2.0; // Angiotensin is a potent vasoconstrictor
    double systolic = 110.0 + (heartRate - 75.0) * 0.5 + angiotensinEffect;
    double diastolic = 75.0 + (heartRate - 75.0) * 0.25 + angiotensinEffect;
    patient.blood.bloodPressure.systolic_mmHg = std::clamp(systolic, 80.0, 180.0);
    patient.blood.bloodPressure.diastolic_mmHg = std::clamp(diastolic, 50.0, 110.0);
}

double Heart::simulateEkgWaveform(double timeInCycle) {
    double p_time = 0.1, q_time = 0.2, r_time = 0.22, s_time = 0.24, t_time = 0.4;
    double p_amp = 0.15, q_amp = -0.1, r_amp = 1.0, s_amp = -0.25, t_amp = 0.3;
    double p_sigma = 0.04, qrs_sigma = 0.02, t_sigma = 0.06;
    double voltage = 0.0;
    voltage += p_amp * gaussian(timeInCycle, p_time, p_sigma);
    voltage += q_amp * gaussian(timeInCycle, q_time, qrs_sigma);
    voltage += r_amp * gaussian(timeInCycle, r_time, qrs_sigma);
    voltage += s_amp * gaussian(timeInCycle, s_time, qrs_sigma);
    voltage += t_amp * gaussian(timeInCycle, t_time, t_sigma);
    return voltage;
}

std::string Heart::getSummary() const {
    std::stringstream ss;
    ss.precision(2);
    ss << std::fixed;
    ss << "--- Heart Summary ---\n"
       << "Heart Rate (Measured): " << getHeartRate() << " bpm\n"
       << "Ejection Fraction: " << getEjectionFraction() * 100.0 << "%\n"
       << "Aortic Pressure: " << getAorticPressure() << " mmHg\n\n"
       << "--- Chambers ---\n"
       << " LV Volume: " << leftVentricle.volume_mL << " mL\n"
       << " LV Pressure: " << leftVentricle.pressure_mmHg << " mmHg\n"
       << " RV Volume: " << rightVentricle.volume_mL << " mL\n"
       << " RV Pressure: " << rightVentricle.pressure_mmHg << " mmHg\n\n"
       << "--- Valves ---\n"
       << " Aortic Valve: " << (aorticValve.status == ValveStatus::OPEN ? "OPEN" : "CLOSED") << "\n"
       << " Mitral Valve: " << (mitralValve.status == ValveStatus::OPEN ? "OPEN" : "CLOSED") << "\n";
    return ss.str();
}

double Heart::getHeartRate() const { return measuredHeartRate; }
const std::map<std::string, std::deque<double>>& Heart::getEkgData() const { return ekgData; }
double Heart::getEjectionFraction() const { return ejectionFraction; }
double Heart::getAorticPressure() const {
    // Aortic pressure decays over time, but gets "pumped up" by ventricular ejection
    // This is a placeholder for a more complex arterial model.
    if(aorticValve.status == ValveStatus::OPEN) {
        return leftVentricle.pressure_mmHg;
    }
    // Simplified diastolic pressure decay
    return 80.0 + 40.0 * exp(-cardiacCyclePosition_s);
}

void Heart::setHeartRate(double newRate_bpm) {
    // Set the underlying target heart rate. The simulation will then use this
    // as the basis for its cycle timing.
    heartRate = newRate_bpm;
}
