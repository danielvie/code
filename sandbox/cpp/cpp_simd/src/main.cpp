#include <iostream>
#include <immintrin.h>

int main()
{
    constexpr int N = 16;           // total number of floats (must be multiple of 8 for AVX2)
    alignas(32) float a[N], b[N], c[N], d[N];

    // initialize
    for (int i = 0; i < N; ++i) {
        a[i] = float(i);
        b[i] = float(i) * 2.0f;
    }
    
    // print values of a and b
    for (int i = 0; i < N; ++i) {
        std::cout << a[i] << ", " << b[i] << std::endl;
    }
    
    // 15*8=120
    // 15*2*8=240

    // SIMD loop: process 8 floats at a time
    for (int i = 0; i < N; i += 8) {
        __m256 x = _mm256_load_ps(a + i);
        __m256 y = _mm256_load_ps(b + i);

        __m256 z = _mm256_add_ps(x, y);
        
        _mm256_store_ps(c + i, z);
    }

    // scalar reduction of c[]
    float total = 0.0f;
    for (int i = 0; i < N; ++i) {
        total += c[i];
    }

    std::cout << "Sum of c = " << total << "\n";
    return 0;
}