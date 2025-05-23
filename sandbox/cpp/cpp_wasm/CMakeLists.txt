cmake_minimum_required(VERSION 3.10)
project(WasmCalculator CXX)

set(CMAKE_CXX_STANDARD 11)

# Emscripten-specific settings
if (EMSCRIPTEN)
    set(CMAKE_EXECUTABLE_SUFFIX ".js")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -s WASM=1 -s EXPORTED_FUNCTIONS=_sum,_subtract,_multiply,_greet -s EXPORTED_RUNTIME_METHODS=ccall,cwrap -s MODULARIZE=1 -s EXPORT_NAME=WasmCalculator")
endif()

set(SOURCES
    src/calculator.cpp
    src/greet.cpp
    src/multiply.cpp 
    src/subtract.cpp
    src/sum.cpp 
)

add_executable(wasm_calculator ${SOURCES})