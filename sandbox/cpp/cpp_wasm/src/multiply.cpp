#include "calculator.hpp"

extern "C" {
    int multiply(int a, int b) {
        Calculator calc;
        return calc.multiply(a, b);
    }
}