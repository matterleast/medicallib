#pragma once

// Define MY_LIB_EXPORT for exporting symbols from the DLL
#if defined(_WIN32)
    #if defined(MY_LIB_EXPORT)
        #define MY_LIB_API __declspec(dllexport)
    #else
        #define MY_LIB_API __declspec(dllimport)
    #endif
#else
    #define MY_LIB_API
#endif

// A simple function to be exported
MY_LIB_API int add(int a, int b);
