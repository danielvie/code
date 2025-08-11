#include <iostream>
#include <vector>
#include <cstdint> // For std::uintptr_t

// A simple arena allocator
class Arena {
public:
    explicit Arena(size_t size) : m_buffer(size), m_offset(0) {}

    // Allocate memory from the arena
    void* allocate(size_t size) {
        // Check for alignment. Let's assume we want 8-byte alignment for simplicity.
        size_t alignment = 8;
        size_t alignedSize = (size + alignment - 1) & ~(alignment - 1);
        
        if (m_offset + alignedSize > m_buffer.size()) {
            throw std::bad_alloc(); // Not enough memory in the arena
        }
        
        void* ptr = &m_buffer[m_offset];
        m_offset += alignedSize;
        
        return ptr;
    }
    
    // Deallocating is a no-op; the entire arena is freed when it goes out of scope.
    void deallocate(void* ptr, size_t size) {
        // Do nothing
    }

private:
    std::vector<char> m_buffer;
    size_t m_offset;
};

// Example usage
struct MyData {
    int x;
    double y;
};

int main() {
    // Create an arena with a size of 1024 bytes
    Arena arena(1024);

    try {
        // Allocate an integer
        int* p_int = static_cast<int*>(arena.allocate(sizeof(int)));
        *p_int = 42;
        std::cout << "Allocated integer: " << *p_int << std::endl;

        // Allocate a MyData struct
        MyData* p_data = static_cast<MyData*>(arena.allocate(sizeof(MyData)));
        p_data->x = 100;
        p_data->y = 3.14;
        std::cout << "Allocated MyData: x=" << p_data->x << ", y=" << p_data->y << std::endl;
        
        // Allocate another integer
        int* p_int2 = static_cast<int*>(arena.allocate(sizeof(int)));
        *p_int2 = 99;
        std::cout << "Allocated another integer: " << *p_int2 << std::endl;

        // The entire arena is automatically cleaned up when 'arena' goes out of scope.
        // No need to manually 'delete' p_int, p_data, etc.
    } catch (const std::bad_alloc& e) {
        std::cerr << "Allocation failed: " << e.what() << std::endl;
    }

    return 0;
}