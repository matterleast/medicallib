# CMake minimum version
cmake_minimum_required(VERSION 3.10)

# Project name
project(MyLibrary)

# Add the library
add_library(MyLibrary SHARED src/Library.cpp)

# Define MY_LIB_EXPORT, so that __declspec(dllexport) is used
target_compile_definitions(MyLibrary PRIVATE MY_LIB_EXPORT)

# Public headers
target_include_directories(MyLibrary PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)

# Set the output directory for the library
set_target_properties(MyLibrary PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Add the examples subdirectory
add_subdirectory(examples)
