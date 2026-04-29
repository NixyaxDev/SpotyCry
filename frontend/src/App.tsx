import './App.css'
import { PlayerBar } from './components/PlayerBar'
import { Sidebar } from './components/Sidebar'
import { TopBar } from './components/TopBar'
import { useAppViewModel } from './app/useAppViewModel'
import { NowPlayingView } from './views/NowPlayingView'
import { PlaylistDetailView } from './views/PlaylistDetailView'
import { PlaylistsView } from './views/PlaylistsView'
import { SongsView } from './views/SongsView'

function App() {
  const viewModel = useAppViewModel()

  return (
    <div className="app-shell">
      <Sidebar screen={viewModel.screen} onNavigate={viewModel.setScreen} />

      <div className="main-column">
        <TopBar />

        <main className="content-shell">
          {viewModel.screen === 'songs' && (
            <SongsView
              songs={viewModel.songListItems}
              loading={viewModel.songsState.loading}
              error={viewModel.songsState.error}
              onReload={viewModel.songsState.reload}
              searchCriteria={viewModel.songsState.searchCriteria}
              onSearchCriteriaChange={viewModel.songsState.setSearchCriteria}
              searchValue={viewModel.songsState.searchValue}
              onSearchChange={viewModel.songsState.setSearchValue}
              onPlay={viewModel.handlePlaySong}
              isPlaybackLoading={viewModel.playbackState.loading}
              activeSongId={viewModel.playbackState.currentSongId}
              isPlaying={viewModel.playbackState.isPlaying}
            />
          )}

          {viewModel.screen === 'playlists' && (
            <PlaylistsView
              playlists={viewModel.playlistsState.playlists}
              loading={viewModel.playlistsState.loading}
              error={viewModel.playlistsState.error}
              createError={viewModel.playlistsState.createError}
              createLoading={viewModel.playlistsState.createLoading}
              onCreatePlaylist={viewModel.playlistsState.createPlaylist}
              onOpenPlaylist={viewModel.handleOpenPlaylist}
              onPlayPlaylist={viewModel.handlePlayPlaylistFromList}
              onReload={viewModel.playlistsState.reload}
            />
          )}

          {viewModel.screen === 'playlist-detail' && (
            <PlaylistDetailView
              playlist={viewModel.selectedPlaylist}
              songs={viewModel.selectedPlaylistViewSongs}
              availableSongs={viewModel.availableSongsForPlaylist}
              summary={viewModel.playlistsState.summary}
              error={viewModel.playlistsState.detailError}
              actionLoading={viewModel.playlistsState.actionLoading}
              onAddSong={viewModel.handleAddSongToPlaylist}
              onRemoveSong={viewModel.handleRemoveSongFromPlaylist}
              onFilterSongs={viewModel.handleFilterPlaylistSongs}
              onSortSongs={viewModel.handleSortPlaylistSongs}
              onPlayPlaylist={viewModel.handlePlaySelectedPlaylist}
              onResetSongView={viewModel.playlistsState.clearDetailSongs}
              selectedSong={viewModel.selectedSong}
            />
          )}

          {viewModel.screen === 'now-playing' && (
            <NowPlayingView
              selectedSong={viewModel.selectedSong}
              queueSongs={viewModel.queueSongs}
              currentQueueIndex={viewModel.currentQueueIndex}
              isPlaying={viewModel.playbackState.isPlaying}
              isPlaybackLoading={viewModel.playbackState.loading}
              playbackError={viewModel.playbackState.error}
              hasBufferedSong={viewModel.playbackState.audioUrl !== null}
              hasPreviousSong={viewModel.hasPreviousSong}
              hasNextSong={viewModel.hasNextSong}
              onPausePlayback={viewModel.handlePauseBufferedSong}
              onResumePlayback={viewModel.handleResumeBufferedSong}
              onStopPlayback={viewModel.playbackState.stopPlayback}
              onPlayPreviousSong={viewModel.handlePlayPreviousSong}
              onPlayNextSong={viewModel.handlePlayNextSong}
              onPlaySong={viewModel.handlePlaySong}
            />
          )}
        </main>

        <PlayerBar
          song={viewModel.selectedSong}
          audioUrl={viewModel.playbackState.audioUrl}
          audioCommand={viewModel.playbackState.audioCommand}
          hasBufferedSong={viewModel.playbackState.audioUrl !== null}
          hasPreviousSong={viewModel.hasPreviousSong}
          hasNextSong={viewModel.hasNextSong}
          onAudioPlay={viewModel.playbackState.markAudioPlaying}
          onAudioPause={viewModel.handleAudioPause}
          onStopPlayback={viewModel.playbackState.stopPlayback}
          onAudioEnded={viewModel.handleAudioEnded}
          onPlayPreviousSong={viewModel.handlePlayPreviousSong}
          onPlayNextSong={viewModel.handlePlayNextSong}
        />
      </div>
    </div>
  )
}

export default App
