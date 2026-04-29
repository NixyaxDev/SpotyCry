import { useState } from 'react'
import './App.css'
import { PlayerBar } from './components/PlayerBar'
import { Sidebar } from './components/Sidebar'
import { TopBar } from './components/TopBar'
import { playlists, songs, upNext } from './data/mockData'
import { useSongs } from './features/songs/hooks/useSongs'
import type { SongListItem } from './features/songs/types'
import type { Screen } from './types/music'
import { NowPlayingView } from './views/NowPlayingView'
import { PlaylistDetailView } from './views/PlaylistDetailView'
import { PlaylistsView } from './views/PlaylistsView'
import { SongsView } from './views/SongsView'

const songCoverFallback = songs[0]?.cover ?? ''

function App() {
  const [screen, setScreen] = useState<Screen>('songs')
  const selectedSong = songs[1]
  const {
    songs: serverSongs,
    loading: songsLoading,
    error: songsError,
    reload: reloadSongs,
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
            />
          )}

          {screen === 'playlists' && (
            <PlaylistsView
              playlists={playlists}
              onOpenPlaylist={() => setScreen('playlist-detail')}
            />
          )}

          {screen === 'playlist-detail' && (
            <PlaylistDetailView songs={songs} selectedSong={selectedSong} />
          )}

          {screen === 'now-playing' && (
            <NowPlayingView selectedSong={selectedSong} upNext={upNext} />
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
