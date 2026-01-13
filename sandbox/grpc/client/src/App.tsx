import { useState } from 'react';
// 1. Import the Transport for the browser
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
// 2. Import the generated Client and Client Interface
import { GreeterClient } from "./generated/service.client";

// 3. Configure the Transport (Point to Envoy on 8080)
const transport = new GrpcWebFetchTransport({
  baseUrl: "http://localhost:8080"
});

// 4. Create the Client instance
const client = new GreeterClient(transport);

export default function App() {
  const [responseMsg, setResponseMsg] = useState("");

  const callBackend = async () => {
    try {
      console.log("Sending request...");
      
      // 5. Make the Async Call (No more callbacks!)
      const { response } = await client.sayHello({
        name: "Bun User with protobuf-ts"
      });

      console.log("Success:", response.message);
      setResponseMsg(response.message);
      
    } catch (e: any) {
      console.error("Error:", e);
      setResponseMsg(`Error: ${e.message}`);
    }
  };

  return (
    <>
      <div className='flex w-screen h-screen items-center justify-center'>
        <div className='flex flex-col items-center gap-2'>
          <h1>Vite + protobuf-ts</h1>
          <button onClick={callBackend}>Execute Backend Function</button>
          <p>Result: {responseMsg}</p>
        </div>
      </div>
    </>
  );
}