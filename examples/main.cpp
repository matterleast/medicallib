#include <iostream>
#include "MedicalLib/MedicalLib.h"

void printPatientVitals(const Patient& patient) {
    std::cout << "Patient ID: " << patient.patientId << std::endl;
    std::cout << "  Blood Pressure: " << patient.bloodPressureSystolic << "/" << patient.bloodPressureDiastolic << " mmHg" << std::endl;
    std::cout << "  Heart Rate: " << patient.heartRate << " bpm" << std::endl;
    std::cout << "  Respiration Rate: " << patient.respirationRate << " breaths/min" << std::endl;
    std::cout << "  Body Temperature: " << patient.bodyTemperature << " C" << std::endl;
    std::cout << "  Oxygen Saturation: " << patient.oxygenSaturation << " %" << std::endl;
}

int main() {
    // Initialize a new patient
    Patient patient = initializePatient(1);

    std::cout << "Initial Vitals:" << std::endl;
    printPatientVitals(patient);

    // Simulate a time step
    double deltaTime_s = 1.0;
    updatePatient(patient, deltaTime_s);

    std::cout << "\nVitals after " << deltaTime_s << " second(s):" << std::endl;
    printPatientVitals(patient);

    return 0;
}
