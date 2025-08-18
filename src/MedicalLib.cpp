#include "MedicalLib/MedicalLib.h"
#include <stdexcept>

/**
 * @brief Calculates the Body Mass Index (BMI).
 * @param weight_kg The weight in kilograms.
 * @param height_m The height in meters.
 * @return The calculated BMI.
 * @throws std::invalid_argument if height or weight are non-positive.
 */
double calculateBMI(double weight_kg, double height_m) {
    if (height_m <= 0) {
        throw std::invalid_argument("Height must be positive.");
    }
    if (weight_kg <= 0) {
        throw std::invalid_argument("Weight must be positive.");
    }
    return weight_kg / (height_m * height_m);
}
