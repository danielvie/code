cmake_minimum_required(VERSION 3.10)
project(MathLibsProject C)

# Set C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/include)

# Add subdirectories for libraries
add_subdirectory(lib/mul)
add_subdirectory(lib/sub)
add_subdirectory(lib/sum)

# Add main executable
add_executable(main src/main.c)

# Link libraries to main executable
# target_link_libraries(main PRIVATE mul sub sum)
target_link_libraries(main PRIVATE mul sub sum)