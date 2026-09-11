#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main() {

    char *var_ = "ik ben hier";
    int32_t a_ = 15;
    char var[] = "ik ben hier";
    int32_t a = 15;

    int32_t *v = &a;
    int32_t *v_ = &a_;

    printf("value b: %d\n", *v);

    v += 2;
    v_ += 2;

    printf("value b: %s\n", (char*)v);
    printf("value b: %s\n", (char*)*v_);

    return 0;
}
