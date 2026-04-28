export type Screen = 'songs' | 'playlists' | 'playlist-detail' | 'now-playing'

export type Song = {
  id: string
  title: string
  artist: string
  album: string
  genre: string
  duration: string
  cover: string
}

export type Playlist = {
  id: string
  name: string
  tracks: number
  cover: string
}

export type RecentSong = {
  title: string
  artist: string
  cover: string
}
