import grpc
from concurrent import futures
import service_pb2
import service_pb2_grpc


class Greeter(service_pb2_grpc.GreeterServicer):
    def SayHello(self, request, context):
        print(f"Received request from {request.name}")
        return service_pb2.HelloReply(message=f"Hello, {request.name}! This is the Python backend.")


def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    service_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server.add_insecure_port("[::]:50051")
    print("Python gRPC server running on port 50051...")
    server.start()
    server.wait_for_termination()


if __name__ == "__main__":
    serve()
