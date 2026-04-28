import { useState } from 'react'
import './App.css'
import { PlayerBar } from './components/PlayerBar'
import { Sidebar } from './components/Sidebar'
import { TopBar } from './components/TopBar'
import { playlists, songs, upNext } from './data/mockData'
import type { Screen } from './types/music'
import { NowPlayingView } from './views/NowPlayingView'
import { PlaylistDetailView } from './views/PlaylistDetailView'
import { PlaylistsView } from './views/PlaylistsView'
import { SongsView } from './views/SongsView'

function App() {
  const [screen, setScreen] = useState<Screen>('songs')
  const selectedSong = songs[1]

  return (
    <div className="app-shell">
      <Sidebar screen={screen} onNavigate={setScreen} />

      <div className="main-column">
        <TopBar />

        <main className="content-shell">
          {screen === 'songs' && <SongsView songs={songs} selectedSong={selectedSong} />}

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

export default App
