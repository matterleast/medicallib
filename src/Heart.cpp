#include "MedicalLib/Heart.h"
#include <random>
#include <algorithm>
#include <sstream>

// Helper function for random fluctuations
static double getFluctuation(double stddev) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::normal_distribution<> d(0, stddev);
    return d(gen);
}

Heart::Heart(int id) : Organ(id, "Heart"), heartRate(75.0), bloodPressureSystolic(120.0), bloodPressureDiastolic(80.0) {}

void Heart::update(double deltaTime_s) {
    const double baseline_hr = 75.0;
    const double baseline_bp_systolic = 120.0;
    const double baseline_bp_diastolic = 80.0;
    const double theta = 0.1; // Mean reversion speed
    const double hr_stddev = 0.1;
    const double bp_stddev = 0.1;

    heartRate += theta * (baseline_hr - heartRate) * deltaTime_s + getFluctuation(hr_stddev * deltaTime_s);
    bloodPressureSystolic += theta * (baseline_bp_systolic - bloodPressureSystolic) * deltaTime_s + getFluctuation(bp_stddev * deltaTime_s);
    bloodPressureDiastolic += theta * (baseline_bp_diastolic - bloodPressureDiastolic) * deltaTime_s + getFluctuation(bp_stddev * deltaTime_s);

    heartRate = std::clamp(heartRate, 60.0, 100.0);
    bloodPressureSystolic = std::clamp(bloodPressureSystolic, 90.0, 120.0);
    bloodPressureDiastolic = std::clamp(bloodPressureDiastolic, 60.0, 80.0);
}

std::string Heart::getSummary() const {
    std::stringstream ss;
    ss << "Type: " << organType << " (ID: " << organId << ")\n"
       << "  Heart Rate: " << heartRate << " bpm\n"
       << "  Blood Pressure: " << bloodPressureSystolic << "/" << bloodPressureDiastolic << " mmHg";
    return ss.str();
}

double Heart::getHeartRate() const { return heartRate; }
double Heart::getBloodPressureSystolic() const { return bloodPressureSystolic; }
double Heart::getBloodPressureDiastolic() const { return bloodPressureDiastolic; }
