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
  const [playQueue, setPlayQueue] = useState<ReturnType<typeof mapSongListItemToUiSong>[]>([])
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

  const currentQueueIndex = playQueue.findIndex(
    (song) => song.id === playbackState.currentSongId,
  )

  const queueSongs = playQueue.length > 0 ? playQueue : uiSongs

  const hasPreviousSong = currentQueueIndex > 0
  const hasNextSong = currentQueueIndex >= 0 && currentQueueIndex < queueSongs.length - 1

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

  const selectedPlaylistQueueSongs = useMemo(
    () =>
      selectedPlaylist
        ? selectedPlaylist.songIds
            .map((songId) => uiSongs.find((song) => song.id === songId) ?? null)
            .filter((song): song is (typeof uiSongs)[number] => song !== null)
        : [],
    [selectedPlaylist, uiSongs],
  )

  useEffect(() => {
    if (!selectedPlaylistId) {
      return
    }

    void playlistsState.loadPlaylistSummary(selectedPlaylistId)
  }, [selectedPlaylistId, selectedPlaylist?.songIds.length])

  useEffect(() => {
    if (!playbackState.currentSongId && playQueue.length > 0) {
      const queueStillExists = playQueue.some((song) => uiSongs.some((uiSong) => uiSong.id === song.id))

      if (!queueStillExists) {
        setPlayQueue([])
      }
    }
  }, [playbackState.currentSongId, playQueue, uiSongs])

  return {
    screen,
    setScreen,
    selectedPlaylistId,
    selectedPlaylist,
    selectedSong,
    queueSongs,
    currentQueueIndex,
    hasPreviousSong,
    hasNextSong,
    songListItems,
    uiSongs,
    songsState,
    playlistsState,
    playbackState,
    selectedPlaylistViewSongs,
    selectedPlaylistQueueSongs,
    availableSongsForPlaylist,
    handlePlaySong,
    handleAudioPause,
    handlePauseBufferedSong,
    handleResumeBufferedSong,
    handleAudioEnded,
    handlePlayPreviousSong,
    handlePlayNextSong,
    handlePlayPlaylistFromList,
    handlePlaySelectedPlaylist,
    handleOpenPlaylist,
    handleAddSongToPlaylist,
    handleRemoveSongFromPlaylist,
    handleFilterPlaylistSongs,
    handleSortPlaylistSongs,
  }

  async function handlePlaySong(songId: string) {
    const sourceQueue = queueSongs.some((song) => song.id === songId) ? queueSongs : uiSongs
    await playSongFromQueue(sourceQueue, songId)
  }

  async function playSongFromQueue(queue: typeof uiSongs, songId: string) {
    if (
      playbackState.currentSongId === songId &&
      playbackState.isPlaying
    ) {
      playbackState.pauseBufferedAudio()
      return
    }

    if (
      playbackState.currentSongId === songId &&
      playbackState.audioUrl &&
      !playbackState.isPlaying
    ) {
      playbackState.resumeBufferedAudio()
      return
    }

    const songToPlay = queue.find((song) => song.id === songId)

    if (!songToPlay) {
      return
    }

    setPlayQueue(queue)
    await playbackState.startPlayback(songToPlay)
    setScreen('now-playing')
  }

  function handleAudioPause() {
    playbackState.markAudioStopped()
  }

  function handlePauseBufferedSong() {
    playbackState.pauseBufferedAudio()
  }

  function handleResumeBufferedSong() {
    playbackState.resumeBufferedAudio()
  }

  async function handleAudioEnded() {
    if (hasNextSong) {
      const nextSong = queueSongs[currentQueueIndex + 1]

      if (nextSong) {
        await playSongFromQueue(queueSongs, nextSong.id)
        return
      }
    }

    playbackState.clearPlaybackState()
  }

  async function handlePlayPreviousSong() {
    if (!hasPreviousSong) {
      return
    }

    const previousSong = queueSongs[currentQueueIndex - 1]

    if (!previousSong) {
      return
    }

    await playSongFromQueue(queueSongs, previousSong.id)
  }

  async function handlePlayNextSong() {
    if (!hasNextSong) {
      return
    }

    const nextSong = queueSongs[currentQueueIndex + 1]

    if (!nextSong) {
      return
    }

    await playSongFromQueue(queueSongs, nextSong.id)
  }

  async function handlePlayPlaylistFromList(playlistId: string) {
    const playlist = uiPlaylists.find((item) => item.id === playlistId)

    if (!playlist) {
      return
    }

    const playlistSongs = playlist.songIds
      .map((songId) => uiSongs.find((song) => song.id === songId) ?? null)
      .filter((song): song is (typeof uiSongs)[number] => song !== null)

    const firstSong = playlistSongs[0]

    if (!firstSong) {
      return
    }

    await playSongFromQueue(playlistSongs, firstSong.id)
  }

  async function handlePlaySelectedPlaylist() {
    const queue = selectedPlaylistViewSongs.length > 0
      ? selectedPlaylistViewSongs
      : selectedPlaylistQueueSongs

    const firstSong = queue[0]

    if (!firstSong) {
      return
    }

    await playSongFromQueue(queue, firstSong.id)
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
