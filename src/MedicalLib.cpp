#include "MedicalLib/MedicalLib.h"
#include <stdexcept>

double calculateBMI(double weight_kg, double height_m) {
    if (height_m <= 0) {
        throw std::invalid_argument("Height must be positive.");
    }
    if (weight_kg <= 0) {
        throw std::invalid_argument("Weight must be positive.");
    }
    return weight_kg / (height_m * height_m);
}

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

void updatePatient(Patient& patient, double deltaTime_s) {
    // For this initial implementation, we will not change the patient's vitals.
    // This function serves as a placeholder for future, more complex simulation logic.
    (void)patient;
    (void)deltaTime_s;
}
