export interface PlaylistDto {
  id: string
  name: string
  song_ids: string[]
}

export interface PlaylistResponseData {
  playlist: PlaylistDto
}

export interface ListPlaylistsData {
  playlists: PlaylistDto[]
}

export interface CreatePlaylistPayload {
  name: string
}

export interface AddSongToPlaylistPayload {
  playlist_id: string
  song_id: string
}

export interface RemoveSongFromPlaylistPayload {
  playlist_id: string
  song_id: string
}

export interface FilterPlaylistSongsPayload {
  playlist_id: string
  criteria: 'title' | 'artist' | 'genre'
  value: string
}

export interface SortPlaylistSongsPayload {
  playlist_id: string
  criteria: 'title' | 'artist' | 'duration'
  direction: 'asc' | 'desc'
}

export interface PlaylistSummaryPayload {
  playlist_id: string
}

export interface SongDto {
  id: string
  title: string
  artist: string | null
  album: string | null
  genre: string | null
  duration: number | null
}

export interface PlaylistSongsData {
  songs: SongDto[]
}

export interface PlaylistSummaryDto {
  song_count: number
  total_duration_seconds: number
  unknown_duration_count: number
}

export interface PlaylistSummaryData {
  summary: PlaylistSummaryDto
}
