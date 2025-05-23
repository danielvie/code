#include "calculator.hpp"

extern "C" {
    int sum(int a, int b) {
        Calculator calc;
        return calc.sum(a, b);
    }
}