# Agent Instructions for MedicalLib

This document provides instructions for developers and agents working on the MedicalLib Rust project.

## Project Purpose

MedicalLib is a Rust library for detailed mathematical simulations of various medical information. Its goal is to provide a robust set of tools for applications that require calculations related to:

-   Heart rate and EKG readings
-   Body metrics (e.g., BMI, body fat percentage)
-   Physiological responses to injuries
-   Other life-requirement math simulations

## Project Structure

This project is structured as a software-agnostic Rust library, designed for easy integration into various larger projects.

-   `/src`: Contains the Rust source code for the library
    -   `/src/organs`: Individual organ implementations
    -   `/src/lib.rs`: Main library interface
    -   `/src/patient.rs`: Patient management system
    -   `/src/organ.rs`: Base organ trait
-   `/examples`: Contains example code showing how to use the library
-   `/target`: This directory is created by Cargo and contains the compiled library and example executables. It is not tracked by git.
-   `Cargo.toml`: The Cargo manifest file for building the project

## Building the Project

To build the project, use Cargo:

```bash
# Build the library
cargo build

# Build in release mode
cargo build --release

# Run the simulation example
cargo run --example simulation

# Build documentation
cargo doc --open
```

The compiled library will be in `target/debug` (or `target/release`), and examples will be in `target/debug/examples`.

## Coding Standards

### Documentation

All public functions, structs, enums, and traits must be documented using Rust documentation comments. This ensures that the code is easy to understand and that high-quality documentation can be automatically generated.

- Use `///` for documenting items (functions, structs, etc.)
- Use `//!` for module-level documentation
- Use markdown formatting in documentation

Example:
```rust
/// Calculate Body Mass Index (BMI)
///
/// # Arguments
/// * `weight_kg` - Weight in kilograms
/// * `height_m` - Height in meters
///
/// # Returns
/// The calculated BMI value
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    if height_m <= 0.0 {
        return 0.0;
    }
    weight_kg / (height_m * height_m)
}
```

## Using the Library

Add MedicalLib to your `Cargo.toml`:

```toml
[dependencies]
medicallib = { path = "../medicallib" }
```

Then use it in your code:

```rust
use medicallib::*;

fn main() {
    let mut patient = initialize_patient(1, 12);
    update_patient(&mut patient, 0.1);
    println!("{}", get_patient_summary(&patient));
}
```
