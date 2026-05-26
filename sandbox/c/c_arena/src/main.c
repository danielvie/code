#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    uint8_t *buffer;
    size_t capacity;
    size_t offset;
} Arena;

// Initialize the arena with a large block of memory
Arena arena_init(size_t capacity) {
    Arena arena;
    arena.buffer = (uint8_t *)malloc(capacity);
    arena.capacity = capacity;
    arena.offset = 0;
    return arena;
}

// Allocate a chunk of memory from the arena
void *arena_alloc(Arena *arena, size_t size) {
    // Basic alignment (e.g., aligning to 8 bytes for 64-bit systems)
    size_t aligned_size = (size + 7) & ~7;

    if (arena->offset + aligned_size > arena->capacity) {
        printf("Arena out of memory!\n");
        return NULL; // Out of memory
    }

    // Get the pointer to the free space
    void *ptr = &arena->buffer[arena->offset];
    arena->offset += aligned_size;
    return ptr;
}

// Clear the entire arena instantly
void arena_reset(Arena *arena) {
    arena->offset = 0;
}

// Free the backing memory completely
void arena_free(Arena *arena) {
    free(arena->buffer);
    arena->buffer = NULL;
    arena->capacity = 0;
    arena->offset = 0;
}

int main_heap() {
    // Create a 1KB arena
    Arena arena = arena_init(1024);
    
    // Allocate on arena memory
    int *point_x = (int *)arena_alloc(&arena, sizeof(int));
    int *point_y = (int *)arena_alloc(&arena, sizeof(int));
    
    *point_x = 42;
    *point_y = 84;
    
    printf(" Heap Arena Point: (%d, %d)\n", *point_x, *point_y);
    
    // Free everything in a single CPU instruction
    arena_reset(&arena);
    arena_free(&arena);
    return 0;
}

Arena arena_init_from_buffer(uint8_t *stack_buffer, size_t capacity) {
    Arena arena;
    arena.buffer = stack_buffer;
    arena.capacity = capacity;
    arena.offset = 0;
    return arena;
}

int main_stack() {
    // 1. Allocate a 1KB block of bytes directly on the CPU stack
    uint8_t stack_mem[1024];

    // 2. Point our arena structure to that stack memory
    // No malloc, no system calls, instant initialization.
    Arena arena = arena_init_from_buffer(stack_mem, sizeof(stack_mem));

    // 3. Fast O(1) allocations with incredible cache locality
    int *point_x = (int *)arena_alloc(&arena, sizeof(int));
    int *point_y = (int *)arena_alloc(&arena, sizeof(int));

    if (point_x && point_y) {
        *point_x = 42;
        *point_y = 84;
        printf("Stack Arena Point: (%d, %d)\n", *point_x, *point_y);
    }

    // 4. Resetting is still just moving an offset
    arena_reset(&arena);

    // Note: We DO NOT call free() here. 
    // When the main() function ends, `stack_mem` is automatically 
    // cleaned up when the stack frame pops.
    return 0;
}

int main() {
    main_heap();
    main_stack();
}