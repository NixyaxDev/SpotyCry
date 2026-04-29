import { SectionHeader } from '../components/SectionHeader'
import { CreatePlaylistForm } from '../features/playlists/components/CreatePlaylistForm'
import { PlaylistList } from '../features/playlists/components/PlaylistList'
import type { PlaylistDto } from '../features/playlists/types'

type PlaylistsViewProps = {
  playlists: PlaylistDto[]
  loading: boolean
  error: string | null
  createError: string | null
  createLoading: boolean
  onCreatePlaylist: (name: string) => Promise<boolean>
  onOpenPlaylist: (playlistId: string) => void
  onReload: () => void
}

export function PlaylistsView({
  playlists,
  loading,
  error,
  createError,
  createLoading,
  onCreatePlaylist,
  onOpenPlaylist,
  onReload,
}: PlaylistsViewProps) {
  return (
    <>
      <SectionHeader
        title="Your Playlists"
        subtitle="Collections created from the songs available on the server."
        stacked
      />

      <section className="playlist-layout">
        <CreatePlaylistForm
          onSubmit={onCreatePlaylist}
          loading={createLoading}
          error={createError}
        />

        <div className="playlist-results">
          {loading && <div className="feedback-card">Loading playlists...</div>}

          {!loading && error && (
            <div className="feedback-card feedback-card--error">
              <p>{error}</p>
              <button type="button" className="primary-button" onClick={onReload}>
                Try again
              </button>
            </div>
          )}

          {!loading && !error && playlists.length === 0 && (
            <div className="feedback-card">
              <p>No playlists created yet. They remain available while the server is running.</p>
            </div>
          )}

          {!loading && !error && playlists.length > 0 && (
            <PlaylistList playlists={playlists} onOpenPlaylist={onOpenPlaylist} />
          )}
        </div>
      </section>
    </>
  )
}
