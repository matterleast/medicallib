#include "MedicalLib/Patient.h"
#include "MedicalLib/Heart.h"
#include "MedicalLib/Lungs.h"
#include "MedicalLib/Brain.h"
#include "MedicalLib/Liver.h"
#include "MedicalLib/Kidneys.h"
#include "MedicalLib/Bladder.h"
#include "MedicalLib/Stomach.h"
#include "MedicalLib/Intestines.h"
#include "MedicalLib/Gallbladder.h"
#include "MedicalLib/Pancreas.h"
#include "MedicalLib/Esophagus.h"
#include "MedicalLib/Spleen.h"
#include "MedicalLib/SpinalCord.h"
#include <memory>

/**
 * @brief Initializes a new patient with baseline vital signs and a standard set of organs.
 * @param patientId The ID for the new patient.
 * @return A Patient struct with default healthy values.
 */
Patient initializePatient(int patientId) {
    Patient patient;
    patient.patientId = patientId;

    // Initialize default organs
    patient.organs.push_back(std::make_unique<Heart>(1));
    patient.organs.push_back(std::make_unique<Lungs>(2));
    patient.organs.push_back(std::make_unique<Brain>(3));
    patient.organs.push_back(std::make_unique<Liver>(4));
    patient.organs.push_back(std::make_unique<Kidneys>(5));
    patient.organs.push_back(std::make_unique<Bladder>(6));
    patient.organs.push_back(std::make_unique<Stomach>(7));
    patient.organs.push_back(std::make_unique<Intestines>(8));
    patient.organs.push_back(std::make_unique<Gallbladder>(9));
    patient.organs.push_back(std::make_unique<Pancreas>(10));
    patient.organs.push_back(std::make_unique<Esophagus>(11));
    patient.organs.push_back(std::make_unique<Spleen>(12));
    patient.organs.push_back(std::make_unique<SpinalCord>(13));

    return patient;
}

/**
 * @brief Updates the patient's state by updating the state of all their organs.
 * @param patient The patient to update.
 * @param deltaTime_s The time elapsed in seconds.
 */
void updatePatient(Patient& patient, double deltaTime_s) {
    // Update all organs
    for (auto& organ : patient.organs) {
        organ->update(deltaTime_s);
    }
}
