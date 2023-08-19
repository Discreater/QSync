# QSync

### MuSync
![playing](docs/img/playing.png)
- [x] Music Player
- [x] Local Music Library
- [] Unified Track ID
- [] Remote Music Library
   - Netease
   - QQ Music
   - Bilibili
   - Migu
   - Youtube
- [] Lyric
- [] Download metadata

## Tech Stack

- Tauri
- Frontend
   - Vite
   - Vue
   - Typescript
   - TailwindCSS

### Contributor

- generate typescript protobuf client
```bash
# windows
protoc --plugin=protoc-gen-ts_proto=".\\node_modules\\.bin\\protoc-gen-ts_proto.cmd" --ts_proto_out=./src/generated --ts_proto_opt=esModuleInterop=true --ts_proto_opt=outputClientImpl=grpc-web ./protos/musync.proto
# others
protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./src/generated --ts_proto_opt=esModuleInterop=true --ts_proto_opt=outputClientImpl=grpc-web ./protos/musync.proto
```