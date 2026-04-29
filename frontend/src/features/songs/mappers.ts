import type { SongDto, SongListItem } from './types'
import type { Song } from '../../types/music'
import { formatDuration } from '../../shared/formatters/duration'

export function mapSongDtoToSongListItem(song: SongDto): SongListItem {
  return {
    id: song.id,
    title: song.title,
    artist: song.artist ?? 'Artista desconocido',
    album: song.album ?? 'Álbum desconocido',
    genre: song.genre ?? 'Género desconocido',
    duration: formatDuration(song.duration),
  }
}

export function mapSongListItemToUiSong(song: SongListItem): Song {
  return {
    id: song.id,
    title: song.title,
    artist: song.artist,
    album: song.album,
    genre: song.genre,
    duration: song.duration,
  }
}
