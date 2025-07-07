# wasm.toolchain.cmake

# Target system definition
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR wasm32)

# Find and set the tools from the LLVM toolchain
# Make sure your LLVM 'bin' directory is in the PATH
find_program(CLANG_EXECUTABLE clang REQUIRED)
find_program(WASM_LD_EXECUTABLE wasm-ld REQUIRED)

# Set the C/C++ compiler and the linker
set(CMAKE_C_COMPILER ${CLANG_EXECUTABLE})
set(CMAKE_CXX_COMPILER ${CLANG_EXECUTABLE}++)
set(CMAKE_LINKER ${WASM_LD_EXECUTABLE} CACHE FILEPATH "wasm-ld linker")

# Define the target for the compiler
set(CMAKE_C_FLAGS "--target=wasm32" CACHE STRING "C compiler flags")
set(CMAKE_CXX_FLAGS "--target=wasm32" CACHE STRING "C++ compiler flags")

# Prevent host-side compilation checks
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)