#include "calculator.hpp"

extern "C" {
    int subtract(int a, int b) {
        Calculator calc;
        return calc.subtract(a, b);
    }
}