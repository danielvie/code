cmake_minimum_required(VERSION 3.20.0)
project(main C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_C_STANDARD 23)

file(GLOB sources "src/*.c")
add_executable(main ${sources})
