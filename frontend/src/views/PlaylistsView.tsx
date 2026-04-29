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
  onPlayPlaylist: (playlistId: string) => void
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
  onPlayPlaylist,
  onReload,
}: PlaylistsViewProps) {
  return (
    <>
      <SectionHeader
        title="Tus playlists"
        subtitle="Colecciones creadas a partir de las canciones disponibles en el servidor."
        stacked
      />

      <section className="playlist-layout">
        <CreatePlaylistForm
          onSubmit={onCreatePlaylist}
          loading={createLoading}
          error={createError}
        />

        <div className="playlist-results">
          {loading && <div className="feedback-card">Cargando playlists...</div>}

          {!loading && error && (
            <div className="feedback-card feedback-card--error">
              <p>{error}</p>
              <button type="button" className="primary-button" onClick={onReload}>
                Intentar de nuevo
              </button>
            </div>
          )}

          {!loading && !error && playlists.length === 0 && (
            <div className="feedback-card">
              <p>No se han creado playlists todavía. Seguirán disponibles mientras el servidor esté en ejecución.</p>
            </div>
          )}

          {!loading && !error && playlists.length > 0 && (
            <PlaylistList
              playlists={playlists}
              onOpenPlaylist={onOpenPlaylist}
              onPlayPlaylist={onPlayPlaylist}
            />
          )}
        </div>
      </section>
    </>
  )
}
