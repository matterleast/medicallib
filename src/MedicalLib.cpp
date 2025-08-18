#include "MedicalLib/MedicalLib.h"
#include <stdexcept>

double calculateBMI(double weight_kg, double height_m) {
    if (height_m <= 0) {
        throw std::invalid_argument("Height must be positive.");
    }
    if (weight_kg <= 0) {
        throw std::invalid_argument("Weight must be positive.");
    }
    return weight_kg / (height_m * height_m);
}
