#include <stdint.h>
#include <stdio.h>

int main() {
    
    int16_t e = 10;
    int16_t s = 857;
    
    int32_t b = 15;
    
    int32_t *v = &b;
    
    
    uint8_t num = -1;
    
    v = v + 1;
    
    printf("value: %d\n", (int16_t)*v);
    printf("uint: %d\n", num);
    return 0;
}
