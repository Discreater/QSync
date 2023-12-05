export interface NcmSearchResult {
  code: number
  result: {
    searchQcReminder: null
    songCount: number
    songs: NcmSong[]
  }
}

export interface NcmSong {
  /** ? */
  a: unknown
  /** song title */
  name: string
  /** ncm id */
  id: number
  /** album */
  al: NcmAlbum
  /** alias name ? */
  alia: string[]
  /** artist */
  ar: NcmArtist[]
  cd: string
  cf: string
  copyright: number
  cp: number
  crbt: unknown
  djld: number
  /** duration time in millisecond (Maybe?) */
  dt: number
  entertainmentTags: unknown
  fee: number
  ftype: number
  hr: unknown
  /** high quality ? */
  h: NcmQuality
  /** low quality ? */
  l: NcmQuality
  /** middle quality ? */
  m: NcmQuality
  /** super quality ? */
  sq: NcmQuality
  mark: number
  mst: number
  mv: number
  no: number
  noCopyrightRcmd: unknown
  originCoverType: number
  originSongSimpleData: unknown
  pop: number
  privilege: object
  pst: number
  publishTime: number
  resourceState: boolean
  rt: unknown
  rtUrl: unknown
  rtUrls: unknown[]
  rtype: number
  rurl: unknown
  s_id: number
  single: number
  songJumpInfo: unknown
  st: number
  t: number
  tagPicList: unknown
  v: number
  version: number
}

export interface NcmAlbum {
  id: number
  name: string
  picUrl: string
}

export interface NcmArtist {
  id: number
  name: string
}

export interface NcmQuality {
  br: number
  fid: number
  size: number
  /** sample rate */
  sr: number
  vd: number
}
