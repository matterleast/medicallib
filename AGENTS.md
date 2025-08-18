# Agent Instructions for MedicalLib

This document provides instructions for developers and agents working on the MedicalLib C++ project.

## Project Purpose

MedicalLib is a C++ library for detailed mathematical simulations of various medical information. Its goal is to provide a robust set of tools for applications that require calculations related to:

-   Heart rate and EKG readings
-   Body metrics (e.g., BMI, body fat percentage)
-   Physiological responses to injuries
-   Other life-requirement math simulations

## Project Structure

This project is structured as a software-agnostic C++ library, designed for easy integration into various larger projects, including game engines like Unreal Engine.

-   `/include`: Contains all public header files.
    -   `/include/MedicalLib`: Headers for the MedicalLib library are placed here to prevent naming conflicts.
-   `/src`: Contains the implementation (.cpp) files for the library.
-   `/examples`: Contains example code showing how to use the library.
-   `/build`: This directory is created by the build scripts and contains the compiled library and example executables. It is not tracked by git.
-   `CMakeLists.txt`: The root CMake file for building the project.

## Building the Project

To build the project, use the provided build scripts:

-   On Linux or macOS: `./build.sh`
-   On Windows: `build.bat`

The compiled library will be placed in `build/lib`, and the example executable will be in `build/examples/`.

## Integration with Unreal Engine

To use this library in an Unreal Engine plugin:

1.  Copy the contents of the `include` directory into your plugin's `ThirdParty/MedicalLib/include` directory.
2.  Compile the library for the desired platforms (e.g., Win64, Linux) and copy the compiled library files (e.g., `.lib`, `.so`, `.a`) into your plugin's `ThirdParty/MedicalLib/lib/<Platform>` directory.
3.  In your plugin's `.Build.cs` file, add the necessary paths for the include files and link against the compiled library.
