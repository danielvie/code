import grpc
import sys
import os

# We assume the proto generated files will be created in the current directory
# This block ensures we can import them even if generated dynamically
import helloworld_pb2
import helloworld_pb2_grpc


def run():
    # Connect to localhost:50051
    # Because the server is in Docker mapped to 50051, localhost works fine here.
    print("Trying to connect to C++ server via Docker...")

    with grpc.insecure_channel("localhost:50051") as channel:
        stub = helloworld_pb2_grpc.GreeterStub(channel)

        name = "Python User"
        if len(sys.argv) > 1:
            name = sys.argv[1]

        try:
            response = stub.SayHello(helloworld_pb2.HelloRequest(name=name))
            print(f"Client received: {response.message}")
        except grpc.RpcError as e:
            print(f"RPC failed: {e.code()}")
            print(f"Details: {e.details()}")


if __name__ == "__main__":
    run()
