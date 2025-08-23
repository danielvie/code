import * as grpc from '@grpc/grpc-js';
import { MyServiceClient } from './gen/service_grpc_pb';
import { HelloRequest } from './gen/service_pb';

// const client = new MyServiceClient('localhost:50051', grpc.credentials.createInsecure());
// client.ts
const client = new MyServiceClient('host.docker.internal:50051', grpc.credentials.createInsecure());

const request = new HelloRequest();
request.setName('World');

client.sayHello(request, (error, response) => {
  if (error) {
    console.error('Error:', error);
  } else {
    console.log('Response:', response.getMessage());
  }
});