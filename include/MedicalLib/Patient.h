#pragma once

#include <vector>
#include <memory>
#include <string>

// Forward-declare the Organ class to avoid circular dependencies
class Organ;

/**
 * @brief Represents the composition of the patient's blood.
 */
struct Blood {
    double oxygenSaturation = 98.0;      // Normal SpO2
    double co2PartialPressure_mmHg = 40.0; // Normal PaCO2
    double glucose_mg_per_dL = 100.0;    // Normal fasting glucose
    double toxins_au = 0.0;              // Arbitrary units, 0 is clean
};

/**
 * @brief Holds all the vital signs and other medical information for a patient.
 */
struct Patient {
    int patientId;
    Blood blood;
    std::vector<std::unique_ptr<Organ>> organs;
};

// Define MEDICAL_LIB_EXPORT for exporting symbols from the DLL
#if defined(_WIN32)
    #if defined(MEDICAL_LIB_EXPORT)
        #define MEDICAL_LIB_API __declspec(dllexport)
    #else
        #define MEDICAL_LIB_API __declspec(dllimport)
    #endif
#else
    #define MEDICAL_LIB_API
#endif

/**
 * @brief Initializes a new patient with baseline vital signs and a 12-lead heart.
 * @param patientId The ID for the new patient.
 * @return A Patient struct with default healthy values.
 */
MEDICAL_LIB_API Patient initializePatient(int patientId);

/**
 * @brief Initializes a new patient with a specific number of heart leads.
 * @param patientId The ID for the new patient.
 * @param numHeartLeads The number of EKG leads for the heart.
 * @return A Patient struct with default healthy values.
 */
MEDICAL_LIB_API Patient initializePatient(int patientId, int numHeartLeads);

/**
 * @brief Updates the patient's vital signs based on the time elapsed.
 * @param patient The patient to update.
 * @param deltaTime_s The time elapsed in seconds.
 */
MEDICAL_LIB_API void updatePatient(Patient& patient, double deltaTime_s);

/**
 * @brief Gets a summary of a specific organ's vitals.
 * @param patient The patient to get the organ summary from.
 * @param organType The type of the organ to get the summary for.
 * @return A string containing the organ's vital signs, or an empty string if not found.
 */
MEDICAL_LIB_API std::string getOrganSummary(const Patient& patient, const std::string& organType);

/**
 * @brief Gets a consolidated summary of all the patient's organ vitals.
 * @param patient The patient to get the summary from.
 * @return A string containing the vital signs of all the patient's organs.
 */
MEDICAL_LIB_API std::string getPatientSummary(const Patient& patient);

/**
 * @brief Gets a pointer to a specific organ by its type.
 * @tparam T The type of the organ to get.
 * @param patient The patient to get the organ from.
 * @return A pointer to the organ if found, otherwise nullptr.
 */
template<typename T>
T* getOrgan(Patient& patient) {
    for (auto& organ : patient.organs) {
        if (T* specificOrgan = dynamic_cast<T*>(organ.get())) {
            return specificOrgan;
        }
    }
    return nullptr;
}

/**
 * @brief Gets a const pointer to a specific organ by its type.
 * @tparam T The type of the organ to get.
 * @param patient The patient to get the organ from.
 * @return A const pointer to the organ if found, otherwise nullptr.
 */
template<typename T>
const T* getOrgan(const Patient& patient) {
    for (const auto& organ : patient.organs) {
        if (const T* specificOrgan = dynamic_cast<const T*>(organ.get())) {
            return specificOrgan;
        }
    }
    return nullptr;
}
