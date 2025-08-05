
# CMakeLists.txt
# This file uses FetchContent to automatically download and build dependencies.

# Minimum version of CMake required (FetchContent works best with 3.11+)
cmake_minimum_required(VERSION 3.11)

# Project name and language
project(HelloOpenGL CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# --- Find System Libraries ---
# We need to find the OpenGL library provided by the operating system.
# This command creates the 'OpenGL::GL' target.
find_package(OpenGL REQUIRED)

# Include the FetchContent module to manage dependencies
include(FetchContent)

# --- Declare Dependencies ---
# We now use GLAD instead of GLEW as it's modern and easy to build.

# Declare the GLFW dependency
FetchContent_Declare(
    glfw
    GIT_REPOSITORY https://github.com/glfw/glfw.git
    GIT_TAG        3.3.8 # Use a specific, stable version for consistency
)

# Declare the GLAD dependency
# We use the official CMake-compatible repository.
FetchContent_Declare(
    glad
    GIT_REPOSITORY https://github.com/dav1dde/glad.git
    GIT_TAG        v0.1.34 # Use a specific, stable version
)


# --- Make Dependencies Available ---
# This command ensures the dependencies are downloaded and configured,
# making their targets available for linking.
FetchContent_MakeAvailable(glfw glad)


# --- Create Executable ---

# Add the main.cpp source file to create an executable
add_executable(HelloOpenGL src/main.cpp) 

# --- Configure Target ---

# Link the required libraries to our executable.
# By linking to glad and glfw, our HelloOpenGL target automatically
# gets access to their include directories.
target_link_libraries(HelloOpenGL
    PRIVATE
    OpenGL::GL # Standard OpenGL target (found by find_package)
    glad       # The target from GLAD's build
    glfw       # The target from GLFW's build
)
