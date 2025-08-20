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
#include "MedicalLib/Stomach.h"
#include "MedicalLib/Lungs.h"

int main() {
    // Initialize a new patient with a 12-lead heart
    Patient patient = initializePatient(1, 12);
    std::cout << "Patient created with ID: " << patient.patientId << std::endl;

    // Introduce a toxin load to the blood for the liver to clear
    patient.blood.toxins_au = 100.0;
    std::cout << "Initial toxin load of 100.0 a.u. introduced.\n" << std::endl;

    // Simulate time passing and print a live summary
    const double simulationTime_s = 60.0; // Run for a longer time to see effects
    const double deltaTime_s = 0.1;
    const int numSteps = static_cast<int>(simulationTime_s / deltaTime_s);

    // Get pointers to organs we want to interact with
    Stomach* stomach = getOrgan<Stomach>(patient);
    if(stomach) {
        stomach->addSubstance(300.0); // Simulate eating a meal
        std::cout << "A 300mL meal has been consumed." << std::endl;
    }
    Lungs* lungs = getOrgan<Lungs>(patient);


    std::cout << "\n--- Simulating " << simulationTime_s << " seconds... ---" << std::endl;

    for (int i = 0; i < numSteps; ++i) {
        double currentTime = i * deltaTime_s;
        // Clear console on systems that support ANSI escape codes
        #if defined(__linux__) || defined(__APPLE__)
        std::cout << "\033[2J\033[1;1H";
        #endif

        // --- Event scripting ---
        if (std::abs(currentTime - 20.0) < deltaTime_s/2.0) {
            if (lungs) {
                std::cout << "\n*** LUNG INJURY EVENT ***\n" << std::endl;
                lungs->inflictDamage(0.8); // 80% damage
            }
        }

        updatePatient(patient, deltaTime_s);

        std::cout << "Time: " << currentTime << "s / " << simulationTime_s << "s\n" << std::endl;
        std::cout << "--- Blood Chemistry ---\n"
                  << "SpO2: " << patient.blood.oxygenSaturation << " %\n"
                  << "PaCO2: " << patient.blood.co2PartialPressure_mmHg << " mmHg\n"
                  << "Glucose: " << patient.blood.glucose_mg_per_dL << " mg/dL\n"
                  << "Toxins: " << patient.blood.toxins_au << " a.u.\n\n";

        std::cout << getOrganSummary(patient, "Heart") << std::endl;
        std::cout << getOrganSummary(patient, "Lungs") << std::endl;
        std::cout << getOrganSummary(patient, "Brain") << std::endl;
        std::cout << getOrganSummary(patient, "Stomach") << std::endl;
        std::cout << getOrganSummary(patient, "Intestines") << std::endl;
        std::cout << getOrganSummary(patient, "Pancreas") << std::endl;
        std::cout << getOrganSummary(patient, "Kidneys") << std::endl;
        std::cout << getOrganSummary(patient, "Bladder") << std::endl;


        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }

    std::cout << "\n--- Simulation Complete. Final State: ---\n" << std::endl;
    std::cout << getPatientSummary(patient) << std::endl;

    return 0;
}
