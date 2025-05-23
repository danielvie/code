#include "calculator.hpp"

extern "C" {
    const char* greet(const char* name) {
        Calculator calc;
        std::string greeting = calc.greet(std::string(name));
        char* result = (char*)malloc(greeting.length() + 1);
        strcpy(result, greeting.c_str());
        return result;
    }
}