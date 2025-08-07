cmake_minimum_required(VERSION 3.14)
project(grpc_example CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# --- Fetch Dependencies using FetchContent ---

# First, declare the dependencies and their sources.
include(FetchContent)

FetchContent_Declare(
  protobuf
  GIT_REPOSITORY https://github.com/protocolbuffers/protobuf.git
  GIT_TAG        v27.2
  GIT_SHALLOW    TRUE
)

FetchContent_Declare(
  grpc
  GIT_REPOSITORY https://github.com/grpc/grpc.git
  GIT_TAG        v1.64.1
  GIT_SHALLOW    TRUE
)

# Populate the dependencies. This will download and configure them.
# The `grpc` dependency includes `protobuf`, so we only need to call this on `grpc`.
# The GRPC project will handle the rest.
FetchContent_MakeAvailable(grpc)

# --- gRPC and Protocol Buffer Configuration ---

# This includes the necessary CMake functions to handle .proto files.
# The GRPC build automatically exposes this.
include_directories(${grpc_SOURCE_DIR}/third_party/abseil)

# Define the PROTO source files.
set(PROTO_SOURCES proto/helloworld.proto)

# This command generates C++ source and header files from the .proto files.
# It also defines a target named `grpc_example_proto` that we can link against.
# `GRPC_CPP_PLUGIN_EXECUTABLE` is automatically set by the gRPC build.
grpc_generate_cpp(
  GRPC_PROTO_SOURCES ${PROTO_SOURCES}
  PROTOC_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/proto"
  GRPC_CPP_PLUGIN_EXECUTABLE ${GRPC_CPP_PLUGIN_EXECUTABLE}
  GRPC_CPP_PLUGIN_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
)

# --- Executable Definitions ---

add_executable(grpc_server server.cpp)
target_link_libraries(grpc_server
  PRIVATE
  gpr grpc++ grpc++_reflection
  protobuf::libprotobuf
  ${GRPC_PROTO_SOURCES}
)

add_executable(grpc_client client.cpp)
target_link_libraries(grpc_client
  PRIVATE
  gpr grpc++
  protobuf::libprotobuf
  ${GRPC_PROTO_SOURCES}
)
