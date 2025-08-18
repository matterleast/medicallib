# CMake minimum version
cmake_minimum_required(VERSION 3.10)

# Project name
project(MedicalLib)

# Add the library
add_library(MedicalLib SHARED src/MedicalLib.cpp src/Patient.cpp)

# Define MEDICAL_LIB_EXPORT, so that __declspec(dllexport) is used
target_compile_definitions(MedicalLib PRIVATE MEDICAL_LIB_EXPORT)

# Public headers
target_include_directories(MedicalLib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Set the output directory for the library
set_target_properties(MedicalLib PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Add the examples subdirectory
add_subdirectory(examples)
