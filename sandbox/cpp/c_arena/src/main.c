#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

// arena buffer size, in multiples of pages: 4096*4=16384
#define ARENA_SIZE 16384 // 16 KB is a safe size for most stacks

// arena structure
typedef struct Arena {
    void* buffer;      // Pointer to the start of the memory buffer (the stack array)
    size_t size;       // Total size of the buffer
    void* current_pos; // A pointer to the next available byte in the buffer
    size_t remaining;  // How many bytes are left
} Arena;

// Initializes a stack-based Arena allocator.
void arena_init(Arena* arena, void* buffer, size_t size) {
    arena->buffer = buffer;
    arena->size = size;
    arena->current_pos = buffer;
    arena->remaining = size;
    printf("Stack-based arena initialized with a size of %zu bytes.\n", size);
}

// Allocates a block of memory from the stack-based arena.
void* arena_alloc(Arena* arena, size_t size) {
    // Add a small alignment padding to prevent misaligned memory access issues.
    size_t padded_size = (size + 7) & ~7;

    // Check if the padded size fits in the remaining space.
    if (padded_size > arena->remaining) {
        printf("Error: Not enough space (%zu bytes needed) in arena (remaining: %zu).\n",
               padded_size, arena->remaining);
        return NULL;
    }

    // Allocate memory from the buffer.
    void* allocated_memory = arena->current_pos;
    arena->current_pos = (char*)arena->current_pos + padded_size;
    arena->remaining -= padded_size;

    printf("Allocated %zu bytes (padded to %zu). Remaining: %zu\n",
           size, padded_size, arena->remaining);
    return allocated_memory;
}

int main() {

    // check page size
    long page_size;
    SYSTEM_INFO sys_info;
    GetSystemInfo(&sys_info);
    
    printf("System page size is: %lu bytes\n", sys_info.dwPageSize);

    // -----------------------------------------------------------------
    // -----------------------------------------------------------------

    // declare the arena's memory buffer on the stack.
    char my_stack_arena_memory[ARENA_SIZE];

    // initialize the arena with this buffer.
    Arena my_arena;
    arena_init(&my_arena, my_stack_arena_memory, ARENA_SIZE);

    printf("\n--- Test 1: Allocations within the stack-based arena ---\n");
    void* ptr1 = arena_alloc(&my_arena, 1000);
    if (ptr1) {
        memset(ptr1, 'A', 1000);
    }

    void* ptr2 = arena_alloc(&my_arena, 2048);
    if (ptr2) {
        memset(ptr2, 'B', 2048);
    }

    // print values for the pointers
    if (ptr1) { printf("Content at ptr1: '%.10s...'\n", (char*)ptr1); }

    printf("\n--- Test 2: Allocation that exceeds the stack-based arena ---\n");
    // Request a large block of memory (20 KB). This will fail because
    void* too_large_ptr = arena_alloc(&my_arena, 20000);
    if (!too_large_ptr) {
        printf("Successfully failed to allocate a block larger than the arena size.\n");
    }

    // Cleanup: Nothing to do! The entire buffer is automatically
    return 0;
}
