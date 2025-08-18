#pragma once

/**
 * @brief Holds all the vital signs and other medical information for a patient.
 */
struct Patient {
    int patientId;
    double bloodPressureSystolic;
    double bloodPressureDiastolic;
    double heartRate;
    double respirationRate;
    double bodyTemperature;
    double oxygenSaturation;
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
 * @brief Calculates the Body Mass Index (BMI).
 * @param weight_kg The weight in kilograms.
 * @param height_m The height in meters.
 * @return The calculated BMI.
 */
MEDICAL_LIB_API double calculateBMI(double weight_kg, double height_m);

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
