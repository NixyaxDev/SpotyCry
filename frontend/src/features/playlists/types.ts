export interface PlaylistDto {
  id: string
  name: string
  song_ids: string[]
}

export interface ListPlaylistsData {
  playlists: PlaylistDto[]
}

export interface CreatePlaylistPayload {
  name: string
}

export interface CreatePlaylistData {
  playlist: PlaylistDto
}
