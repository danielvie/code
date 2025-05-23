#include "calculator.hpp"

int Calculator::sum(int a, int b) {
    return a + b;
}

int Calculator::subtract(int a, int b) {
    return a - b;
}

int Calculator::multiply(int a, int b) {
    return a * b;
}

std::string Calculator::greet(const std::string& name) {
    return "Hello, " + name + "!";
}