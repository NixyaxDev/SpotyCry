export interface SongDto {
  id: string
  title: string
  artist: string | null
  genre: string | null
  duration: number | null
}

export interface ListSongsData {
  songs: SongDto[]
}

export interface SearchSongsPayload {
  criteria: 'title'
  value: string
}

export interface SongListItem {
  id: string
  title: string
  artist: string
  album: string
  genre: string
  duration: string
  cover: string
}
