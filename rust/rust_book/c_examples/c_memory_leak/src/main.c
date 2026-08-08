#include <stdio.h>
#include <stdlib.h>
#include <time.h>

static void wait_ms(unsigned milliseconds) {
    const clock_t duration =
        (clock_t)milliseconds * CLOCKS_PER_SEC / 1000;
    const clock_t start = clock();

    while (clock() - start < duration) {
    }
}

int main(void) {
    const size_t block_size = 1024 * 1024; // 1Mb
    unsigned long long iteration = 0;

    printf("Leaking 1 MiB every 250 ms. Press Ctrl+C to stop.\n");

    for (;;) {
        // Allocating block
        volatile unsigned char *block = malloc(block_size);
        if (block == NULL) {
            fprintf(stderr, "malloc failed after %llu MiB\n", iteration);
            return 1;
        }
        
        // Touch each page so the allocation is visible in memory monitors.
        for (size_t offset = 0; offset < block_size; offset += 4096) {
            block[offset] = (unsigned char)iteration;
        }

        // Uncomment to release memory.
        // free((void *)block);

        iteration++;
        printf("allocated total: %llu MiB\n", iteration);
        fflush(stdout);

        wait_ms(250);
    }
}
