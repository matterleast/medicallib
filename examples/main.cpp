#include <iostream>
#include "MedicalLib/MedicalLib.h"

int main() {
    double weight = 70.0; // kg
    double height = 1.75; // meters
    double bmi = calculateBMI(weight, height);
    std::cout << "Weight: " << weight << " kg, Height: " << height << " m" << std::endl;
    std::cout << "Calculated BMI: " << bmi << std::endl;
    return 0;
}
