#pragma once

#include "MedicalLib/Patient.h"

/**
 * @brief Calculates the Body Mass Index (BMI).
 * @param weight_kg The weight in kilograms.
 * @param height_m The height in meters.
 * @return The calculated BMI.
 */
MEDICAL_LIB_API double calculateBMI(double weight_kg, double height_m);
