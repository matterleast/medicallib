#include <iostream>
#include <memory>
#include "MedicalLib/MedicalLib.h"
#include "MedicalLib/Patient.h"
#include "MedicalLib/Organ.h"
#include "MedicalLib/Heart.h"
#include "MedicalLib/Lungs.h"
#include "MedicalLib/Brain.h"
#include "MedicalLib/Liver.h"
#include "MedicalLib/Kidneys.h"
#include "MedicalLib/Stomach.h"

void printPatientSummary(const Patient& patient) {
    std::cout << "--- Patient Summary (ID: " << patient.patientId << ") ---" << std::endl;
    for (const auto& organ_ptr : patient.organs) {
        // Print the general summary for each organ
        std::cout << organ_ptr->getSummary() << std::endl;

        // Example of accessing specialized properties using dynamic_cast
        if (const Heart* heart = dynamic_cast<const Heart*>(organ_ptr.get())) {
            std::cout << "  -> Specific: Patient Heart Rate is " << heart->getHeartRate() << " bpm." << std::endl;
        }
        if (const Lungs* lungs = dynamic_cast<const Lungs*>(organ_ptr.get())) {
            std::cout << "  -> Specific: Patient SpO2 is " << lungs->getOxygenSaturation() << "%." << std::endl;
        }
        if (const Kidneys* kidneys = dynamic_cast<const Kidneys*>(organ_ptr.get())) {
            std::cout << "  -> Specific: Kidney Filtration Rate is " << kidneys->getFiltrationRate() << " ml/min." << std::endl;
        }
        if (const Stomach* stomach = dynamic_cast<const Stomach*>(organ_ptr.get())) {
            std::cout << "  -> Specific: Stomach pH is " << stomach->getPhLevel() << "." << std::endl;
        }
    }
    std::cout << "------------------------------------" << std::endl;
}

int main() {
    // Initialize a new patient
    Patient patient = initializePatient(1);

    std::cout << "Initial State:" << std::endl;
    printPatientSummary(patient);

    // Simulate a time step
    double deltaTime_s = 1.0;
    updatePatient(patient, deltaTime_s);

    std::cout << "\nState after " << deltaTime_s << " second(s):" << std::endl;
    printPatientSummary(patient);

    return 0;
}
