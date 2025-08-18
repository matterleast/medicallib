#pragma once

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
