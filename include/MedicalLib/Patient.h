#pragma once

#include <vector>
#include <memory>

// Forward-declare the Organ class to avoid circular dependencies
class Organ;

/**
 * @brief Holds all the vital signs and other medical information for a patient.
 */
struct Patient {
    int patientId;
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
 * @brief Initializes a new patient with baseline vital signs.
 * @param patientId The ID for the new patient.
 * @return A Patient struct with default healthy values.
 */
MEDICAL_LIB_API Patient initializePatient(int patientId);

/**
 * @brief Updates the patient's vital signs based on the time elapsed.
 * @param patient The patient to update.
 * @param deltaTime_s The time elapsed in seconds.
 */
MEDICAL_LIB_API void updatePatient(Patient& patient, double deltaTime_s);
