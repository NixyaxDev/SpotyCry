import { SongSearch } from '../features/songs/components/SongSearch'
import { SongList } from '../features/songs/components/SongList'
import type { SongListItem } from '../features/songs/types'

type SongsViewProps = {
  songs: SongListItem[]
  loading: boolean
  error: string | null
  onReload: () => void
  searchCriteria: 'title' | 'artist' | 'album' | 'genre'
  onSearchCriteriaChange: (criteria: 'title' | 'artist' | 'album' | 'genre') => void
  searchValue: string
  onSearchChange: (value: string) => void
  onPlay: (songId: string) => void
  isPlaybackLoading: boolean
  activeSongId: string | null
  isPlaying: boolean
}

export function SongsView({
  songs,
  loading,
  error,
  onReload,
  searchCriteria,
  onSearchCriteriaChange,
  searchValue,
  onSearchChange,
  onPlay,
  isPlaybackLoading,
  activeSongId,
  isPlaying,
}: SongsViewProps) {
  return (
    <>
      <section className="panel">
        <div className="panel-title-row">
          <h3>Todas las canciones</h3>
        </div>
        <SongSearch
          criteria={searchCriteria}
          initialValue={searchValue}
          onCriteriaChange={onSearchCriteriaChange}
          onSearchChange={onSearchChange}
          loading={loading}
        />

        {loading && <div className="feedback-card">Cargando canciones...</div>}

        {!loading && error && (
          <div className="feedback-card feedback-card--error">
            <p>{error}</p>
            <button type="button" className="primary-button" onClick={onReload}>
              Intentar de nuevo
            </button>
          </div>
        )}

        {!loading && !error && songs.length === 0 && (
          <div className="feedback-card">
            <p>{searchValue.trim().length > 0 ? 'No se encontraron canciones' : 'No hay canciones disponibles'}</p>
            <button type="button" className="primary-button" onClick={onReload}>
              Actualizar
            </button>
          </div>
        )}

        {!loading && !error && songs.length > 0 && (
          <SongList
            songs={songs}
            onPlay={onPlay}
            isPlaybackLoading={isPlaybackLoading}
            activeSongId={activeSongId}
            isPlaying={isPlaying}
          />
        )}
      </section>
    </>
  )
}
