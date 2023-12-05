# QSync

![playing](docs/img/playing.png)

You can download the prebuilt installer from the newest successful build of the [Github Actions](https://github.com/Discreater/QSync/actions). (Only Windows is supported now)

## Tech Stack

- Server
   - axum: http & websocket
   - tonic: grpc
   - sea-orm: orm
   - sqlite: database
- Frontend
   - Vue
   - Vite
   - Typescript
   - TailwindCSS

## Build

- Create `.env` file and add env variables
   ```env
   DATABASE_URL=sqlite://./target/db.sqlite3
   JWT_SECRET=JJJWWW
   ```

- Generate typescript protobuf client, if you have modified the proto files in `protos/`. Rust grpc server code will audo generated when building.
   ```bash
   # windows
   protoc --plugin=protoc-gen-ts_proto=".\\node_modules\\.bin\\protoc-gen-ts_proto.cmd" --ts_proto_out=./src/generated --ts_proto_opt=esModuleInterop=true --ts_proto_opt=outputClientImpl=grpc-web ./protos/musync.proto
   # others
   protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./src/generated --ts_proto_opt=esModuleInterop=true --ts_proto_opt=outputClientImpl=grpc-web ./protos/musync.proto
   ```

## Run in Tauri client
```
pnpm tauri dev
```

## Run in Web

Server will run on `localhost:8396`.

Login with user `Kerse`.

### Server
```
cd src-tauri/server/
cargo run --example server
```

### Web
```
pnpm dev
```
