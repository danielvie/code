cmake_minimum_required(VERSION 3.14)
project(grpc_cpp_example)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# FetchContent is used to download dependencies at configure time.
include(FetchContent)

# Set the policy to NEW for CMP0079 to allow linking to targets not in the
# current directory. This is necessary for the transitive dependencies
# to work correctly.
cmake_policy(SET CMP0079 NEW)

# By default, gRPC's build system looks for its dependencies on the system.
# We will tell it to fetch all of its dependencies itself.
set(gRPC_FETCH_FROM_GIT ON)
# We also explicitly disable building tests for both gRPC and Protobuf to
# avoid errors from test-specific dependencies that we don't need.
set(gRPC_BUILD_TESTS OFF)
set(protobuf_BUILD_TESTS OFF)

# Use FetchContent to pull in gRPC and let it manage all of its
# own dependencies (including Protobuf, Abseil, etc.). This is a
# much more robust way to handle the dependency tree.
FetchContent_Declare(
  gRPC
  GIT_REPOSITORY https://github.com/grpc/grpc.git
  GIT_TAG v1.51.1
  OVERRIDE_FIND_PACKAGE
)

# Make the gRPC dependency available in the current build.
# This will download gRPC and all its transitive dependencies.
FetchContent_MakeAvailable(gRPC)

# After FetchContent_MakeAvailable, the targets from the fetched projects
# are available to our project.

# Define the source and header variables for the generated files.
set(PROTO_SRC "${CMAKE_CURRENT_BINARY_DIR}/helloworld.pb.cc")
set(PROTO_HDR "${CMAKE_CURRENT_BINARY_DIR}/helloworld.pb.h")
set(GRPC_SRC "${CMAKE_CURRENT_BINARY_DIR}/helloworld.grpc.pb.cc")
set(GRPC_HDR "${CMAKE_CURRENT_BINARY_DIR}/helloworld.grpc.pb.h")

# The `protoc` and `grpc_cpp_plugin` executables are built as part of the
# fetched gRPC and Protobuf projects. We need to find their location within
# the build directory to use them in our custom command.
set(PROTOC_EXECUTABLE "$<TARGET_FILE:protoc>")
set(GRPC_CPP_PLUGIN_EXECUTABLE "$<TARGET_FILE:grpc_cpp_plugin>")

add_custom_command(
    OUTPUT "${PROTO_SRC}" "${PROTO_HDR}" "${GRPC_SRC}" "${GRPC_HDR}"
    COMMAND ${PROTOC_EXECUTABLE}
    ARGS --cpp_out=${CMAKE_CURRENT_BINARY_DIR}
         --grpc_out=${CMAKE_CURRENT_BINARY_DIR}
         --plugin=protoc-gen-grpc=${GRPC_CPP_PLUGIN_EXECUTABLE}
         -I "${CMAKE_CURRENT_SOURCE_DIR}"
         "${CMAKE_CURRENT_SOURCE_DIR}/proto/helloworld.proto"
    DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/proto/helloworld.proto"
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    COMMENT "Generating C++ from helloworld.proto"
)

# Add the server executable
add_executable(greeter_server src/server.cpp "${PROTO_SRC}" "${GRPC_SRC}")
target_link_libraries(greeter_server
  PRIVATE
    grpc++
    grpc++_reflection
)
target_include_directories(greeter_server
  PRIVATE
    "${CMAKE_CURRENT_BINARY_DIR}"
)

# Add the client executable
add_executable(greeter_client src/client.cpp "${PROTO_SRC}" "${GRPC_SRC}")
target_link_libraries(greeter_client
  PRIVATE
    grpc++
)
target_include_directories(greeter_client
  PRIVATE
    "${CMAKE_CURRENT_BINARY_DIR}"
)