#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    
    char var[] = "ik ben hier";
    int32_t a = 15;
    
    int32_t *v = &a;
    
    printf("value b: %d\n", *v);
    
    v += 2;
    
    printf("value b: %s\n", (char*)v);
    
    return 0;
}
