#include <cstddef>
#include <iostream>
#include <mutex>
#include <stdexcept>


class HeapBuffer {
public:
    explicit HeapBuffer(std::size_t size) : data_(new std::byte[size]) {
        std::cout << "Buffer allocated\n";
    }

    ~HeapBuffer() {
        delete[] data_;
        std::cout << "Buffer released\n";
    }

    HeapBuffer(const HeapBuffer&) = delete;
    HeapBuffer& operator=(const HeapBuffer&) = delete;

private:
    std::byte* data_;
};

void use_lock(std::mutex& mutex) {
    std::lock_guard<std::mutex> lock(mutex);
    std::cout << "Mutex locked\n";

    throw std::runtime_error("simulated lock failure");
}

void use_buffer() {
    HeapBuffer buffer(1024);
    throw std::runtime_error("simulated buffer failure");
}

int main() {
    std::mutex mutex;

    std::cout << "Lock example:\n";
    
    try {
        use_lock(mutex);
    } catch (const std::exception& error) {
        std::cout << "Caught error after mutex cleanup: " << error.what() << "\n\n";
    }

    std::cout << "Buffer example:\n";
    try {
        use_buffer();
    } catch (const std::exception& error) {
        std::cout << "Caught error: " << error.what() << '\n';
    }
}
