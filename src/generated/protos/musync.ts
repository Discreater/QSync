/* eslint-disable */
import { grpc } from "@improbable-eng/grpc-web";
import { BrowserHeaders } from "browser-headers";
import _m0 from "protobufjs/minimal";
import { Observable } from "rxjs";
import { share } from "rxjs/operators";
import { Timestamp } from "../google/protobuf/timestamp";

export const protobufPackage = "musync";

/** Playlist */
export interface Playlist {
  /** unique id for the playlist */
  id: number;
  /** id of the owner of the playlist */
  ownerId: number;
  /** ids of tracks in the playlist */
  trackIds: number[];
  /** name of the playlist */
  name: string;
  /** description of the playlist */
  description: string;
  /** time when the playlist is created */
  createdAt:
    | Date
    | undefined;
  /** time of the last update */
  updatedAt: Date | undefined;
}

/** CurrentPlaylist controller */
export interface PlayQueue {
  /** unique id for the current playlist */
  id: number;
  /** ids of tracks in the playlist */
  trackIds: number[];
  /** index of the current playing track in the playlist */
  position: number;
  /** is the playlist playing */
  playing: boolean;
  /** time when the current track started playing. */
  startedAt:
    | Date
    | undefined;
  /** progress of the current track when the play queu is paused. In milliseconds */
  pausedAt: number;
}

/** Track */
export interface Track {
  /** unique id for the track */
  id: number;
  /** title of the track */
  title: string;
  /** artist of the track */
  artist?:
    | string
    | undefined;
  /** album of the track */
  album?:
    | string
    | undefined;
  /** duration of the track in milliseconds */
  duration?:
    | number
    | undefined;
  /** genre of the track */
  genre?:
    | string
    | undefined;
  /** year of the track */
  year?:
    | number
    | undefined;
  /** time when the track is created */
  createdAt:
    | Date
    | undefined;
  /** time of the last update */
  updatedAt:
    | Date
    | undefined;
  /** local source of the track */
  localSrc?:
    | LocalSource
    | undefined;
  /** netease source of the track */
  neteaseSrc?: NeteaseSource | undefined;
}

/** NeteaseSource, not implemented yet */
export interface NeteaseSource {
  /** id of the track in netease */
  id: string;
}

/** LocalSource */
export interface LocalSource {
  /** path of the track */
  path: string;
}

/** User */
export interface User {
  /** unique id for the user */
  id: number;
  /** name of the user */
  name: string;
  /** time when the user is created */
  createdAt:
    | Date
    | undefined;
  /** time of the last update */
  updatedAt: Date | undefined;
}

/** Token */
export interface Token {
  data: string;
  type: string;
}

/** Cover */
export interface Picture {
  /** picture type according to ID3v2 APIC */
  picType?: string | undefined;
  mimeType: string;
  description?:
    | string
    | undefined;
  /** base64 picture data */
  data: string;
}

/** Create playlist request */
export interface CreatePlaylistRequest {
  /** ids of tracks in the playlist */
  trackIds: number[];
  /** name of the playlist */
  name: string;
  /** description of the playlist */
  description: string;
}

/** Create playlist response */
export interface CreatePlaylistResponse {
  /** Created playlist */
  playlist: Playlist | undefined;
}

/** Delete playlist request */
export interface DeletePlaylistsRequest {
  /** Ids of playlists to be deleted */
  ids: number[];
}

/** Query playlist by user id and track id */
export interface QueryPlaylistsRequest {
  /** which user has the playlist */
  userId?:
    | number
    | undefined;
  /** Query by contained track id */
  trackId?:
    | number
    | undefined;
  /** Query by name */
  name?: string | undefined;
}

/** Update playlist: add/remove tracks, update name and description */
export interface UpdatePlaylistRequest {
  /** Id of the playlist to be updated */
  id: number;
  /** Ids of tracks to be added */
  addedTrackIds: number[];
  /** Ids of tracks to be removed */
  removedTrackIds: number[];
  /** New name of the playlist */
  name?:
    | string
    | undefined;
  /** New description of the playlist */
  description?: string | undefined;
}

/** Updated playlist will be returned in response */
export interface UpdatePlaylistResponse {
  /** Updated playlist */
  playlist: Playlist | undefined;
}

/** Create track request */
export interface CreateTrackRequest {
  /** Track to be created */
  track: Track | undefined;
}

/** Create track response */
export interface CreateTrackResponse {
  /** Created track */
  track: Track | undefined;
}

/** Query track */
export interface QueryTracksRequest {
  /** Query by which playlist contains the track */
  playlistId?:
    | number
    | undefined;
  /** Query by title */
  title?:
    | string
    | undefined;
  /** Query by artist */
  artist?:
    | string
    | undefined;
  /** Query by album */
  album?:
    | string
    | undefined;
  /** Query by genre */
  genre?:
    | string
    | undefined;
  /** Query by year */
  year?: number | undefined;
}

export interface TrackUpdate {
  /** Id of the track to be updated */
  id: number;
  /** New title of the track */
  title?:
    | string
    | undefined;
  /** New artist of the track */
  artist?:
    | string
    | undefined;
  /** New album of the track */
  album?: string | undefined;
  duration?:
    | number
    | undefined;
  /** New genre of the track */
  genre?:
    | string
    | undefined;
  /** New year of the track */
  year?:
    | number
    | undefined;
  /** New local source of the track */
  localSrc?:
    | LocalSource
    | undefined;
  /** New netease source of the track */
  neteaseSrc?: NeteaseSource | undefined;
}

/** Update track request */
export interface UpdateTrackRequest {
  update: TrackUpdate | undefined;
}

/** Update track response */
export interface UpdateTrackResponse {
  /** Updated track */
  track: Track | undefined;
}

/** Delete tracks request */
export interface DeleteTracksRequest {
  /** Ids of tracks to be deleted */
  trackIds: number[];
}

/** Create user request */
export interface CreateUserRequest {
  /** User to be created */
  user: User | undefined;
}

/** Create user response */
export interface CreateUserResponse {
  /** Created user */
  user: User | undefined;
}

/** Query user */
export interface QueryUsersRequest {
  /** Query by name */
  name?: string | undefined;
}

export interface UserUpdate {
  /** Id of the user to be updated */
  id: number;
  /** New name of the user */
  name: string;
}

/** Update user request */
export interface UpdateUserRequest {
  update: UserUpdate | undefined;
}

/** Update user response */
export interface UpdateUserResponse {
  /** Updated user */
  user: User | undefined;
}

/** Delete users request */
export interface DeleteUsersRequest {
  /** Ids of users to be deleted */
  ids: number[];
}

export interface LoginRequest {
  name: string;
  type: string;
}

export interface AddLocalFolderRequest {
  path: string;
}

export interface RemoveLocalFolderRequest {
  path: string;
}

export interface AddLocalFolderResponse {
  addedTracks: number;
}

export interface RemoveLocalFolderResponse {
  removedTracks: number;
}

export interface GetTrackRequest {
  id: number;
}

export interface GetTrackCoverRequest {
  trackId: number;
  coverIdx?: number | undefined;
}

export interface QueryLocalFoldersRequest {
}

export interface LocalFolder {
  path: string;
}

export interface CreatePlayQueueRequest {
  trackIds: number[];
}

export interface CreatePlayQueueResponse {
  playQueueId: number;
}

/** Currently, users can only get thier own play queue */
export interface GetPlayQueueRequest {
}

export interface UpdatePlayerRequest {
  /**
   * is the update triggerred by user action.
   * If this is false, it won't be broadcasted to other users.
   * Otherwise, it will be broadcasted to other users.
   */
  manul: boolean;
  /** index of the current playing track in the playlist */
  position?:
    | number
    | undefined;
  /** is the playlist playing */
  playing?:
    | boolean
    | undefined;
  /** Move the seek bar to the specified position. */
  progress?: number | undefined;
}

export interface UpdatePlayQueueEvent {
  trackIds: number[];
}

export interface UpdatePlayerEvent {
  position: number;
  playing: boolean;
  progress: number;
}

export interface SearchAllRequest {
  query: string;
}

export interface SearchAllResponse {
  tracks: Track[];
}

function createBasePlaylist(): Playlist {
  return { id: 0, ownerId: 0, trackIds: [], name: "", description: "", createdAt: undefined, updatedAt: undefined };
}

export const Playlist = {
  encode(message: Playlist, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    if (message.ownerId !== 0) {
      writer.uint32(16).int32(message.ownerId);
    }
    writer.uint32(26).fork();
    for (const v of message.trackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.name !== "") {
      writer.uint32(34).string(message.name);
    }
    if (message.description !== "") {
      writer.uint32(42).string(message.description);
    }
    if (message.createdAt !== undefined) {
      Timestamp.encode(toTimestamp(message.createdAt), writer.uint32(50).fork()).ldelim();
    }
    if (message.updatedAt !== undefined) {
      Timestamp.encode(toTimestamp(message.updatedAt), writer.uint32(58).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Playlist {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlaylist();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.ownerId = reader.int32();
          continue;
        case 3:
          if (tag === 24) {
            message.trackIds.push(reader.int32());

            continue;
          }

          if (tag === 26) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.trackIds.push(reader.int32());
            }

            continue;
          }

          break;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.name = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.description = reader.string();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.createdAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.updatedAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Playlist {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      ownerId: isSet(object.ownerId) ? Number(object.ownerId) : 0,
      trackIds: Array.isArray(object?.trackIds) ? object.trackIds.map((e: any) => Number(e)) : [],
      name: isSet(object.name) ? String(object.name) : "",
      description: isSet(object.description) ? String(object.description) : "",
      createdAt: isSet(object.createdAt) ? fromJsonTimestamp(object.createdAt) : undefined,
      updatedAt: isSet(object.updatedAt) ? fromJsonTimestamp(object.updatedAt) : undefined,
    };
  },

  toJSON(message: Playlist): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.ownerId !== 0) {
      obj.ownerId = Math.round(message.ownerId);
    }
    if (message.trackIds?.length) {
      obj.trackIds = message.trackIds.map((e) => Math.round(e));
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.description !== "") {
      obj.description = message.description;
    }
    if (message.createdAt !== undefined) {
      obj.createdAt = message.createdAt.toISOString();
    }
    if (message.updatedAt !== undefined) {
      obj.updatedAt = message.updatedAt.toISOString();
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Playlist>, I>>(base?: I): Playlist {
    return Playlist.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Playlist>, I>>(object: I): Playlist {
    const message = createBasePlaylist();
    message.id = object.id ?? 0;
    message.ownerId = object.ownerId ?? 0;
    message.trackIds = object.trackIds?.map((e) => e) || [];
    message.name = object.name ?? "";
    message.description = object.description ?? "";
    message.createdAt = object.createdAt ?? undefined;
    message.updatedAt = object.updatedAt ?? undefined;
    return message;
  },
};

function createBasePlayQueue(): PlayQueue {
  return { id: 0, trackIds: [], position: 0, playing: false, startedAt: undefined, pausedAt: 0 };
}

export const PlayQueue = {
  encode(message: PlayQueue, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    writer.uint32(18).fork();
    for (const v of message.trackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.position !== 0) {
      writer.uint32(24).uint32(message.position);
    }
    if (message.playing === true) {
      writer.uint32(32).bool(message.playing);
    }
    if (message.startedAt !== undefined) {
      Timestamp.encode(toTimestamp(message.startedAt), writer.uint32(42).fork()).ldelim();
    }
    if (message.pausedAt !== 0) {
      writer.uint32(48).uint32(message.pausedAt);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayQueue {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayQueue();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag === 16) {
            message.trackIds.push(reader.int32());

            continue;
          }

          if (tag === 18) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.trackIds.push(reader.int32());
            }

            continue;
          }

          break;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.position = reader.uint32();
          continue;
        case 4:
          if (tag !== 32) {
            break;
          }

          message.playing = reader.bool();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.startedAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
        case 6:
          if (tag !== 48) {
            break;
          }

          message.pausedAt = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PlayQueue {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      trackIds: Array.isArray(object?.trackIds) ? object.trackIds.map((e: any) => Number(e)) : [],
      position: isSet(object.position) ? Number(object.position) : 0,
      playing: isSet(object.playing) ? Boolean(object.playing) : false,
      startedAt: isSet(object.startedAt) ? fromJsonTimestamp(object.startedAt) : undefined,
      pausedAt: isSet(object.pausedAt) ? Number(object.pausedAt) : 0,
    };
  },

  toJSON(message: PlayQueue): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.trackIds?.length) {
      obj.trackIds = message.trackIds.map((e) => Math.round(e));
    }
    if (message.position !== 0) {
      obj.position = Math.round(message.position);
    }
    if (message.playing === true) {
      obj.playing = message.playing;
    }
    if (message.startedAt !== undefined) {
      obj.startedAt = message.startedAt.toISOString();
    }
    if (message.pausedAt !== 0) {
      obj.pausedAt = Math.round(message.pausedAt);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<PlayQueue>, I>>(base?: I): PlayQueue {
    return PlayQueue.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<PlayQueue>, I>>(object: I): PlayQueue {
    const message = createBasePlayQueue();
    message.id = object.id ?? 0;
    message.trackIds = object.trackIds?.map((e) => e) || [];
    message.position = object.position ?? 0;
    message.playing = object.playing ?? false;
    message.startedAt = object.startedAt ?? undefined;
    message.pausedAt = object.pausedAt ?? 0;
    return message;
  },
};

function createBaseTrack(): Track {
  return {
    id: 0,
    title: "",
    artist: undefined,
    album: undefined,
    duration: undefined,
    genre: undefined,
    year: undefined,
    createdAt: undefined,
    updatedAt: undefined,
    localSrc: undefined,
    neteaseSrc: undefined,
  };
}

export const Track = {
  encode(message: Track, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    if (message.title !== "") {
      writer.uint32(18).string(message.title);
    }
    if (message.artist !== undefined) {
      writer.uint32(26).string(message.artist);
    }
    if (message.album !== undefined) {
      writer.uint32(34).string(message.album);
    }
    if (message.duration !== undefined) {
      writer.uint32(40).uint32(message.duration);
    }
    if (message.genre !== undefined) {
      writer.uint32(50).string(message.genre);
    }
    if (message.year !== undefined) {
      writer.uint32(56).uint32(message.year);
    }
    if (message.createdAt !== undefined) {
      Timestamp.encode(toTimestamp(message.createdAt), writer.uint32(66).fork()).ldelim();
    }
    if (message.updatedAt !== undefined) {
      Timestamp.encode(toTimestamp(message.updatedAt), writer.uint32(74).fork()).ldelim();
    }
    if (message.localSrc !== undefined) {
      LocalSource.encode(message.localSrc, writer.uint32(82).fork()).ldelim();
    }
    if (message.neteaseSrc !== undefined) {
      NeteaseSource.encode(message.neteaseSrc, writer.uint32(90).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Track {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTrack();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.title = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.artist = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.album = reader.string();
          continue;
        case 5:
          if (tag !== 40) {
            break;
          }

          message.duration = reader.uint32();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.genre = reader.string();
          continue;
        case 7:
          if (tag !== 56) {
            break;
          }

          message.year = reader.uint32();
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.createdAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          message.updatedAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
        case 10:
          if (tag !== 82) {
            break;
          }

          message.localSrc = LocalSource.decode(reader, reader.uint32());
          continue;
        case 11:
          if (tag !== 90) {
            break;
          }

          message.neteaseSrc = NeteaseSource.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Track {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      title: isSet(object.title) ? String(object.title) : "",
      artist: isSet(object.artist) ? String(object.artist) : undefined,
      album: isSet(object.album) ? String(object.album) : undefined,
      duration: isSet(object.duration) ? Number(object.duration) : undefined,
      genre: isSet(object.genre) ? String(object.genre) : undefined,
      year: isSet(object.year) ? Number(object.year) : undefined,
      createdAt: isSet(object.createdAt) ? fromJsonTimestamp(object.createdAt) : undefined,
      updatedAt: isSet(object.updatedAt) ? fromJsonTimestamp(object.updatedAt) : undefined,
      localSrc: isSet(object.localSrc) ? LocalSource.fromJSON(object.localSrc) : undefined,
      neteaseSrc: isSet(object.neteaseSrc) ? NeteaseSource.fromJSON(object.neteaseSrc) : undefined,
    };
  },

  toJSON(message: Track): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.title !== "") {
      obj.title = message.title;
    }
    if (message.artist !== undefined) {
      obj.artist = message.artist;
    }
    if (message.album !== undefined) {
      obj.album = message.album;
    }
    if (message.duration !== undefined) {
      obj.duration = Math.round(message.duration);
    }
    if (message.genre !== undefined) {
      obj.genre = message.genre;
    }
    if (message.year !== undefined) {
      obj.year = Math.round(message.year);
    }
    if (message.createdAt !== undefined) {
      obj.createdAt = message.createdAt.toISOString();
    }
    if (message.updatedAt !== undefined) {
      obj.updatedAt = message.updatedAt.toISOString();
    }
    if (message.localSrc !== undefined) {
      obj.localSrc = LocalSource.toJSON(message.localSrc);
    }
    if (message.neteaseSrc !== undefined) {
      obj.neteaseSrc = NeteaseSource.toJSON(message.neteaseSrc);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Track>, I>>(base?: I): Track {
    return Track.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Track>, I>>(object: I): Track {
    const message = createBaseTrack();
    message.id = object.id ?? 0;
    message.title = object.title ?? "";
    message.artist = object.artist ?? undefined;
    message.album = object.album ?? undefined;
    message.duration = object.duration ?? undefined;
    message.genre = object.genre ?? undefined;
    message.year = object.year ?? undefined;
    message.createdAt = object.createdAt ?? undefined;
    message.updatedAt = object.updatedAt ?? undefined;
    message.localSrc = (object.localSrc !== undefined && object.localSrc !== null)
      ? LocalSource.fromPartial(object.localSrc)
      : undefined;
    message.neteaseSrc = (object.neteaseSrc !== undefined && object.neteaseSrc !== null)
      ? NeteaseSource.fromPartial(object.neteaseSrc)
      : undefined;
    return message;
  },
};

function createBaseNeteaseSource(): NeteaseSource {
  return { id: "" };
}

export const NeteaseSource = {
  encode(message: NeteaseSource, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): NeteaseSource {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseNeteaseSource();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.id = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): NeteaseSource {
    return { id: isSet(object.id) ? String(object.id) : "" };
  },

  toJSON(message: NeteaseSource): unknown {
    const obj: any = {};
    if (message.id !== "") {
      obj.id = message.id;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<NeteaseSource>, I>>(base?: I): NeteaseSource {
    return NeteaseSource.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<NeteaseSource>, I>>(object: I): NeteaseSource {
    const message = createBaseNeteaseSource();
    message.id = object.id ?? "";
    return message;
  },
};

function createBaseLocalSource(): LocalSource {
  return { path: "" };
}

export const LocalSource = {
  encode(message: LocalSource, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LocalSource {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLocalSource();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): LocalSource {
    return { path: isSet(object.path) ? String(object.path) : "" };
  },

  toJSON(message: LocalSource): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<LocalSource>, I>>(base?: I): LocalSource {
    return LocalSource.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<LocalSource>, I>>(object: I): LocalSource {
    const message = createBaseLocalSource();
    message.path = object.path ?? "";
    return message;
  },
};

function createBaseUser(): User {
  return { id: 0, name: "", createdAt: undefined, updatedAt: undefined };
}

export const User = {
  encode(message: User, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.createdAt !== undefined) {
      Timestamp.encode(toTimestamp(message.createdAt), writer.uint32(26).fork()).ldelim();
    }
    if (message.updatedAt !== undefined) {
      Timestamp.encode(toTimestamp(message.updatedAt), writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): User {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUser();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.name = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.createdAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.updatedAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): User {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      name: isSet(object.name) ? String(object.name) : "",
      createdAt: isSet(object.createdAt) ? fromJsonTimestamp(object.createdAt) : undefined,
      updatedAt: isSet(object.updatedAt) ? fromJsonTimestamp(object.updatedAt) : undefined,
    };
  },

  toJSON(message: User): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.createdAt !== undefined) {
      obj.createdAt = message.createdAt.toISOString();
    }
    if (message.updatedAt !== undefined) {
      obj.updatedAt = message.updatedAt.toISOString();
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<User>, I>>(base?: I): User {
    return User.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<User>, I>>(object: I): User {
    const message = createBaseUser();
    message.id = object.id ?? 0;
    message.name = object.name ?? "";
    message.createdAt = object.createdAt ?? undefined;
    message.updatedAt = object.updatedAt ?? undefined;
    return message;
  },
};

function createBaseToken(): Token {
  return { data: "", type: "" };
}

export const Token = {
  encode(message: Token, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.data !== "") {
      writer.uint32(10).string(message.data);
    }
    if (message.type !== "") {
      writer.uint32(18).string(message.type);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Token {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseToken();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.data = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.type = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Token {
    return { data: isSet(object.data) ? String(object.data) : "", type: isSet(object.type) ? String(object.type) : "" };
  },

  toJSON(message: Token): unknown {
    const obj: any = {};
    if (message.data !== "") {
      obj.data = message.data;
    }
    if (message.type !== "") {
      obj.type = message.type;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Token>, I>>(base?: I): Token {
    return Token.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Token>, I>>(object: I): Token {
    const message = createBaseToken();
    message.data = object.data ?? "";
    message.type = object.type ?? "";
    return message;
  },
};

function createBasePicture(): Picture {
  return { picType: undefined, mimeType: "", description: undefined, data: "" };
}

export const Picture = {
  encode(message: Picture, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.picType !== undefined) {
      writer.uint32(10).string(message.picType);
    }
    if (message.mimeType !== "") {
      writer.uint32(18).string(message.mimeType);
    }
    if (message.description !== undefined) {
      writer.uint32(26).string(message.description);
    }
    if (message.data !== "") {
      writer.uint32(34).string(message.data);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Picture {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePicture();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.picType = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.mimeType = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.description = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.data = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Picture {
    return {
      picType: isSet(object.picType) ? String(object.picType) : undefined,
      mimeType: isSet(object.mimeType) ? String(object.mimeType) : "",
      description: isSet(object.description) ? String(object.description) : undefined,
      data: isSet(object.data) ? String(object.data) : "",
    };
  },

  toJSON(message: Picture): unknown {
    const obj: any = {};
    if (message.picType !== undefined) {
      obj.picType = message.picType;
    }
    if (message.mimeType !== "") {
      obj.mimeType = message.mimeType;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.data !== "") {
      obj.data = message.data;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Picture>, I>>(base?: I): Picture {
    return Picture.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Picture>, I>>(object: I): Picture {
    const message = createBasePicture();
    message.picType = object.picType ?? undefined;
    message.mimeType = object.mimeType ?? "";
    message.description = object.description ?? undefined;
    message.data = object.data ?? "";
    return message;
  },
};

function createBaseCreatePlaylistRequest(): CreatePlaylistRequest {
  return { trackIds: [], name: "", description: "" };
}

export const CreatePlaylistRequest = {
  encode(message: CreatePlaylistRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.trackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.description !== "") {
      writer.uint32(26).string(message.description);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreatePlaylistRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreatePlaylistRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.trackIds.push(reader.int32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.trackIds.push(reader.int32());
            }

            continue;
          }

          break;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.name = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.description = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreatePlaylistRequest {
    return {
      trackIds: Array.isArray(object?.trackIds) ? object.trackIds.map((e: any) => Number(e)) : [],
      name: isSet(object.name) ? String(object.name) : "",
      description: isSet(object.description) ? String(object.description) : "",
    };
  },

  toJSON(message: CreatePlaylistRequest): unknown {
    const obj: any = {};
    if (message.trackIds?.length) {
      obj.trackIds = message.trackIds.map((e) => Math.round(e));
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.description !== "") {
      obj.description = message.description;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreatePlaylistRequest>, I>>(base?: I): CreatePlaylistRequest {
    return CreatePlaylistRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreatePlaylistRequest>, I>>(object: I): CreatePlaylistRequest {
    const message = createBaseCreatePlaylistRequest();
    message.trackIds = object.trackIds?.map((e) => e) || [];
    message.name = object.name ?? "";
    message.description = object.description ?? "";
    return message;
  },
};

function createBaseCreatePlaylistResponse(): CreatePlaylistResponse {
  return { playlist: undefined };
}

export const CreatePlaylistResponse = {
  encode(message: CreatePlaylistResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playlist !== undefined) {
      Playlist.encode(message.playlist, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreatePlaylistResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreatePlaylistResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.playlist = Playlist.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreatePlaylistResponse {
    return { playlist: isSet(object.playlist) ? Playlist.fromJSON(object.playlist) : undefined };
  },

  toJSON(message: CreatePlaylistResponse): unknown {
    const obj: any = {};
    if (message.playlist !== undefined) {
      obj.playlist = Playlist.toJSON(message.playlist);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreatePlaylistResponse>, I>>(base?: I): CreatePlaylistResponse {
    return CreatePlaylistResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreatePlaylistResponse>, I>>(object: I): CreatePlaylistResponse {
    const message = createBaseCreatePlaylistResponse();
    message.playlist = (object.playlist !== undefined && object.playlist !== null)
      ? Playlist.fromPartial(object.playlist)
      : undefined;
    return message;
  },
};

function createBaseDeletePlaylistsRequest(): DeletePlaylistsRequest {
  return { ids: [] };
}

export const DeletePlaylistsRequest = {
  encode(message: DeletePlaylistsRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.ids) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeletePlaylistsRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeletePlaylistsRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.ids.push(reader.int32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.ids.push(reader.int32());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): DeletePlaylistsRequest {
    return { ids: Array.isArray(object?.ids) ? object.ids.map((e: any) => Number(e)) : [] };
  },

  toJSON(message: DeletePlaylistsRequest): unknown {
    const obj: any = {};
    if (message.ids?.length) {
      obj.ids = message.ids.map((e) => Math.round(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<DeletePlaylistsRequest>, I>>(base?: I): DeletePlaylistsRequest {
    return DeletePlaylistsRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<DeletePlaylistsRequest>, I>>(object: I): DeletePlaylistsRequest {
    const message = createBaseDeletePlaylistsRequest();
    message.ids = object.ids?.map((e) => e) || [];
    return message;
  },
};

function createBaseQueryPlaylistsRequest(): QueryPlaylistsRequest {
  return { userId: undefined, trackId: undefined, name: undefined };
}

export const QueryPlaylistsRequest = {
  encode(message: QueryPlaylistsRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== undefined) {
      writer.uint32(8).int32(message.userId);
    }
    if (message.trackId !== undefined) {
      writer.uint32(16).int32(message.trackId);
    }
    if (message.name !== undefined) {
      writer.uint32(26).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryPlaylistsRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryPlaylistsRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.userId = reader.int32();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.trackId = reader.int32();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.name = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryPlaylistsRequest {
    return {
      userId: isSet(object.userId) ? Number(object.userId) : undefined,
      trackId: isSet(object.trackId) ? Number(object.trackId) : undefined,
      name: isSet(object.name) ? String(object.name) : undefined,
    };
  },

  toJSON(message: QueryPlaylistsRequest): unknown {
    const obj: any = {};
    if (message.userId !== undefined) {
      obj.userId = Math.round(message.userId);
    }
    if (message.trackId !== undefined) {
      obj.trackId = Math.round(message.trackId);
    }
    if (message.name !== undefined) {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryPlaylistsRequest>, I>>(base?: I): QueryPlaylistsRequest {
    return QueryPlaylistsRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryPlaylistsRequest>, I>>(object: I): QueryPlaylistsRequest {
    const message = createBaseQueryPlaylistsRequest();
    message.userId = object.userId ?? undefined;
    message.trackId = object.trackId ?? undefined;
    message.name = object.name ?? undefined;
    return message;
  },
};

function createBaseUpdatePlaylistRequest(): UpdatePlaylistRequest {
  return { id: 0, addedTrackIds: [], removedTrackIds: [], name: undefined, description: undefined };
}

export const UpdatePlaylistRequest = {
  encode(message: UpdatePlaylistRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    writer.uint32(18).fork();
    for (const v of message.addedTrackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    writer.uint32(26).fork();
    for (const v of message.removedTrackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.name !== undefined) {
      writer.uint32(34).string(message.name);
    }
    if (message.description !== undefined) {
      writer.uint32(42).string(message.description);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdatePlaylistRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdatePlaylistRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag === 16) {
            message.addedTrackIds.push(reader.int32());

            continue;
          }

          if (tag === 18) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.addedTrackIds.push(reader.int32());
            }

            continue;
          }

          break;
        case 3:
          if (tag === 24) {
            message.removedTrackIds.push(reader.int32());

            continue;
          }

          if (tag === 26) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.removedTrackIds.push(reader.int32());
            }

            continue;
          }

          break;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.name = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.description = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdatePlaylistRequest {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      addedTrackIds: Array.isArray(object?.addedTrackIds) ? object.addedTrackIds.map((e: any) => Number(e)) : [],
      removedTrackIds: Array.isArray(object?.removedTrackIds) ? object.removedTrackIds.map((e: any) => Number(e)) : [],
      name: isSet(object.name) ? String(object.name) : undefined,
      description: isSet(object.description) ? String(object.description) : undefined,
    };
  },

  toJSON(message: UpdatePlaylistRequest): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.addedTrackIds?.length) {
      obj.addedTrackIds = message.addedTrackIds.map((e) => Math.round(e));
    }
    if (message.removedTrackIds?.length) {
      obj.removedTrackIds = message.removedTrackIds.map((e) => Math.round(e));
    }
    if (message.name !== undefined) {
      obj.name = message.name;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdatePlaylistRequest>, I>>(base?: I): UpdatePlaylistRequest {
    return UpdatePlaylistRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdatePlaylistRequest>, I>>(object: I): UpdatePlaylistRequest {
    const message = createBaseUpdatePlaylistRequest();
    message.id = object.id ?? 0;
    message.addedTrackIds = object.addedTrackIds?.map((e) => e) || [];
    message.removedTrackIds = object.removedTrackIds?.map((e) => e) || [];
    message.name = object.name ?? undefined;
    message.description = object.description ?? undefined;
    return message;
  },
};

function createBaseUpdatePlaylistResponse(): UpdatePlaylistResponse {
  return { playlist: undefined };
}

export const UpdatePlaylistResponse = {
  encode(message: UpdatePlaylistResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playlist !== undefined) {
      Playlist.encode(message.playlist, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdatePlaylistResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdatePlaylistResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.playlist = Playlist.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdatePlaylistResponse {
    return { playlist: isSet(object.playlist) ? Playlist.fromJSON(object.playlist) : undefined };
  },

  toJSON(message: UpdatePlaylistResponse): unknown {
    const obj: any = {};
    if (message.playlist !== undefined) {
      obj.playlist = Playlist.toJSON(message.playlist);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdatePlaylistResponse>, I>>(base?: I): UpdatePlaylistResponse {
    return UpdatePlaylistResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdatePlaylistResponse>, I>>(object: I): UpdatePlaylistResponse {
    const message = createBaseUpdatePlaylistResponse();
    message.playlist = (object.playlist !== undefined && object.playlist !== null)
      ? Playlist.fromPartial(object.playlist)
      : undefined;
    return message;
  },
};

function createBaseCreateTrackRequest(): CreateTrackRequest {
  return { track: undefined };
}

export const CreateTrackRequest = {
  encode(message: CreateTrackRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.track !== undefined) {
      Track.encode(message.track, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateTrackRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateTrackRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.track = Track.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreateTrackRequest {
    return { track: isSet(object.track) ? Track.fromJSON(object.track) : undefined };
  },

  toJSON(message: CreateTrackRequest): unknown {
    const obj: any = {};
    if (message.track !== undefined) {
      obj.track = Track.toJSON(message.track);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreateTrackRequest>, I>>(base?: I): CreateTrackRequest {
    return CreateTrackRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreateTrackRequest>, I>>(object: I): CreateTrackRequest {
    const message = createBaseCreateTrackRequest();
    message.track = (object.track !== undefined && object.track !== null) ? Track.fromPartial(object.track) : undefined;
    return message;
  },
};

function createBaseCreateTrackResponse(): CreateTrackResponse {
  return { track: undefined };
}

export const CreateTrackResponse = {
  encode(message: CreateTrackResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.track !== undefined) {
      Track.encode(message.track, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateTrackResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateTrackResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.track = Track.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreateTrackResponse {
    return { track: isSet(object.track) ? Track.fromJSON(object.track) : undefined };
  },

  toJSON(message: CreateTrackResponse): unknown {
    const obj: any = {};
    if (message.track !== undefined) {
      obj.track = Track.toJSON(message.track);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreateTrackResponse>, I>>(base?: I): CreateTrackResponse {
    return CreateTrackResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreateTrackResponse>, I>>(object: I): CreateTrackResponse {
    const message = createBaseCreateTrackResponse();
    message.track = (object.track !== undefined && object.track !== null) ? Track.fromPartial(object.track) : undefined;
    return message;
  },
};

function createBaseQueryTracksRequest(): QueryTracksRequest {
  return {
    playlistId: undefined,
    title: undefined,
    artist: undefined,
    album: undefined,
    genre: undefined,
    year: undefined,
  };
}

export const QueryTracksRequest = {
  encode(message: QueryTracksRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playlistId !== undefined) {
      writer.uint32(8).int32(message.playlistId);
    }
    if (message.title !== undefined) {
      writer.uint32(18).string(message.title);
    }
    if (message.artist !== undefined) {
      writer.uint32(26).string(message.artist);
    }
    if (message.album !== undefined) {
      writer.uint32(34).string(message.album);
    }
    if (message.genre !== undefined) {
      writer.uint32(42).string(message.genre);
    }
    if (message.year !== undefined) {
      writer.uint32(48).uint32(message.year);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryTracksRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryTracksRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.playlistId = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.title = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.artist = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.album = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.genre = reader.string();
          continue;
        case 6:
          if (tag !== 48) {
            break;
          }

          message.year = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryTracksRequest {
    return {
      playlistId: isSet(object.playlistId) ? Number(object.playlistId) : undefined,
      title: isSet(object.title) ? String(object.title) : undefined,
      artist: isSet(object.artist) ? String(object.artist) : undefined,
      album: isSet(object.album) ? String(object.album) : undefined,
      genre: isSet(object.genre) ? String(object.genre) : undefined,
      year: isSet(object.year) ? Number(object.year) : undefined,
    };
  },

  toJSON(message: QueryTracksRequest): unknown {
    const obj: any = {};
    if (message.playlistId !== undefined) {
      obj.playlistId = Math.round(message.playlistId);
    }
    if (message.title !== undefined) {
      obj.title = message.title;
    }
    if (message.artist !== undefined) {
      obj.artist = message.artist;
    }
    if (message.album !== undefined) {
      obj.album = message.album;
    }
    if (message.genre !== undefined) {
      obj.genre = message.genre;
    }
    if (message.year !== undefined) {
      obj.year = Math.round(message.year);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryTracksRequest>, I>>(base?: I): QueryTracksRequest {
    return QueryTracksRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryTracksRequest>, I>>(object: I): QueryTracksRequest {
    const message = createBaseQueryTracksRequest();
    message.playlistId = object.playlistId ?? undefined;
    message.title = object.title ?? undefined;
    message.artist = object.artist ?? undefined;
    message.album = object.album ?? undefined;
    message.genre = object.genre ?? undefined;
    message.year = object.year ?? undefined;
    return message;
  },
};

function createBaseTrackUpdate(): TrackUpdate {
  return {
    id: 0,
    title: undefined,
    artist: undefined,
    album: undefined,
    duration: undefined,
    genre: undefined,
    year: undefined,
    localSrc: undefined,
    neteaseSrc: undefined,
  };
}

export const TrackUpdate = {
  encode(message: TrackUpdate, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    if (message.title !== undefined) {
      writer.uint32(18).string(message.title);
    }
    if (message.artist !== undefined) {
      writer.uint32(26).string(message.artist);
    }
    if (message.album !== undefined) {
      writer.uint32(34).string(message.album);
    }
    if (message.duration !== undefined) {
      writer.uint32(40).uint32(message.duration);
    }
    if (message.genre !== undefined) {
      writer.uint32(50).string(message.genre);
    }
    if (message.year !== undefined) {
      writer.uint32(56).uint32(message.year);
    }
    if (message.localSrc !== undefined) {
      LocalSource.encode(message.localSrc, writer.uint32(66).fork()).ldelim();
    }
    if (message.neteaseSrc !== undefined) {
      NeteaseSource.encode(message.neteaseSrc, writer.uint32(74).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TrackUpdate {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTrackUpdate();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.title = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.artist = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.album = reader.string();
          continue;
        case 5:
          if (tag !== 40) {
            break;
          }

          message.duration = reader.uint32();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.genre = reader.string();
          continue;
        case 7:
          if (tag !== 56) {
            break;
          }

          message.year = reader.uint32();
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.localSrc = LocalSource.decode(reader, reader.uint32());
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          message.neteaseSrc = NeteaseSource.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): TrackUpdate {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      title: isSet(object.title) ? String(object.title) : undefined,
      artist: isSet(object.artist) ? String(object.artist) : undefined,
      album: isSet(object.album) ? String(object.album) : undefined,
      duration: isSet(object.duration) ? Number(object.duration) : undefined,
      genre: isSet(object.genre) ? String(object.genre) : undefined,
      year: isSet(object.year) ? Number(object.year) : undefined,
      localSrc: isSet(object.localSrc) ? LocalSource.fromJSON(object.localSrc) : undefined,
      neteaseSrc: isSet(object.neteaseSrc) ? NeteaseSource.fromJSON(object.neteaseSrc) : undefined,
    };
  },

  toJSON(message: TrackUpdate): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.title !== undefined) {
      obj.title = message.title;
    }
    if (message.artist !== undefined) {
      obj.artist = message.artist;
    }
    if (message.album !== undefined) {
      obj.album = message.album;
    }
    if (message.duration !== undefined) {
      obj.duration = Math.round(message.duration);
    }
    if (message.genre !== undefined) {
      obj.genre = message.genre;
    }
    if (message.year !== undefined) {
      obj.year = Math.round(message.year);
    }
    if (message.localSrc !== undefined) {
      obj.localSrc = LocalSource.toJSON(message.localSrc);
    }
    if (message.neteaseSrc !== undefined) {
      obj.neteaseSrc = NeteaseSource.toJSON(message.neteaseSrc);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<TrackUpdate>, I>>(base?: I): TrackUpdate {
    return TrackUpdate.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<TrackUpdate>, I>>(object: I): TrackUpdate {
    const message = createBaseTrackUpdate();
    message.id = object.id ?? 0;
    message.title = object.title ?? undefined;
    message.artist = object.artist ?? undefined;
    message.album = object.album ?? undefined;
    message.duration = object.duration ?? undefined;
    message.genre = object.genre ?? undefined;
    message.year = object.year ?? undefined;
    message.localSrc = (object.localSrc !== undefined && object.localSrc !== null)
      ? LocalSource.fromPartial(object.localSrc)
      : undefined;
    message.neteaseSrc = (object.neteaseSrc !== undefined && object.neteaseSrc !== null)
      ? NeteaseSource.fromPartial(object.neteaseSrc)
      : undefined;
    return message;
  },
};

function createBaseUpdateTrackRequest(): UpdateTrackRequest {
  return { update: undefined };
}

export const UpdateTrackRequest = {
  encode(message: UpdateTrackRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.update !== undefined) {
      TrackUpdate.encode(message.update, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateTrackRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateTrackRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.update = TrackUpdate.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdateTrackRequest {
    return { update: isSet(object.update) ? TrackUpdate.fromJSON(object.update) : undefined };
  },

  toJSON(message: UpdateTrackRequest): unknown {
    const obj: any = {};
    if (message.update !== undefined) {
      obj.update = TrackUpdate.toJSON(message.update);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdateTrackRequest>, I>>(base?: I): UpdateTrackRequest {
    return UpdateTrackRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdateTrackRequest>, I>>(object: I): UpdateTrackRequest {
    const message = createBaseUpdateTrackRequest();
    message.update = (object.update !== undefined && object.update !== null)
      ? TrackUpdate.fromPartial(object.update)
      : undefined;
    return message;
  },
};

function createBaseUpdateTrackResponse(): UpdateTrackResponse {
  return { track: undefined };
}

export const UpdateTrackResponse = {
  encode(message: UpdateTrackResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.track !== undefined) {
      Track.encode(message.track, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateTrackResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateTrackResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.track = Track.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdateTrackResponse {
    return { track: isSet(object.track) ? Track.fromJSON(object.track) : undefined };
  },

  toJSON(message: UpdateTrackResponse): unknown {
    const obj: any = {};
    if (message.track !== undefined) {
      obj.track = Track.toJSON(message.track);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdateTrackResponse>, I>>(base?: I): UpdateTrackResponse {
    return UpdateTrackResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdateTrackResponse>, I>>(object: I): UpdateTrackResponse {
    const message = createBaseUpdateTrackResponse();
    message.track = (object.track !== undefined && object.track !== null) ? Track.fromPartial(object.track) : undefined;
    return message;
  },
};

function createBaseDeleteTracksRequest(): DeleteTracksRequest {
  return { trackIds: [] };
}

export const DeleteTracksRequest = {
  encode(message: DeleteTracksRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.trackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteTracksRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteTracksRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.trackIds.push(reader.int32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.trackIds.push(reader.int32());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): DeleteTracksRequest {
    return { trackIds: Array.isArray(object?.trackIds) ? object.trackIds.map((e: any) => Number(e)) : [] };
  },

  toJSON(message: DeleteTracksRequest): unknown {
    const obj: any = {};
    if (message.trackIds?.length) {
      obj.trackIds = message.trackIds.map((e) => Math.round(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<DeleteTracksRequest>, I>>(base?: I): DeleteTracksRequest {
    return DeleteTracksRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<DeleteTracksRequest>, I>>(object: I): DeleteTracksRequest {
    const message = createBaseDeleteTracksRequest();
    message.trackIds = object.trackIds?.map((e) => e) || [];
    return message;
  },
};

function createBaseCreateUserRequest(): CreateUserRequest {
  return { user: undefined };
}

export const CreateUserRequest = {
  encode(message: CreateUserRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.user !== undefined) {
      User.encode(message.user, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateUserRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateUserRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.user = User.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreateUserRequest {
    return { user: isSet(object.user) ? User.fromJSON(object.user) : undefined };
  },

  toJSON(message: CreateUserRequest): unknown {
    const obj: any = {};
    if (message.user !== undefined) {
      obj.user = User.toJSON(message.user);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreateUserRequest>, I>>(base?: I): CreateUserRequest {
    return CreateUserRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreateUserRequest>, I>>(object: I): CreateUserRequest {
    const message = createBaseCreateUserRequest();
    message.user = (object.user !== undefined && object.user !== null) ? User.fromPartial(object.user) : undefined;
    return message;
  },
};

function createBaseCreateUserResponse(): CreateUserResponse {
  return { user: undefined };
}

export const CreateUserResponse = {
  encode(message: CreateUserResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.user !== undefined) {
      User.encode(message.user, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateUserResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateUserResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.user = User.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreateUserResponse {
    return { user: isSet(object.user) ? User.fromJSON(object.user) : undefined };
  },

  toJSON(message: CreateUserResponse): unknown {
    const obj: any = {};
    if (message.user !== undefined) {
      obj.user = User.toJSON(message.user);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreateUserResponse>, I>>(base?: I): CreateUserResponse {
    return CreateUserResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreateUserResponse>, I>>(object: I): CreateUserResponse {
    const message = createBaseCreateUserResponse();
    message.user = (object.user !== undefined && object.user !== null) ? User.fromPartial(object.user) : undefined;
    return message;
  },
};

function createBaseQueryUsersRequest(): QueryUsersRequest {
  return { name: undefined };
}

export const QueryUsersRequest = {
  encode(message: QueryUsersRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== undefined) {
      writer.uint32(10).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryUsersRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryUsersRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryUsersRequest {
    return { name: isSet(object.name) ? String(object.name) : undefined };
  },

  toJSON(message: QueryUsersRequest): unknown {
    const obj: any = {};
    if (message.name !== undefined) {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryUsersRequest>, I>>(base?: I): QueryUsersRequest {
    return QueryUsersRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryUsersRequest>, I>>(object: I): QueryUsersRequest {
    const message = createBaseQueryUsersRequest();
    message.name = object.name ?? undefined;
    return message;
  },
};

function createBaseUserUpdate(): UserUpdate {
  return { id: 0, name: "" };
}

export const UserUpdate = {
  encode(message: UserUpdate, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UserUpdate {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUserUpdate();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.name = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UserUpdate {
    return { id: isSet(object.id) ? Number(object.id) : 0, name: isSet(object.name) ? String(object.name) : "" };
  },

  toJSON(message: UserUpdate): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UserUpdate>, I>>(base?: I): UserUpdate {
    return UserUpdate.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UserUpdate>, I>>(object: I): UserUpdate {
    const message = createBaseUserUpdate();
    message.id = object.id ?? 0;
    message.name = object.name ?? "";
    return message;
  },
};

function createBaseUpdateUserRequest(): UpdateUserRequest {
  return { update: undefined };
}

export const UpdateUserRequest = {
  encode(message: UpdateUserRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.update !== undefined) {
      UserUpdate.encode(message.update, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateUserRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateUserRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.update = UserUpdate.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdateUserRequest {
    return { update: isSet(object.update) ? UserUpdate.fromJSON(object.update) : undefined };
  },

  toJSON(message: UpdateUserRequest): unknown {
    const obj: any = {};
    if (message.update !== undefined) {
      obj.update = UserUpdate.toJSON(message.update);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdateUserRequest>, I>>(base?: I): UpdateUserRequest {
    return UpdateUserRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdateUserRequest>, I>>(object: I): UpdateUserRequest {
    const message = createBaseUpdateUserRequest();
    message.update = (object.update !== undefined && object.update !== null)
      ? UserUpdate.fromPartial(object.update)
      : undefined;
    return message;
  },
};

function createBaseUpdateUserResponse(): UpdateUserResponse {
  return { user: undefined };
}

export const UpdateUserResponse = {
  encode(message: UpdateUserResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.user !== undefined) {
      User.encode(message.user, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateUserResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateUserResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.user = User.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdateUserResponse {
    return { user: isSet(object.user) ? User.fromJSON(object.user) : undefined };
  },

  toJSON(message: UpdateUserResponse): unknown {
    const obj: any = {};
    if (message.user !== undefined) {
      obj.user = User.toJSON(message.user);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdateUserResponse>, I>>(base?: I): UpdateUserResponse {
    return UpdateUserResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdateUserResponse>, I>>(object: I): UpdateUserResponse {
    const message = createBaseUpdateUserResponse();
    message.user = (object.user !== undefined && object.user !== null) ? User.fromPartial(object.user) : undefined;
    return message;
  },
};

function createBaseDeleteUsersRequest(): DeleteUsersRequest {
  return { ids: [] };
}

export const DeleteUsersRequest = {
  encode(message: DeleteUsersRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.ids) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteUsersRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteUsersRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.ids.push(reader.int32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.ids.push(reader.int32());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): DeleteUsersRequest {
    return { ids: Array.isArray(object?.ids) ? object.ids.map((e: any) => Number(e)) : [] };
  },

  toJSON(message: DeleteUsersRequest): unknown {
    const obj: any = {};
    if (message.ids?.length) {
      obj.ids = message.ids.map((e) => Math.round(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<DeleteUsersRequest>, I>>(base?: I): DeleteUsersRequest {
    return DeleteUsersRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<DeleteUsersRequest>, I>>(object: I): DeleteUsersRequest {
    const message = createBaseDeleteUsersRequest();
    message.ids = object.ids?.map((e) => e) || [];
    return message;
  },
};

function createBaseLoginRequest(): LoginRequest {
  return { name: "", type: "" };
}

export const LoginRequest = {
  encode(message: LoginRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.type !== "") {
      writer.uint32(18).string(message.type);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LoginRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLoginRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.type = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): LoginRequest {
    return { name: isSet(object.name) ? String(object.name) : "", type: isSet(object.type) ? String(object.type) : "" };
  },

  toJSON(message: LoginRequest): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.type !== "") {
      obj.type = message.type;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<LoginRequest>, I>>(base?: I): LoginRequest {
    return LoginRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<LoginRequest>, I>>(object: I): LoginRequest {
    const message = createBaseLoginRequest();
    message.name = object.name ?? "";
    message.type = object.type ?? "";
    return message;
  },
};

function createBaseAddLocalFolderRequest(): AddLocalFolderRequest {
  return { path: "" };
}

export const AddLocalFolderRequest = {
  encode(message: AddLocalFolderRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AddLocalFolderRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAddLocalFolderRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AddLocalFolderRequest {
    return { path: isSet(object.path) ? String(object.path) : "" };
  },

  toJSON(message: AddLocalFolderRequest): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AddLocalFolderRequest>, I>>(base?: I): AddLocalFolderRequest {
    return AddLocalFolderRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AddLocalFolderRequest>, I>>(object: I): AddLocalFolderRequest {
    const message = createBaseAddLocalFolderRequest();
    message.path = object.path ?? "";
    return message;
  },
};

function createBaseRemoveLocalFolderRequest(): RemoveLocalFolderRequest {
  return { path: "" };
}

export const RemoveLocalFolderRequest = {
  encode(message: RemoveLocalFolderRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RemoveLocalFolderRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRemoveLocalFolderRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RemoveLocalFolderRequest {
    return { path: isSet(object.path) ? String(object.path) : "" };
  },

  toJSON(message: RemoveLocalFolderRequest): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RemoveLocalFolderRequest>, I>>(base?: I): RemoveLocalFolderRequest {
    return RemoveLocalFolderRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RemoveLocalFolderRequest>, I>>(object: I): RemoveLocalFolderRequest {
    const message = createBaseRemoveLocalFolderRequest();
    message.path = object.path ?? "";
    return message;
  },
};

function createBaseAddLocalFolderResponse(): AddLocalFolderResponse {
  return { addedTracks: 0 };
}

export const AddLocalFolderResponse = {
  encode(message: AddLocalFolderResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.addedTracks !== 0) {
      writer.uint32(8).uint32(message.addedTracks);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AddLocalFolderResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAddLocalFolderResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.addedTracks = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AddLocalFolderResponse {
    return { addedTracks: isSet(object.addedTracks) ? Number(object.addedTracks) : 0 };
  },

  toJSON(message: AddLocalFolderResponse): unknown {
    const obj: any = {};
    if (message.addedTracks !== 0) {
      obj.addedTracks = Math.round(message.addedTracks);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AddLocalFolderResponse>, I>>(base?: I): AddLocalFolderResponse {
    return AddLocalFolderResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AddLocalFolderResponse>, I>>(object: I): AddLocalFolderResponse {
    const message = createBaseAddLocalFolderResponse();
    message.addedTracks = object.addedTracks ?? 0;
    return message;
  },
};

function createBaseRemoveLocalFolderResponse(): RemoveLocalFolderResponse {
  return { removedTracks: 0 };
}

export const RemoveLocalFolderResponse = {
  encode(message: RemoveLocalFolderResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.removedTracks !== 0) {
      writer.uint32(8).uint32(message.removedTracks);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RemoveLocalFolderResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRemoveLocalFolderResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.removedTracks = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RemoveLocalFolderResponse {
    return { removedTracks: isSet(object.removedTracks) ? Number(object.removedTracks) : 0 };
  },

  toJSON(message: RemoveLocalFolderResponse): unknown {
    const obj: any = {};
    if (message.removedTracks !== 0) {
      obj.removedTracks = Math.round(message.removedTracks);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RemoveLocalFolderResponse>, I>>(base?: I): RemoveLocalFolderResponse {
    return RemoveLocalFolderResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RemoveLocalFolderResponse>, I>>(object: I): RemoveLocalFolderResponse {
    const message = createBaseRemoveLocalFolderResponse();
    message.removedTracks = object.removedTracks ?? 0;
    return message;
  },
};

function createBaseGetTrackRequest(): GetTrackRequest {
  return { id: 0 };
}

export const GetTrackRequest = {
  encode(message: GetTrackRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).int32(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetTrackRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetTrackRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.id = reader.int32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GetTrackRequest {
    return { id: isSet(object.id) ? Number(object.id) : 0 };
  },

  toJSON(message: GetTrackRequest): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<GetTrackRequest>, I>>(base?: I): GetTrackRequest {
    return GetTrackRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GetTrackRequest>, I>>(object: I): GetTrackRequest {
    const message = createBaseGetTrackRequest();
    message.id = object.id ?? 0;
    return message;
  },
};

function createBaseGetTrackCoverRequest(): GetTrackCoverRequest {
  return { trackId: 0, coverIdx: undefined };
}

export const GetTrackCoverRequest = {
  encode(message: GetTrackCoverRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.trackId !== 0) {
      writer.uint32(8).int32(message.trackId);
    }
    if (message.coverIdx !== undefined) {
      writer.uint32(16).uint32(message.coverIdx);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetTrackCoverRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetTrackCoverRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.trackId = reader.int32();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.coverIdx = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GetTrackCoverRequest {
    return {
      trackId: isSet(object.trackId) ? Number(object.trackId) : 0,
      coverIdx: isSet(object.coverIdx) ? Number(object.coverIdx) : undefined,
    };
  },

  toJSON(message: GetTrackCoverRequest): unknown {
    const obj: any = {};
    if (message.trackId !== 0) {
      obj.trackId = Math.round(message.trackId);
    }
    if (message.coverIdx !== undefined) {
      obj.coverIdx = Math.round(message.coverIdx);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<GetTrackCoverRequest>, I>>(base?: I): GetTrackCoverRequest {
    return GetTrackCoverRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GetTrackCoverRequest>, I>>(object: I): GetTrackCoverRequest {
    const message = createBaseGetTrackCoverRequest();
    message.trackId = object.trackId ?? 0;
    message.coverIdx = object.coverIdx ?? undefined;
    return message;
  },
};

function createBaseQueryLocalFoldersRequest(): QueryLocalFoldersRequest {
  return {};
}

export const QueryLocalFoldersRequest = {
  encode(_: QueryLocalFoldersRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryLocalFoldersRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryLocalFoldersRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): QueryLocalFoldersRequest {
    return {};
  },

  toJSON(_: QueryLocalFoldersRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryLocalFoldersRequest>, I>>(base?: I): QueryLocalFoldersRequest {
    return QueryLocalFoldersRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryLocalFoldersRequest>, I>>(_: I): QueryLocalFoldersRequest {
    const message = createBaseQueryLocalFoldersRequest();
    return message;
  },
};

function createBaseLocalFolder(): LocalFolder {
  return { path: "" };
}

export const LocalFolder = {
  encode(message: LocalFolder, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LocalFolder {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLocalFolder();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): LocalFolder {
    return { path: isSet(object.path) ? String(object.path) : "" };
  },

  toJSON(message: LocalFolder): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<LocalFolder>, I>>(base?: I): LocalFolder {
    return LocalFolder.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<LocalFolder>, I>>(object: I): LocalFolder {
    const message = createBaseLocalFolder();
    message.path = object.path ?? "";
    return message;
  },
};

function createBaseCreatePlayQueueRequest(): CreatePlayQueueRequest {
  return { trackIds: [] };
}

export const CreatePlayQueueRequest = {
  encode(message: CreatePlayQueueRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.trackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreatePlayQueueRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreatePlayQueueRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.trackIds.push(reader.int32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.trackIds.push(reader.int32());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreatePlayQueueRequest {
    return { trackIds: Array.isArray(object?.trackIds) ? object.trackIds.map((e: any) => Number(e)) : [] };
  },

  toJSON(message: CreatePlayQueueRequest): unknown {
    const obj: any = {};
    if (message.trackIds?.length) {
      obj.trackIds = message.trackIds.map((e) => Math.round(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreatePlayQueueRequest>, I>>(base?: I): CreatePlayQueueRequest {
    return CreatePlayQueueRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreatePlayQueueRequest>, I>>(object: I): CreatePlayQueueRequest {
    const message = createBaseCreatePlayQueueRequest();
    message.trackIds = object.trackIds?.map((e) => e) || [];
    return message;
  },
};

function createBaseCreatePlayQueueResponse(): CreatePlayQueueResponse {
  return { playQueueId: 0 };
}

export const CreatePlayQueueResponse = {
  encode(message: CreatePlayQueueResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playQueueId !== 0) {
      writer.uint32(8).int32(message.playQueueId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreatePlayQueueResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreatePlayQueueResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.playQueueId = reader.int32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CreatePlayQueueResponse {
    return { playQueueId: isSet(object.playQueueId) ? Number(object.playQueueId) : 0 };
  },

  toJSON(message: CreatePlayQueueResponse): unknown {
    const obj: any = {};
    if (message.playQueueId !== 0) {
      obj.playQueueId = Math.round(message.playQueueId);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CreatePlayQueueResponse>, I>>(base?: I): CreatePlayQueueResponse {
    return CreatePlayQueueResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CreatePlayQueueResponse>, I>>(object: I): CreatePlayQueueResponse {
    const message = createBaseCreatePlayQueueResponse();
    message.playQueueId = object.playQueueId ?? 0;
    return message;
  },
};

function createBaseGetPlayQueueRequest(): GetPlayQueueRequest {
  return {};
}

export const GetPlayQueueRequest = {
  encode(_: GetPlayQueueRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetPlayQueueRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetPlayQueueRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): GetPlayQueueRequest {
    return {};
  },

  toJSON(_: GetPlayQueueRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<GetPlayQueueRequest>, I>>(base?: I): GetPlayQueueRequest {
    return GetPlayQueueRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GetPlayQueueRequest>, I>>(_: I): GetPlayQueueRequest {
    const message = createBaseGetPlayQueueRequest();
    return message;
  },
};

function createBaseUpdatePlayerRequest(): UpdatePlayerRequest {
  return { manul: false, position: undefined, playing: undefined, progress: undefined };
}

export const UpdatePlayerRequest = {
  encode(message: UpdatePlayerRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.manul === true) {
      writer.uint32(8).bool(message.manul);
    }
    if (message.position !== undefined) {
      writer.uint32(16).uint32(message.position);
    }
    if (message.playing !== undefined) {
      writer.uint32(24).bool(message.playing);
    }
    if (message.progress !== undefined) {
      writer.uint32(32).uint32(message.progress);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdatePlayerRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdatePlayerRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.manul = reader.bool();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.position = reader.uint32();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.playing = reader.bool();
          continue;
        case 4:
          if (tag !== 32) {
            break;
          }

          message.progress = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdatePlayerRequest {
    return {
      manul: isSet(object.manul) ? Boolean(object.manul) : false,
      position: isSet(object.position) ? Number(object.position) : undefined,
      playing: isSet(object.playing) ? Boolean(object.playing) : undefined,
      progress: isSet(object.progress) ? Number(object.progress) : undefined,
    };
  },

  toJSON(message: UpdatePlayerRequest): unknown {
    const obj: any = {};
    if (message.manul === true) {
      obj.manul = message.manul;
    }
    if (message.position !== undefined) {
      obj.position = Math.round(message.position);
    }
    if (message.playing !== undefined) {
      obj.playing = message.playing;
    }
    if (message.progress !== undefined) {
      obj.progress = Math.round(message.progress);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdatePlayerRequest>, I>>(base?: I): UpdatePlayerRequest {
    return UpdatePlayerRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdatePlayerRequest>, I>>(object: I): UpdatePlayerRequest {
    const message = createBaseUpdatePlayerRequest();
    message.manul = object.manul ?? false;
    message.position = object.position ?? undefined;
    message.playing = object.playing ?? undefined;
    message.progress = object.progress ?? undefined;
    return message;
  },
};

function createBaseUpdatePlayQueueEvent(): UpdatePlayQueueEvent {
  return { trackIds: [] };
}

export const UpdatePlayQueueEvent = {
  encode(message: UpdatePlayQueueEvent, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.trackIds) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdatePlayQueueEvent {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdatePlayQueueEvent();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.trackIds.push(reader.int32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.trackIds.push(reader.int32());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdatePlayQueueEvent {
    return { trackIds: Array.isArray(object?.trackIds) ? object.trackIds.map((e: any) => Number(e)) : [] };
  },

  toJSON(message: UpdatePlayQueueEvent): unknown {
    const obj: any = {};
    if (message.trackIds?.length) {
      obj.trackIds = message.trackIds.map((e) => Math.round(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdatePlayQueueEvent>, I>>(base?: I): UpdatePlayQueueEvent {
    return UpdatePlayQueueEvent.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdatePlayQueueEvent>, I>>(object: I): UpdatePlayQueueEvent {
    const message = createBaseUpdatePlayQueueEvent();
    message.trackIds = object.trackIds?.map((e) => e) || [];
    return message;
  },
};

function createBaseUpdatePlayerEvent(): UpdatePlayerEvent {
  return { position: 0, playing: false, progress: 0 };
}

export const UpdatePlayerEvent = {
  encode(message: UpdatePlayerEvent, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.position !== 0) {
      writer.uint32(8).uint32(message.position);
    }
    if (message.playing === true) {
      writer.uint32(16).bool(message.playing);
    }
    if (message.progress !== 0) {
      writer.uint32(24).uint32(message.progress);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdatePlayerEvent {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdatePlayerEvent();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.position = reader.uint32();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.playing = reader.bool();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.progress = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UpdatePlayerEvent {
    return {
      position: isSet(object.position) ? Number(object.position) : 0,
      playing: isSet(object.playing) ? Boolean(object.playing) : false,
      progress: isSet(object.progress) ? Number(object.progress) : 0,
    };
  },

  toJSON(message: UpdatePlayerEvent): unknown {
    const obj: any = {};
    if (message.position !== 0) {
      obj.position = Math.round(message.position);
    }
    if (message.playing === true) {
      obj.playing = message.playing;
    }
    if (message.progress !== 0) {
      obj.progress = Math.round(message.progress);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UpdatePlayerEvent>, I>>(base?: I): UpdatePlayerEvent {
    return UpdatePlayerEvent.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UpdatePlayerEvent>, I>>(object: I): UpdatePlayerEvent {
    const message = createBaseUpdatePlayerEvent();
    message.position = object.position ?? 0;
    message.playing = object.playing ?? false;
    message.progress = object.progress ?? 0;
    return message;
  },
};

function createBaseSearchAllRequest(): SearchAllRequest {
  return { query: "" };
}

export const SearchAllRequest = {
  encode(message: SearchAllRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.query !== "") {
      writer.uint32(10).string(message.query);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SearchAllRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSearchAllRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.query = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SearchAllRequest {
    return { query: isSet(object.query) ? String(object.query) : "" };
  },

  toJSON(message: SearchAllRequest): unknown {
    const obj: any = {};
    if (message.query !== "") {
      obj.query = message.query;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SearchAllRequest>, I>>(base?: I): SearchAllRequest {
    return SearchAllRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SearchAllRequest>, I>>(object: I): SearchAllRequest {
    const message = createBaseSearchAllRequest();
    message.query = object.query ?? "";
    return message;
  },
};

function createBaseSearchAllResponse(): SearchAllResponse {
  return { tracks: [] };
}

export const SearchAllResponse = {
  encode(message: SearchAllResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.tracks) {
      Track.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SearchAllResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSearchAllResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.tracks.push(Track.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SearchAllResponse {
    return { tracks: Array.isArray(object?.tracks) ? object.tracks.map((e: any) => Track.fromJSON(e)) : [] };
  },

  toJSON(message: SearchAllResponse): unknown {
    const obj: any = {};
    if (message.tracks?.length) {
      obj.tracks = message.tracks.map((e) => Track.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SearchAllResponse>, I>>(base?: I): SearchAllResponse {
    return SearchAllResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SearchAllResponse>, I>>(object: I): SearchAllResponse {
    const message = createBaseSearchAllResponse();
    message.tracks = object.tracks?.map((e) => Track.fromPartial(e)) || [];
    return message;
  },
};

/** Musync service */
export interface MusyncService {
  Login(request: DeepPartial<LoginRequest>, metadata?: grpc.Metadata): Promise<Token>;
  AddLocalFolder(
    request: DeepPartial<AddLocalFolderRequest>,
    metadata?: grpc.Metadata,
  ): Promise<AddLocalFolderResponse>;
  RemoveLocalFolder(
    request: DeepPartial<RemoveLocalFolderRequest>,
    metadata?: grpc.Metadata,
  ): Promise<RemoveLocalFolderResponse>;
  QueryLocalFolders(request: DeepPartial<QueryLocalFoldersRequest>, metadata?: grpc.Metadata): Observable<LocalFolder>;
  CreatePlayQueue(
    request: DeepPartial<CreatePlayQueueRequest>,
    metadata?: grpc.Metadata,
  ): Promise<CreatePlayQueueResponse>;
  GetPlayQueue(request: DeepPartial<GetPlayQueueRequest>, metadata?: grpc.Metadata): Promise<PlayQueue>;
  CreatePlaylist(
    request: DeepPartial<CreatePlaylistRequest>,
    metadata?: grpc.Metadata,
  ): Promise<CreatePlaylistResponse>;
  QueryPlaylists(request: DeepPartial<QueryPlaylistsRequest>, metadata?: grpc.Metadata): Observable<Playlist>;
  UpdatePlaylist(
    request: DeepPartial<UpdatePlaylistRequest>,
    metadata?: grpc.Metadata,
  ): Promise<UpdatePlaylistResponse>;
  DeletePlaylists(request: DeepPartial<DeletePlaylistsRequest>, metadata?: grpc.Metadata): Observable<Playlist>;
  GetTrack(request: DeepPartial<GetTrackRequest>, metadata?: grpc.Metadata): Promise<Track>;
  GetTrackCover(request: DeepPartial<GetTrackCoverRequest>, metadata?: grpc.Metadata): Promise<Picture>;
  CreateTrack(request: DeepPartial<CreateTrackRequest>, metadata?: grpc.Metadata): Promise<CreateTrackResponse>;
  QueryTracks(request: DeepPartial<QueryTracksRequest>, metadata?: grpc.Metadata): Observable<Track>;
  UpdateTrack(request: DeepPartial<UpdateTrackRequest>, metadata?: grpc.Metadata): Promise<UpdateTrackResponse>;
  DeleteTracks(request: DeepPartial<DeleteTracksRequest>, metadata?: grpc.Metadata): Observable<Track>;
  CreateUser(request: DeepPartial<CreateUserRequest>, metadata?: grpc.Metadata): Promise<CreateUserResponse>;
  QueryUsers(request: DeepPartial<QueryUsersRequest>, metadata?: grpc.Metadata): Observable<User>;
  UpdateUser(request: DeepPartial<UpdateUserRequest>, metadata?: grpc.Metadata): Promise<UpdateUserResponse>;
  DeleteUsers(request: DeepPartial<DeleteUsersRequest>, metadata?: grpc.Metadata): Observable<User>;
  SearchAll(request: DeepPartial<SearchAllRequest>, metadata?: grpc.Metadata): Promise<SearchAllResponse>;
}

export class MusyncServiceClientImpl implements MusyncService {
  private readonly rpc: Rpc;

  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.Login = this.Login.bind(this);
    this.AddLocalFolder = this.AddLocalFolder.bind(this);
    this.RemoveLocalFolder = this.RemoveLocalFolder.bind(this);
    this.QueryLocalFolders = this.QueryLocalFolders.bind(this);
    this.CreatePlayQueue = this.CreatePlayQueue.bind(this);
    this.GetPlayQueue = this.GetPlayQueue.bind(this);
    this.CreatePlaylist = this.CreatePlaylist.bind(this);
    this.QueryPlaylists = this.QueryPlaylists.bind(this);
    this.UpdatePlaylist = this.UpdatePlaylist.bind(this);
    this.DeletePlaylists = this.DeletePlaylists.bind(this);
    this.GetTrack = this.GetTrack.bind(this);
    this.GetTrackCover = this.GetTrackCover.bind(this);
    this.CreateTrack = this.CreateTrack.bind(this);
    this.QueryTracks = this.QueryTracks.bind(this);
    this.UpdateTrack = this.UpdateTrack.bind(this);
    this.DeleteTracks = this.DeleteTracks.bind(this);
    this.CreateUser = this.CreateUser.bind(this);
    this.QueryUsers = this.QueryUsers.bind(this);
    this.UpdateUser = this.UpdateUser.bind(this);
    this.DeleteUsers = this.DeleteUsers.bind(this);
    this.SearchAll = this.SearchAll.bind(this);
  }

  Login(request: DeepPartial<LoginRequest>, metadata?: grpc.Metadata): Promise<Token> {
    return this.rpc.unary(MusyncServiceLoginDesc, LoginRequest.fromPartial(request), metadata);
  }

  AddLocalFolder(
    request: DeepPartial<AddLocalFolderRequest>,
    metadata?: grpc.Metadata,
  ): Promise<AddLocalFolderResponse> {
    return this.rpc.unary(MusyncServiceAddLocalFolderDesc, AddLocalFolderRequest.fromPartial(request), metadata);
  }

  RemoveLocalFolder(
    request: DeepPartial<RemoveLocalFolderRequest>,
    metadata?: grpc.Metadata,
  ): Promise<RemoveLocalFolderResponse> {
    return this.rpc.unary(MusyncServiceRemoveLocalFolderDesc, RemoveLocalFolderRequest.fromPartial(request), metadata);
  }

  QueryLocalFolders(request: DeepPartial<QueryLocalFoldersRequest>, metadata?: grpc.Metadata): Observable<LocalFolder> {
    return this.rpc.invoke(MusyncServiceQueryLocalFoldersDesc, QueryLocalFoldersRequest.fromPartial(request), metadata);
  }

  CreatePlayQueue(
    request: DeepPartial<CreatePlayQueueRequest>,
    metadata?: grpc.Metadata,
  ): Promise<CreatePlayQueueResponse> {
    return this.rpc.unary(MusyncServiceCreatePlayQueueDesc, CreatePlayQueueRequest.fromPartial(request), metadata);
  }

  GetPlayQueue(request: DeepPartial<GetPlayQueueRequest>, metadata?: grpc.Metadata): Promise<PlayQueue> {
    return this.rpc.unary(MusyncServiceGetPlayQueueDesc, GetPlayQueueRequest.fromPartial(request), metadata);
  }

  CreatePlaylist(
    request: DeepPartial<CreatePlaylistRequest>,
    metadata?: grpc.Metadata,
  ): Promise<CreatePlaylistResponse> {
    return this.rpc.unary(MusyncServiceCreatePlaylistDesc, CreatePlaylistRequest.fromPartial(request), metadata);
  }

  QueryPlaylists(request: DeepPartial<QueryPlaylistsRequest>, metadata?: grpc.Metadata): Observable<Playlist> {
    return this.rpc.invoke(MusyncServiceQueryPlaylistsDesc, QueryPlaylistsRequest.fromPartial(request), metadata);
  }

  UpdatePlaylist(
    request: DeepPartial<UpdatePlaylistRequest>,
    metadata?: grpc.Metadata,
  ): Promise<UpdatePlaylistResponse> {
    return this.rpc.unary(MusyncServiceUpdatePlaylistDesc, UpdatePlaylistRequest.fromPartial(request), metadata);
  }

  DeletePlaylists(request: DeepPartial<DeletePlaylistsRequest>, metadata?: grpc.Metadata): Observable<Playlist> {
    return this.rpc.invoke(MusyncServiceDeletePlaylistsDesc, DeletePlaylistsRequest.fromPartial(request), metadata);
  }

  GetTrack(request: DeepPartial<GetTrackRequest>, metadata?: grpc.Metadata): Promise<Track> {
    return this.rpc.unary(MusyncServiceGetTrackDesc, GetTrackRequest.fromPartial(request), metadata);
  }

  GetTrackCover(request: DeepPartial<GetTrackCoverRequest>, metadata?: grpc.Metadata): Promise<Picture> {
    return this.rpc.unary(MusyncServiceGetTrackCoverDesc, GetTrackCoverRequest.fromPartial(request), metadata);
  }

  CreateTrack(request: DeepPartial<CreateTrackRequest>, metadata?: grpc.Metadata): Promise<CreateTrackResponse> {
    return this.rpc.unary(MusyncServiceCreateTrackDesc, CreateTrackRequest.fromPartial(request), metadata);
  }

  QueryTracks(request: DeepPartial<QueryTracksRequest>, metadata?: grpc.Metadata): Observable<Track> {
    return this.rpc.invoke(MusyncServiceQueryTracksDesc, QueryTracksRequest.fromPartial(request), metadata);
  }

  UpdateTrack(request: DeepPartial<UpdateTrackRequest>, metadata?: grpc.Metadata): Promise<UpdateTrackResponse> {
    return this.rpc.unary(MusyncServiceUpdateTrackDesc, UpdateTrackRequest.fromPartial(request), metadata);
  }

  DeleteTracks(request: DeepPartial<DeleteTracksRequest>, metadata?: grpc.Metadata): Observable<Track> {
    return this.rpc.invoke(MusyncServiceDeleteTracksDesc, DeleteTracksRequest.fromPartial(request), metadata);
  }

  CreateUser(request: DeepPartial<CreateUserRequest>, metadata?: grpc.Metadata): Promise<CreateUserResponse> {
    return this.rpc.unary(MusyncServiceCreateUserDesc, CreateUserRequest.fromPartial(request), metadata);
  }

  QueryUsers(request: DeepPartial<QueryUsersRequest>, metadata?: grpc.Metadata): Observable<User> {
    return this.rpc.invoke(MusyncServiceQueryUsersDesc, QueryUsersRequest.fromPartial(request), metadata);
  }

  UpdateUser(request: DeepPartial<UpdateUserRequest>, metadata?: grpc.Metadata): Promise<UpdateUserResponse> {
    return this.rpc.unary(MusyncServiceUpdateUserDesc, UpdateUserRequest.fromPartial(request), metadata);
  }

  DeleteUsers(request: DeepPartial<DeleteUsersRequest>, metadata?: grpc.Metadata): Observable<User> {
    return this.rpc.invoke(MusyncServiceDeleteUsersDesc, DeleteUsersRequest.fromPartial(request), metadata);
  }

  SearchAll(request: DeepPartial<SearchAllRequest>, metadata?: grpc.Metadata): Promise<SearchAllResponse> {
    return this.rpc.unary(MusyncServiceSearchAllDesc, SearchAllRequest.fromPartial(request), metadata);
  }
}

export const MusyncServiceDesc = { serviceName: "musync.MusyncService" };

export const MusyncServiceLoginDesc: UnaryMethodDefinitionish = {
  methodName: "Login",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return LoginRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Token.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceAddLocalFolderDesc: UnaryMethodDefinitionish = {
  methodName: "AddLocalFolder",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return AddLocalFolderRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = AddLocalFolderResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceRemoveLocalFolderDesc: UnaryMethodDefinitionish = {
  methodName: "RemoveLocalFolder",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return RemoveLocalFolderRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = RemoveLocalFolderResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceQueryLocalFoldersDesc: UnaryMethodDefinitionish = {
  methodName: "QueryLocalFolders",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return QueryLocalFoldersRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = LocalFolder.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceCreatePlayQueueDesc: UnaryMethodDefinitionish = {
  methodName: "CreatePlayQueue",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return CreatePlayQueueRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = CreatePlayQueueResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceGetPlayQueueDesc: UnaryMethodDefinitionish = {
  methodName: "GetPlayQueue",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return GetPlayQueueRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = PlayQueue.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceCreatePlaylistDesc: UnaryMethodDefinitionish = {
  methodName: "CreatePlaylist",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return CreatePlaylistRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = CreatePlaylistResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceQueryPlaylistsDesc: UnaryMethodDefinitionish = {
  methodName: "QueryPlaylists",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return QueryPlaylistsRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Playlist.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceUpdatePlaylistDesc: UnaryMethodDefinitionish = {
  methodName: "UpdatePlaylist",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return UpdatePlaylistRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = UpdatePlaylistResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceDeletePlaylistsDesc: UnaryMethodDefinitionish = {
  methodName: "DeletePlaylists",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return DeletePlaylistsRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Playlist.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceGetTrackDesc: UnaryMethodDefinitionish = {
  methodName: "GetTrack",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return GetTrackRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Track.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceGetTrackCoverDesc: UnaryMethodDefinitionish = {
  methodName: "GetTrackCover",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return GetTrackCoverRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Picture.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceCreateTrackDesc: UnaryMethodDefinitionish = {
  methodName: "CreateTrack",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return CreateTrackRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = CreateTrackResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceQueryTracksDesc: UnaryMethodDefinitionish = {
  methodName: "QueryTracks",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return QueryTracksRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Track.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceUpdateTrackDesc: UnaryMethodDefinitionish = {
  methodName: "UpdateTrack",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return UpdateTrackRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = UpdateTrackResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceDeleteTracksDesc: UnaryMethodDefinitionish = {
  methodName: "DeleteTracks",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return DeleteTracksRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = Track.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceCreateUserDesc: UnaryMethodDefinitionish = {
  methodName: "CreateUser",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return CreateUserRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = CreateUserResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceQueryUsersDesc: UnaryMethodDefinitionish = {
  methodName: "QueryUsers",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return QueryUsersRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = User.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceUpdateUserDesc: UnaryMethodDefinitionish = {
  methodName: "UpdateUser",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return UpdateUserRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = UpdateUserResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceDeleteUsersDesc: UnaryMethodDefinitionish = {
  methodName: "DeleteUsers",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return DeleteUsersRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = User.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const MusyncServiceSearchAllDesc: UnaryMethodDefinitionish = {
  methodName: "SearchAll",
  service: MusyncServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return SearchAllRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SearchAllResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

interface UnaryMethodDefinitionishR extends grpc.UnaryMethodDefinition<any, any> {
  requestStream: any;
  responseStream: any;
}

type UnaryMethodDefinitionish = UnaryMethodDefinitionishR;

interface Rpc {
  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any>;
  invoke<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    request: any,
    metadata: grpc.Metadata | undefined,
  ): Observable<any>;
}

export class GrpcWebImpl {
  private host: string;
  private options: {
    transport?: grpc.TransportFactory;
    streamingTransport?: grpc.TransportFactory;
    debug?: boolean;
    metadata?: grpc.Metadata;
    upStreamRetryCodes?: number[];
  };

  constructor(
    host: string,
    options: {
      transport?: grpc.TransportFactory;
      streamingTransport?: grpc.TransportFactory;
      debug?: boolean;
      metadata?: grpc.Metadata;
      upStreamRetryCodes?: number[];
    },
  ) {
    this.host = host;
    this.options = options;
  }

  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    _request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any> {
    const request = { ..._request, ...methodDesc.requestType };
    const maybeCombinedMetadata = metadata && this.options.metadata
      ? new BrowserHeaders({ ...this.options?.metadata.headersMap, ...metadata?.headersMap })
      : metadata ?? this.options.metadata;
    return new Promise((resolve, reject) => {
      grpc.unary(methodDesc, {
        request,
        host: this.host,
        metadata: maybeCombinedMetadata ?? {},
        ...(this.options.transport !== undefined ? { transport: this.options.transport } : {}),
        debug: this.options.debug ?? false,
        onEnd: function (response) {
          if (response.status === grpc.Code.OK) {
            resolve(response.message!.toObject());
          } else {
            const err = new GrpcWebError(response.statusMessage, response.status, response.trailers);
            reject(err);
          }
        },
      });
    });
  }

  invoke<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    _request: any,
    metadata: grpc.Metadata | undefined,
  ): Observable<any> {
    const upStreamCodes = this.options.upStreamRetryCodes ?? [];
    const DEFAULT_TIMEOUT_TIME: number = 3_000;
    const request = { ..._request, ...methodDesc.requestType };
    const transport = this.options.streamingTransport ?? this.options.transport;
    const maybeCombinedMetadata = metadata && this.options.metadata
      ? new BrowserHeaders({ ...this.options?.metadata.headersMap, ...metadata?.headersMap })
      : metadata ?? this.options.metadata;
    return new Observable((observer) => {
      const upStream = (() => {
        const client = grpc.invoke(methodDesc, {
          host: this.host,
          request,
          ...(transport !== undefined ? { transport } : {}),
          metadata: maybeCombinedMetadata ?? {},
          debug: this.options.debug ?? false,
          onMessage: (next) => observer.next(next),
          onEnd: (code: grpc.Code, message: string, trailers: grpc.Metadata) => {
            if (code === 0) {
              observer.complete();
            } else if (upStreamCodes.includes(code)) {
              setTimeout(upStream, DEFAULT_TIMEOUT_TIME);
            } else {
              const err = new Error(message) as any;
              err.code = code;
              err.metadata = trailers;
              observer.error(err);
            }
          },
        });
        observer.add(() => {
          return client.close();
        });
      });
      upStream();
    }).pipe(share());
  }
}

declare const self: any | undefined;
declare const window: any | undefined;
declare const global: any | undefined;
const tsProtoGlobalThis: any = (() => {
  if (typeof globalThis !== "undefined") {
    return globalThis;
  }
  if (typeof self !== "undefined") {
    return self;
  }
  if (typeof window !== "undefined") {
    return window;
  }
  if (typeof global !== "undefined") {
    return global;
  }
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function toTimestamp(date: Date): Timestamp {
  const seconds = date.getTime() / 1_000;
  const nanos = (date.getTime() % 1_000) * 1_000_000;
  return { seconds, nanos };
}

function fromTimestamp(t: Timestamp): Date {
  let millis = (t.seconds || 0) * 1_000;
  millis += (t.nanos || 0) / 1_000_000;
  return new Date(millis);
}

function fromJsonTimestamp(o: any): Date {
  if (o instanceof Date) {
    return o;
  } else if (typeof o === "string") {
    return new Date(o);
  } else {
    return fromTimestamp(Timestamp.fromJSON(o));
  }
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

export class GrpcWebError extends tsProtoGlobalThis.Error {
  constructor(message: string, public code: grpc.Code, public metadata: grpc.Metadata) {
    super(message);
  }
}
