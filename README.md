# QSync

![playing](docs/img/playing.png)

## Roadmap

### MuSync
- [x] Music Player
- [x] Local Music Library
- [x] Sync play through websocket
- [x] Unified Track ID
- [x] [Media Session API](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession)
- Search Music
   - [x] Local
   - [ ] Remote
- [ ] Remote Music Library
   - Netease
   - QQ Music
   - Bilibili
   - Migu
   - Youtube
- [ ] Lyric
- [ ] Download metadata

### Common

- [x] User Management
   - token auth
- [x] Websocket Message
- [ ] Add client id (Each websocket connection has a unique client id)


## FIXME
- [x] Need to consider loading time, pause local player when loading. (Preload?)
   - Renew the progress after track loaded.
- [ ] Refactor client player (init status, error handle, etc.)

## Tech Stack

- Server
   - axum: http & websocket
   - tonic: grpc
   - sea-orm: orm
   - sqlite: database
- Frontend
   - Vite
   - Vue
   - Typescript
   - TailwindCSS

## Build

- Create `.env` file and add env variables
   ```env
   DATABASE_URL=sqlite://./target/db.sqlite3
   JWT_SECRET=JJJWWW
   ```

- Generate typescript protobuf client, if you change the proto files in `protos/`. Rust grpc server code will audo generated when building.
   ```bash
   # windows
   protoc --plugin=protoc-gen-ts_proto=".\\node_modules\\.bin\\protoc-gen-ts_proto.cmd" --ts_proto_out=./src/generated --ts_proto_opt=esModuleInterop=true --ts_proto_opt=outputClientImpl=grpc-web ./protos/musync.proto
   # others
   protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./src/generated --ts_proto_opt=esModuleInterop=true --ts_proto_opt=outputClientImpl=grpc-web ./protos/musync.proto
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

## Run in Tauri client
```
pnpm tauri dev
```
