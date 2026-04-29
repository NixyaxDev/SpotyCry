import type { PlaylistDto, SongDto as PlaylistSongDto } from './types'
import type { Playlist, Song } from '../../types/music'
import { mapSongDtoToSongListItem, mapSongListItemToUiSong } from '../songs/mappers'

export function mapPlaylistDtoToUiPlaylist(playlist: PlaylistDto): Playlist {
  return {
    id: playlist.id,
    name: playlist.name,
    songIds: playlist.song_ids,
  }
}

export function mapPlaylistSongDtoToUiSong(song: PlaylistSongDto): Song {
  return mapSongListItemToUiSong(mapSongDtoToSongListItem(song))
}
