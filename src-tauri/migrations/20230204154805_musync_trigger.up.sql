CREATE TRIGGER playlist_trigger AFTER INSERT ON playlists
BEGIN
  INSERT INTO users_playlists (user_id, playlist_id) VALUES (NEW.owner_id, NEW.id);
END;
