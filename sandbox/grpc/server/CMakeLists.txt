cmake_minimum_required(VERSION 3.15)
project(GrpcDockerServer)

# Find Protobuf
find_package(Protobuf REQUIRED)

# --- FIX: Use PkgConfig instead of find_package(gRPC) ---
find_package(PkgConfig REQUIRED)
pkg_check_modules(GRPC REQUIRED grpc++ grpc)

# Helper for finding the grpc_cpp_plugin
find_program(GRPC_CPP_PLUGIN NAMES grpc_cpp_plugin)

# Define the proto file
set(PROTO_FILE "helloworld.proto")

# Generate the C++ sources from the .proto file
set(PROTO_SRC_DIR ${CMAKE_CURRENT_BINARY_DIR})
file(MAKE_DIRECTORY ${PROTO_SRC_DIR})

set(PROTO_SRCS "${PROTO_SRC_DIR}/helloworld.pb.cc")
set(PROTO_HDRS "${PROTO_SRC_DIR}/helloworld.pb.h")
set(GRPC_SRCS "${PROTO_SRC_DIR}/helloworld.grpc.pb.cc")
set(GRPC_HDRS "${PROTO_SRC_DIR}/helloworld.grpc.pb.h")

add_custom_command(
  OUTPUT ${PROTO_SRCS} ${PROTO_HDRS} ${GRPC_SRCS} ${GRPC_HDRS}
  COMMAND ${Protobuf_PROTOC_EXECUTABLE}
  ARGS --grpc_out=${PROTO_SRC_DIR}
       --cpp_out=${PROTO_SRC_DIR}
       --plugin=protoc-gen-grpc=${GRPC_CPP_PLUGIN}
       -I ${CMAKE_CURRENT_SOURCE_DIR}
       ${PROTO_FILE}
  DEPENDS ${PROTO_FILE}
)

# Define the server executable
add_executable(server server.cpp ${PROTO_SRCS} ${GRPC_SRCS})

# Include directories for generated headers AND the PkgConfig gRPC headers
target_include_directories(server PRIVATE ${PROTO_SRC_DIR} ${GRPC_INCLUDE_DIRS})

# Link libraries
# Note: We use ${GRPC_LIBRARIES} instead of gRPC::grpc++
target_link_libraries(server
    protobuf::libprotobuf
    ${GRPC_LIBRARIES}
    # Sometimes -lgrpc++_reflection is needed depending on implementation, 
    # but basic grpc++ usually covers it.
)