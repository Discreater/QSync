# Local Music Player

- Feature Name: Music Player
- Start Date: 2023-01-28

## Summary

First of all, QSync should be able to build a local music library from the local device, and manage the user's playlist. Of course, it should also be able to play music from the local library. Besides the playlist manager, we will also build the user manager. Although this is an initial model, we will be preparing other features such as playing music from the cloud.

## Motivation

The music player is one of the core parts of QSync. In the future we will add other features to QSync like playlist sync, video sync and others. Besides the playlist manager we will also build the user manager.

## Guide-level Explanation

### Server interface

We would use gRPC as the service interface. Below is the proto definition:

```proto
message Playlist {
  int64 id = 1;
  int64 owner_id = 2;
  repeated int64 track_ids = 3;
  string name = 5;
  string description = 6;
  google.protobuf.Timestamp created_at = 4;
  google.protobuf.Timestamp updated_at = 5;
}

message CurrentPlaylist {
  int64 id = 1;
  int64 playlist_id = 2;
  int64 user_id = 3;
  int64 current_track_id = 4;
  bool playing = 5;
  google.protobuf.Timestamp started_at = 6;
}

message Track {
  int64 id = 1;
  string name = 2;
  string artist = 3;
  string album = 4;
  optional LocalSource local_src = 5;
  optional NeteaseSource netease_src = 6;
}

// TODO: add fields
message NeteaseSource {
}

message LocalSource {
  srring path = 1;
}

message User {
  int64 id = 1;
  string name = 2;
  string current_playlist_id = 3;
}

message CreatePlaylistRequest {
  Playlist playlist = 1;
}

message CreatePlaylistResponse {
  Playlist playlist = 1;
}

message DeletePlaylistsRequest {
  repeated int64 ids = 1;
}

// Query playlist by user id and track id
message PlaylistQuery {
  // If empty, query all playlists
  int64 user_id = 1;
  // If empty, query all playlists
  int64 track_id = 2;
  string name = 3;
}

message QueryPlaylistRequest {
  PlaylistQuery query = 1;
}

// Update playlist: add/remove tracks, update name and description
message UpdatePlaylistRequest {
  int64 id = 1;
  repeated int64 added_track_ids = 1;
  repeated int64 removed_track_ids = 2;
  optional string name = 3;
  optional string description = 4;
}

// Updated playlist will be returned in response
message UpdatePlaylistResponse {
  Playlist playlist = 1;
}

message CreateTrackRequest {
  Track track = 1;
}

message CreateTrackResponse {
  Track track = 1;
}

message TrackQuery {
  int64 playlist_id = 1;
  string name = 2;
  string artist = 3;
  string ablum = 4;
}

message QueryTracksRequest {
  TrackQuery query = 1;
}

message UpdateTrackRequest {
  int64 id = 1;
  string name = 2;
  string artist = 3;
  string album = 4;
  optional LocalSource local_src = 5;
  optional NeteaseSource netease_src = 6;
}

message UpdateTrackResponse {
  Track track = 1;
}

message DeleteTracksRequest {
  repeated int64 track_ids = 1;
}

message CreateUserRequest {
  User user = 1;
}

message CreateUserResponse {
  User user = 1;
}

message QueryUserRequest {
  int64 id = 1;
  string name = 2;
}

message UpdateUserRequest {
  int64 id = 1;
  string name = 2;
  int64 current_playlist_id = 3;
}

message UpdateUserResponse {
  User user = 1;
}

message DeleteUsersRequest {
  repeated int64 ids = 1;
}

service MusyncService {
  rpc createPlaylist(CreatePlaylistRequest) returns (CreatePlaylistResponse);
  rpc queryPlaylist(QueryPlaylistRequest) returns (stream Playlist);
  rpc deletePlaylists(DeletePlaylistsRequest) returns (stream Playlist);
  rpc updatePlaylist(UpdatePlaylistRequest) returns (UpdatePlaylistResponse);

  rpc createTrack(CreateTrackRequest) returns (CreateTrackResponse);
  rpc queryTracks(QueryTracksRequest) returns (stream Track);
  rpc updateTrack(UpdateTrackRequest) returns (UpdateTrackResponse);
  rpc deleteTracks(DeleteTracksRequest) returns (stream Track);

  rpc createUser(CreateUserRequest) returns (CreateUserResponse);
  rpc queryUser(QueryUserRequest) returns (stream User);
  rpc updateUser(UpdateUserRequest) returns (UpdateUserResponse);
  rpc deleteUsers(DeleteUsersRequest) returns (stream User);
}
```

### Database schema

We use sqlite as the database. Below is the schema:

```sql
CREATE TABLE playlists (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  owner_id uuid NOT NULL,
  name varchar(255) NOT NULL,
  description text,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX playlists_owner_id_idx ON playlists (owner_id);
CREATE INDEX playlists_name_id_idx ON playlists (name);

CREATE TABLE playlists_tracks (
  playlist_id uuid NOT NULL,
  track_id uuid NOT NULL,
  PRIMARY KEY (playlist_id, track_id)
);
CREATE INDEX playlists_tracks_playlist_id_idx ON playlists_tracks (playlist_id);
CREATE INDEX playlists_tracks_track_id_idx ON playlists_tracks (track_id);

CREATE TABLE current_playlists (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  playlist_id uuid NOT NULL,
  user_id uuid NOT NULL,
  current_track_id uuid NOT NULL,
  playing boolean NOT NULL,
  started_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX current_playlists_playlist_id_idx ON current_playlists (playlist_id);
CREATE INDEX current_playlists_user_id_idx ON current_playlists (user_id);
CREATE INDEX current_playlists_current_track_id_idx ON current_playlists (current_track_id);

CREATE TABLE tracks (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  name text NOT NULL,
  artist text NOT NULL,
  album text NOT NULL,
  local_src_id uuid,
  netease_src_id uuid
);
CREATE INDEX tracks_name_idx ON tracks (name);
CREATE INDEX tracks_artist_idx ON tracks (artist);
create INDEX tracks_album_idx ON tracks (album);

CREATE TABLE local_srouces (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  path text NOT NULL
);

CREATE TABLE netease_srouces (
  id uuid NOT NULL DEFAULT uuid_generate_v4()
);

CREATE TABLE users (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  name varchar(255) NOT NULL,
  current_playlist_id uuid
);
create INDEX users_name_idx ON users (name);
```
