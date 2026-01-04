# protoc dependencies:

`protoc:`

https://github.com/protocolbuffers/protobuf/releases

`grpc-web:`

https://github.com/grpc/grpc-web/releases

`protobuf-javascript:`

https://github.com/protocolbuffers/protobuf-javascript/releases

download the protoc and the plugins, get the executables in the same path or point to the
executables manually:

```yaml
    - protoc -I=.. service.proto
        --plugin=protoc-gen-js=<path_to_protoc-gen-js.exe> `
        --plugin=protoc-gen-grpc-web=<path_to_protoc-gen-grpc-web.exe> `   
        --js_out=import_style=commonjs:.
        --grpc-web_out=import_style=commonjs,mode=grpcwebtext:. 
```

# web

```bash
bun create vite . --template react-ts
```

install 

```bash
bun add grpc-web google-protobuf
bun add -d @types/google-protobuf
```