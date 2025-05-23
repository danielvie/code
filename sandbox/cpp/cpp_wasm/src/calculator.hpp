#ifndef CALCULATOR_HPP
#define CALCULATOR_HPP

#include <string>

class Calculator {
public:
    int sum(int a, int b);
    int subtract(int a, int b);
    int multiply(int a, int b);
    std::string greet(const std::string& name);
};

#endif