import { useEffect, useState } from 'react'
import './App.css'
import { PlayerBar } from './components/PlayerBar'
import { Sidebar } from './components/Sidebar'
import { TopBar } from './components/TopBar'
import { usePlayback } from './features/playback/hooks/usePlayback'
import { usePlaylists } from './features/playlists/hooks/usePlaylists'
import { useSongs } from './features/songs/hooks/useSongs'
import type { SongListItem } from './features/songs/types'
import type { Playlist, Screen, Song } from './types/music'
import { NowPlayingView } from './views/NowPlayingView'
import { PlaylistDetailView } from './views/PlaylistDetailView'
import { PlaylistsView } from './views/PlaylistsView'
import { SongsView } from './views/SongsView'

const songCoverFallback =
  'https://images.unsplash.com/photo-1511379938547-c1f69419868d?auto=format&fit=crop&w=800&q=80'

function App() {
  const [screen, setScreen] = useState<Screen>('songs')
  const [selectedPlaylistId, setSelectedPlaylistId] = useState<string | null>(null)
  const {
    songs: serverSongs,
    loading: songsLoading,
    error: songsError,
    reload: reloadSongs,
    searchValue,
    setSearchValue,
  } = useSongs()
  const {
    playlists,
    loading: playlistsLoading,
    error: playlistsError,
    createError: playlistCreateError,
    createLoading: playlistCreateLoading,
    detailSongs,
    summary: playlistSummary,
    detailError: playlistDetailError,
    actionLoading: playlistActionLoading,
    createPlaylist,
    addSongToPlaylist,
    removeSongFromPlaylist,
    filterPlaylistSongs,
    sortPlaylistSongs,
    loadPlaylistSummary,
    clearDetailSongs,
    reload: reloadPlaylists,
  } = usePlaylists()
  const {
    audioUrl,
    loading: playbackLoading,
    error: playbackError,
    currentSongId,
    isPlaying,
    startPlayback,
    stopPlayback,
    markAudioPlaying,
    markAudioStopped,
  } = usePlayback()

  const songListItems: SongListItem[] = serverSongs.map((song) => ({
    id: song.id,
    title: song.title,
    artist: song.artist ?? 'Unknown artist',
    album: 'Catalog track',
    genre: song.genre ?? 'Unknown genre',
    duration: formatDuration(song.duration),
    cover: songCoverFallback,
  }))

  const uiSongs: Song[] = songListItems.map((song) => ({
    id: song.id,
    title: song.title,
    artist: song.artist,
    album: song.album,
    genre: song.genre,
    duration: song.duration,
    cover: song.cover,
  }))

  const selectedSong = uiSongs.find((song) => song.id === currentSongId) ?? null
  const upNextSongs = currentSongId
    ? uiSongs.filter((song) => song.id !== currentSongId)
    : []
  const uiPlaylists: Playlist[] = playlists.map((playlist) => ({
    id: playlist.id,
    name: playlist.name,
    songIds: playlist.song_ids,
  }))
  const selectedPlaylist =
    uiPlaylists.find((playlist) => playlist.id === selectedPlaylistId) ?? null
  const selectedPlaylistBaseSongs = serverSongs.filter((song) =>
    selectedPlaylist ? selectedPlaylist.songIds.includes(song.id) : false,
  )
  const selectedPlaylistViewSongs = (detailSongs ?? selectedPlaylistBaseSongs).map((song) => ({
    id: song.id,
    title: song.title,
    artist: song.artist ?? 'Unknown artist',
    album: 'Catalog track',
    genre: song.genre ?? 'Unknown genre',
    duration: formatDuration(song.duration),
    cover: songCoverFallback,
  }))
  const availableSongsForPlaylist = serverSongs.filter(
    (song) => !selectedPlaylist?.songIds.includes(song.id),
  )

  useEffect(() => {
    if (!selectedPlaylistId) {
      return
    }

    void loadPlaylistSummary(selectedPlaylistId)
  }, [selectedPlaylistId, selectedPlaylist?.songIds.length])

  return (
    <div className="app-shell">
      <Sidebar screen={screen} onNavigate={setScreen} />

      <div className="main-column">
        <TopBar />

        <main className="content-shell">
          {screen === 'songs' && (
            <SongsView
              songs={songListItems}
              loading={songsLoading}
              error={songsError}
              onReload={reloadSongs}
              searchValue={searchValue}
              onSearchChange={setSearchValue}
              onPlay={handlePlaySong}
              isPlaybackLoading={playbackLoading}
              activeSongId={currentSongId}
              isPlaying={isPlaying}
            />
          )}

          {screen === 'playlists' && (
            <PlaylistsView
              playlists={playlists}
              loading={playlistsLoading}
              error={playlistsError}
              createError={playlistCreateError}
              createLoading={playlistCreateLoading}
              onCreatePlaylist={createPlaylist}
              onOpenPlaylist={handleOpenPlaylist}
              onReload={reloadPlaylists}
            />
          )}

          {screen === 'playlist-detail' && (
            <PlaylistDetailView
              playlist={selectedPlaylist}
              songs={selectedPlaylistViewSongs}
              availableSongs={availableSongsForPlaylist}
              summary={playlistSummary}
              error={playlistDetailError}
              actionLoading={playlistActionLoading}
              onAddSong={handleAddSongToPlaylist}
              onRemoveSong={handleRemoveSongFromPlaylist}
              onFilterSongs={handleFilterPlaylistSongs}
              onSortSongs={handleSortPlaylistSongs}
              onResetSongView={clearDetailSongs}
              selectedSong={selectedSong}
            />
          )}

          {screen === 'now-playing' && (
            <NowPlayingView selectedSong={selectedSong} upNext={upNextSongs} />
          )}
        </main>

        <PlayerBar
          song={selectedSong}
          audioUrl={audioUrl}
          playbackLoading={playbackLoading}
          playbackError={playbackError}
          isPlaying={isPlaying}
          onAudioPlay={markAudioPlaying}
          onAudioPause={handleAudioPause}
          onStopPlayback={stopPlayback}
        />
      </div>
    </div>
  )

  async function handlePlaySong(songId: string) {
    if (currentSongId === songId && isPlaying) {
      await stopPlayback()
      return
    }

    const songToPlay = uiSongs.find((song) => song.id === songId)

    if (!songToPlay) {
      return
    }

    await startPlayback(songToPlay)
  }

  async function handleAudioPause() {
    markAudioStopped()
    await stopPlayback()
  }

  function handleOpenPlaylist(playlistId: string) {
    clearDetailSongs()
    setSelectedPlaylistId(playlistId)
    setScreen('playlist-detail')
  }

  async function handleAddSongToPlaylist(songId: string) {
    if (!selectedPlaylistId) {
      return
    }

    const wasUpdated = await addSongToPlaylist(selectedPlaylistId, songId)

    if (wasUpdated) {
      await loadPlaylistSummary(selectedPlaylistId)
    }
  }

  async function handleRemoveSongFromPlaylist(songId: string) {
    if (!selectedPlaylistId) {
      return
    }

    const wasUpdated = await removeSongFromPlaylist(selectedPlaylistId, songId)

    if (wasUpdated) {
      await loadPlaylistSummary(selectedPlaylistId)
    }
  }

  async function handleFilterPlaylistSongs(
    criteria: 'title' | 'artist' | 'genre',
    value: string,
  ) {
    if (!selectedPlaylistId) {
      return
    }

    await filterPlaylistSongs(selectedPlaylistId, criteria, value)
  }

  async function handleSortPlaylistSongs(
    criteria: 'title' | 'artist' | 'duration',
    direction: 'asc' | 'desc',
  ) {
    if (!selectedPlaylistId) {
      return
    }

    await sortPlaylistSongs(selectedPlaylistId, criteria, direction)
  }
}

function formatDuration(duration: number | null): string {
  if (duration === null) {
    return 'Unknown'
  }

  const minutes = Math.floor(duration / 60)
  const seconds = duration % 60

  return `${minutes}:${seconds.toString().padStart(2, '0')}`
}

export default App
