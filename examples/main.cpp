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

int main() {
    // Initialize a new patient
    Patient patient = initializePatient(1);
    std::cout << "Patient created with ID: " << patient.patientId << std::endl;

    // Simulate some time passing
    updatePatient(patient, 60.0);
    std::cout << "\nPatient state updated after 60 seconds." << std::endl;

    // Get a summary for a specific organ
    std::cout << "\nHeart Summary:\n" << getOrganSummary(patient, "Heart") << std::endl;

    // Get a summary for all organs
    std::cout << "\nFull Patient Summary:\n" << getPatientSummary(patient) << std::endl;

    // Get a specific organ and call a method on it
    if (const Heart* heart = getOrgan<Heart>(patient)) {
        std::cout << "\nSuccessfully retrieved Heart organ." << std::endl;
        std::cout << "Direct access to heart rate: " << heart->getHeartRate() << " bpm" << std::endl;
    }

    return 0;
}
