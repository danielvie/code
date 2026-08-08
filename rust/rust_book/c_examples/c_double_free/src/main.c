#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *value = malloc(sizeof *value);
    if (value == NULL) {
        return 1;
    }

    *value = 42;
    printf("value: %d\n", *value);

    free(value);
    printf("freed once\n");

    // Intentional double free: value still points to the released allocation.
    free(value);

    return 0;
}
