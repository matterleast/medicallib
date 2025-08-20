#pragma once

#include <string>
#include <vector>

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
 * @brief Abstract base class for all organ types.
 */
class MEDICAL_LIB_API Organ {
public:
    /**
     * @brief Constructor for the Organ class.
     * @param id The ID of the organ.
     * @param type The type of the organ as a string.
     */
    Organ(int id, const std::string& type);

    /**
     * @brief Virtual destructor.
     */
    virtual ~Organ() = default;

    /**
     * @brief Pure virtual function to update the organ's state over time.
     * @param deltaTime_s The time elapsed in seconds.
     */
    virtual void update(double deltaTime_s) = 0;

    /**
     * @brief Pure virtual function to get a string summary of the organ's vitals.
     * @return A string containing the organ's vital signs.
     */
    virtual std::string getSummary() const = 0;

    /**
     * @brief Gets the organ ID.
     * @return The organ ID.
     */
    int getId() const;

    /**
     * @brief Gets the organ type.
     * @return The organ type as a string.
     */
    const std::string& getType() const;

protected:
    int organId;
    std::string organType;
};
