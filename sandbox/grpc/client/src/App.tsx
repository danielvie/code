// ./src/App.tsx

import { useState } from 'react';
// The TypeScript plugin usually capitalizes the S in Service
import { GreeterClient } from './generated/ServiceServiceClientPb';
import * as ServiceProto from './generated/service_pb';

const client = new GreeterClient('http://localhost:8080');

export default function App() {
  const [responseMsg, setResponseMsg] = useState("");

  const callBackend = () => {
    const request = new ServiceProto.HelloRequest();
    request.setName('Bun User');

    console.log("Calling gRPC via Envoy...");

    client.sayHello(request, {}, (err, response: ServiceProto.HelloReply) => {
      if (err) {
        console.error("gRPC Error:", err.message);
        setResponseMsg("Error: Check Console");
        return;
      }
      const text = response.getMessage();
      console.log("Response:", text);
      setResponseMsg(text);
    });
  };

  return (
    <div style={{ padding: '20px' }}>
      <button onClick={callBackend}>Execute Backend Function</button>
      <p>Result: {responseMsg}</p>
    </div>
  );
}