#include <iostream>
#include <memory>
#include <thread>
#include <chrono>
#include <vector>
#include <string>
#include "MedicalLib/MedicalLib.h"
#include "MedicalLib/Patient.h"
#include "MedicalLib/Organ.h"
#include "MedicalLib/Heart.h"

int main() {
    // Initialize a new patient with a 12-lead heart
    Patient patient = initializePatient(1, 12);
    std::cout << "Patient created with ID: " << patient.patientId << std::endl;

    // Simulate time passing and print a live summary
    const double simulationTime_s = 10.0;
    const double deltaTime_s = 0.05; // 20 Hz update rate for display
    const int numSteps = static_cast<int>(simulationTime_s / deltaTime_s);

    std::cout << "\n--- Simulating " << simulationTime_s << " seconds of heart activity... ---" << std::endl;

    for (int i = 0; i < numSteps; ++i) {
        // Clear console on systems that support ANSI escape codes
        #if defined(__linux__) || defined(__APPLE__)
        std::cout << "\033[2J\033[1;1H";
        #endif

        updatePatient(patient, deltaTime_s);

        std::cout << "Time: " << i * deltaTime_s << "s / " << simulationTime_s << "s\n" << std::endl;
        std::cout << getOrganSummary(patient, "Heart") << std::endl;

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }

    std::cout << "\n--- Simulation Complete. Final State: ---\n" << std::endl;
    std::cout << getPatientSummary(patient) << std::endl;

    return 0;
}
