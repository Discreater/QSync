CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  name varchar(255) NOT NULL
);
create INDEX IF NOT EXISTS users_name_idx ON users (name);

CREATE TABLE IF NOT EXISTS playlists (
  id INTEGER PRIMARY KEY,
  owner_id INTEGER NOT NULL,
  name varchar(255) NOT NULL,
  description text,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(owner_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS playlists_owner_id_idx ON playlists (owner_id);
CREATE INDEX IF NOT EXISTS playlists_name_id_idx ON playlists (name);

CREATE TABLE IF NOT EXISTS local_srouces (
  id INTEGER PRIMARY KEY,
  path text NOT NULL
);

CREATE TABLE IF NOT EXISTS netease_srouces (
  id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS tracks (
  id INTEGER PRIMARY KEY,
  name text NOT NULL,
  artist text NOT NULL,
  album text NOT NULL,
  local_src_id INTEGER,
  netease_src_id INTEGER,
  FOREIGN KEY(local_src_id) REFERENCES local_srouces(id) ON DELETE SET NULL,
  FOREIGN KEY(netease_src_id) REFERENCES netease_srouces(id) ON DELETE SET NULL
);
CREATE INDEX IF NOT EXISTS tracks_name_idx ON tracks (name);
CREATE INDEX IF NOT EXISTS tracks_artist_idx ON tracks (artist);
create INDEX IF NOT EXISTS tracks_album_idx ON tracks (album);

CREATE TABLE IF NOT EXISTS users_playlists (
  user_id INTEGER NOT NULL,
  playlist_id INTEGER NOT NULL,
  PRIMARY KEY (user_id, playlist_id),
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS playlists_tracks (
  playlist_id INTEGER NOT NULL,
  track_id INTEGER NOT NULL,
  PRIMARY KEY (playlist_id, track_id),
  FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
  FOREIGN KEY(track_id) REFERENCES tracks(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS playlists_tracks_playlist_id_idx ON playlists_tracks (playlist_id);
CREATE INDEX IF NOT EXISTS playlists_tracks_track_id_idx ON playlists_tracks (track_id);

CREATE TABLE IF NOT EXISTS current_playlists (
  id INTEGER PRIMARY KEY,
  playlist_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  current_track_id INTEGER NOT NULL,
  playing boolean NOT NULL,
  started_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY(current_track_id) REFERENCES tracks(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS current_playlists_playlist_id_idx ON current_playlists (playlist_id);
CREATE INDEX IF NOT EXISTS current_playlists_user_id_idx ON current_playlists (user_id);
CREATE INDEX IF NOT EXISTS current_playlists_current_track_id_idx ON current_playlists (current_track_id);
