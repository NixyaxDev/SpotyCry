import type { SongDto, SongListItem } from './types'
import type { Song } from '../../types/music'
import { songCoverFallback } from '../../shared/constants/media'
import { formatDuration } from '../../shared/formatters/duration'

export function mapSongDtoToSongListItem(song: SongDto): SongListItem {
  return {
    id: song.id,
    title: song.title,
    artist: song.artist ?? 'Unknown artist',
    album: song.album ?? 'Unknown album',
    genre: song.genre ?? 'Unknown genre',
    duration: formatDuration(song.duration),
    cover: songCoverFallback,
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
    cover: song.cover,
  }
}
