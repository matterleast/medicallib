#include "MedicalLib/Patient.h"
#include <random>
#include <algorithm> // For std::clamp

/**
 * @brief Helper function to generate random fluctuations from a normal distribution.
 * @param stddev The standard deviation of the distribution.
 * @return A random value.
 */
double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

/**
 * @brief Initializes a new patient with baseline vital signs.
 * @param patientId The ID for the new patient.
 * @return A Patient struct with default healthy values.
 */
Patient initializePatient(int patientId) {
    Patient patient;
    patient.patientId = patientId;
    patient.bloodPressureSystolic = 120.0;
    patient.bloodPressureDiastolic = 80.0;
    patient.heartRate = 75.0;
    patient.respirationRate = 16.0;
    patient.bodyTemperature = 37.0;
    patient.oxygenSaturation = 98.0;
    return patient;
}

/**
 * @brief Updates the patient's vital signs based on the time elapsed.
 * This function simulates minor, random fluctuations around a healthy baseline.
 * @param patient The patient to update.
 * @param deltaTime_s The time elapsed in seconds.
 */
void updatePatient(Patient& patient, double deltaTime_s) {
    // Baseline healthy values
    const double baseline_hr = 75.0;
    const double baseline_bp_systolic = 120.0;
    const double baseline_bp_diastolic = 80.0;
    const double baseline_rr = 16.0;
    const double baseline_temp = 37.0;
    const double baseline_spo2 = 98.0;

    // Reversion speed (how quickly vitals return to baseline)
    const double theta = 0.1;

    // Standard deviations for random fluctuations per second
    const double hr_stddev = 0.1;
    const double bp_stddev = 0.1;
    const double rr_stddev = 0.05;
    const double temp_stddev = 0.01;
    const double spo2_stddev = 0.02;

    // Update vitals using a mean-reverting model
    patient.heartRate += theta * (baseline_hr - patient.heartRate) * deltaTime_s + getFluctuation(hr_stddev * deltaTime_s);
    patient.bloodPressureSystolic += theta * (baseline_bp_systolic - patient.bloodPressureSystolic) * deltaTime_s + getFluctuation(bp_stddev * deltaTime_s);
    patient.bloodPressureDiastolic += theta * (baseline_bp_diastolic - patient.bloodPressureDiastolic) * deltaTime_s + getFluctuation(bp_stddev * deltaTime_s);
    patient.respirationRate += theta * (baseline_rr - patient.respirationRate) * deltaTime_s + getFluctuation(rr_stddev * deltaTime_s);
    patient.bodyTemperature += theta * (baseline_temp - patient.bodyTemperature) * deltaTime_s + getFluctuation(temp_stddev * deltaTime_s);
    patient.oxygenSaturation += theta * (baseline_spo2 - patient.oxygenSaturation) * deltaTime_s + getFluctuation(spo2_stddev * deltaTime_s);

    // Clamp values to within healthy physiological ranges
    patient.heartRate = std::clamp(patient.heartRate, 60.0, 100.0);
    patient.bloodPressureSystolic = std::clamp(patient.bloodPressureSystolic, 90.0, 120.0);
    patient.bloodPressureDiastolic = std::clamp(patient.bloodPressureDiastolic, 60.0, 80.0);
    patient.respirationRate = std::clamp(patient.respirationRate, 12.0, 20.0);
    patient.bodyTemperature = std::clamp(patient.bodyTemperature, 36.5, 37.3);
    patient.oxygenSaturation = std::clamp(patient.oxygenSaturation, 96.0, 100.0);
}
