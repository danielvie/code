cmake_minimum_required(VERSION 3.20)
project(WasmExample C)

# Place the output .wasm file in the build directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

file(GLOB sources src/*.c)
add_executable(add ${sources})

# Set the output file to have a .wasm extension
set_target_properties(add PROPERTIES SUFFIX ".wasm")

# Add all flags needed for linking to target_link_options
# set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} --no-standard-libraries -Wl,--export-all -Wl,--no-entry")

target_link_options(add PRIVATE
    # This is a driver flag that controls linking
    --no-standard-libraries
    # These flags are passed directly to the linker
    -Wl,--export-all
    -Wl,--no-entry
)