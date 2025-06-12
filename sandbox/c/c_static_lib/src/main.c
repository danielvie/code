#include <stdio.h>
#include "math_ops.h"

int main()
{
    double a = 10.0, b = 5.0;
    printf("Numbers: %.2f, %.2f\n", a, b);
    printf("Multiplication: %.2f\n", multiply(a, b));
    printf("Subtraction: %.2f\n", subtract(a, b));
    printf("Sum: %.2f\n", sum(a, b));

    return 0;
}