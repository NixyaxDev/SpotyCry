import { useEffect, useMemo, useState } from 'react'
import { usePlayback } from '../features/playback/hooks/usePlayback'
import { usePlaylists } from '../features/playlists/hooks/usePlaylists'
import { mapPlaylistDtoToUiPlaylist, mapPlaylistSongDtoToUiSong } from '../features/playlists/mappers'
import { useSongs } from '../features/songs/hooks/useSongs'
import { mapSongDtoToSongListItem, mapSongListItemToUiSong } from '../features/songs/mappers'
import type { Screen } from '../types/music'

export function useAppViewModel() {
  const [screen, setScreen] = useState<Screen>('songs')
  const [selectedPlaylistId, setSelectedPlaylistId] = useState<string | null>(null)
  const songsState = useSongs()
  const playlistsState = usePlaylists()
  const playbackState = usePlayback()

  const songListItems = useMemo(
    () => songsState.songs.map(mapSongDtoToSongListItem),
    [songsState.songs],
  )

  const uiSongs = useMemo(
    () => songListItems.map(mapSongListItemToUiSong),
    [songListItems],
  )

  const uiPlaylists = useMemo(
    () => playlistsState.playlists.map(mapPlaylistDtoToUiPlaylist),
    [playlistsState.playlists],
  )

  const selectedSong =
    uiSongs.find((song) => song.id === playbackState.currentSongId) ?? null

  const upNextSongs = playbackState.currentSongId
    ? uiSongs.filter((song) => song.id !== playbackState.currentSongId)
    : []

  const selectedPlaylist =
    uiPlaylists.find((playlist) => playlist.id === selectedPlaylistId) ?? null

  const selectedPlaylistBaseSongs = songsState.songs.filter((song) =>
    selectedPlaylist ? selectedPlaylist.songIds.includes(song.id) : false,
  )

  const selectedPlaylistViewSongs = useMemo(
    () =>
      playlistsState.detailSongs
        ? playlistsState.detailSongs.map(mapPlaylistSongDtoToUiSong)
        : selectedPlaylistBaseSongs.map((song) => mapSongListItemToUiSong(mapSongDtoToSongListItem(song))),
    [playlistsState.detailSongs, selectedPlaylistBaseSongs],
  )

  const availableSongsForPlaylist = songsState.songs.filter(
    (song) => !selectedPlaylist?.songIds.includes(song.id),
  )

  useEffect(() => {
    if (!selectedPlaylistId) {
      return
    }

    void playlistsState.loadPlaylistSummary(selectedPlaylistId)
  }, [selectedPlaylistId, selectedPlaylist?.songIds.length])

  return {
    screen,
    setScreen,
    selectedPlaylistId,
    selectedPlaylist,
    selectedSong,
    upNextSongs,
    songListItems,
    uiSongs,
    songsState,
    playlistsState,
    playbackState,
    selectedPlaylistViewSongs,
    availableSongsForPlaylist,
    handlePlaySong,
    handleAudioPause,
    handleOpenPlaylist,
    handleAddSongToPlaylist,
    handleRemoveSongFromPlaylist,
    handleFilterPlaylistSongs,
    handleSortPlaylistSongs,
  }

  async function handlePlaySong(songId: string) {
    if (
      playbackState.currentSongId === songId &&
      playbackState.isPlaying
    ) {
      await playbackState.stopPlayback()
      return
    }

    const songToPlay = uiSongs.find((song) => song.id === songId)

    if (!songToPlay) {
      return
    }

    await playbackState.startPlayback(songToPlay)
  }

  async function handleAudioPause() {
    playbackState.markAudioStopped()
    await playbackState.stopPlayback()
  }

  function handleOpenPlaylist(playlistId: string) {
    playlistsState.clearDetailSongs()
    setSelectedPlaylistId(playlistId)
    setScreen('playlist-detail')
  }

  async function handleAddSongToPlaylist(songId: string) {
    if (!selectedPlaylistId) {
      return
    }

    const wasUpdated = await playlistsState.addSongToPlaylist(selectedPlaylistId, songId)

    if (wasUpdated) {
      await playlistsState.loadPlaylistSummary(selectedPlaylistId)
    }
  }

  async function handleRemoveSongFromPlaylist(songId: string) {
    if (!selectedPlaylistId) {
      return
    }

    const wasUpdated = await playlistsState.removeSongFromPlaylist(selectedPlaylistId, songId)

    if (wasUpdated) {
      await playlistsState.loadPlaylistSummary(selectedPlaylistId)
    }
  }

  async function handleFilterPlaylistSongs(
    criteria: 'title' | 'artist' | 'genre',
    value: string,
  ) {
    if (!selectedPlaylistId) {
      return
    }

    await playlistsState.filterPlaylistSongs(selectedPlaylistId, criteria, value)
  }

  async function handleSortPlaylistSongs(
    criteria: 'title' | 'artist' | 'duration',
    direction: 'asc' | 'desc',
  ) {
    if (!selectedPlaylistId) {
      return
    }

    await playlistsState.sortPlaylistSongs(selectedPlaylistId, criteria, direction)
  }
}
