import { useState } from 'react'
import './App.css'
import { PlayerBar } from './components/PlayerBar'
import { Sidebar } from './components/Sidebar'
import { TopBar } from './components/TopBar'
import { playlists } from './data/mockData'
import { useSongs } from './features/songs/hooks/useSongs'
import type { SongListItem } from './features/songs/types'
import type { Screen, Song } from './types/music'
import { NowPlayingView } from './views/NowPlayingView'
import { PlaylistDetailView } from './views/PlaylistDetailView'
import { PlaylistsView } from './views/PlaylistsView'
import { SongsView } from './views/SongsView'

const songCoverFallback =
  'https://images.unsplash.com/photo-1511379938547-c1f69419868d?auto=format&fit=crop&w=800&q=80'

function App() {
  const [screen, setScreen] = useState<Screen>('songs')
  const {
    songs: serverSongs,
    loading: songsLoading,
    error: songsError,
    reload: reloadSongs,
    searchValue,
    setSearchValue,
  } = useSongs()

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

  const selectedSong = uiSongs[0] ?? null
  const upNextSongs = uiSongs.slice(1)

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
            />
          )}

          {screen === 'playlists' && (
            <PlaylistsView
              playlists={playlists}
              onOpenPlaylist={() => setScreen('playlist-detail')}
            />
          )}

          {screen === 'playlist-detail' && (
            <PlaylistDetailView songs={uiSongs} selectedSong={selectedSong} />
          )}

          {screen === 'now-playing' && (
            <NowPlayingView selectedSong={selectedSong} upNext={upNextSongs} />
          )}
        </main>

        <PlayerBar song={selectedSong} />
      </div>
    </div>
  )
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
