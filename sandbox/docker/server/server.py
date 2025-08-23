import grpc
from concurrent import futures
import time

# Import the generated gRPC files
import proto.service_pb2 as service_pb2
import proto.service_pb2_grpc as service_pb2_grpc

class MyService(service_pb2_grpc.MyServiceServicer):
    def SayHello(self, request, context):
        print(f"Received request: {request.name}")
        return service_pb2.HelloReply(message=f"Hello, {request.name}!")

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    service_pb2_grpc.add_MyServiceServicer_to_server(MyService(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    print("Server started on port 50051")
    server.wait_for_termination()

if __name__ == '__main__':
    serve()