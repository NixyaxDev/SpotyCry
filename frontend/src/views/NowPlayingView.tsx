import type { Song } from '../types/music'
import {
  BackwardIcon,
  ForwardIcon,
  MusicalNoteIcon,
  PauseCircleIcon,
  PlayCircleIcon,
  QueueListIcon,
  StopCircleIcon,
} from '../shared/icons'

type NowPlayingViewProps = {
  selectedSong: Song | null
  queueSongs: Song[]
  currentQueueIndex: number
  isPlaying: boolean
  isPlaybackLoading: boolean
  playbackError: string | null
  hasBufferedSong: boolean
  hasPreviousSong: boolean
  hasNextSong: boolean
  onPausePlayback: () => void
  onResumePlayback: () => void
  onStopPlayback: () => void
  onPlayPreviousSong: () => void
  onPlayNextSong: () => void
  onPlaySong: (songId: string) => void
}

export function NowPlayingView({
  selectedSong,
  queueSongs,
  currentQueueIndex,
  isPlaying,
  isPlaybackLoading,
  playbackError,
  hasBufferedSong,
  hasPreviousSong,
  hasNextSong,
  onPausePlayback,
  onResumePlayback,
  onStopPlayback,
  onPlayPreviousSong,
  onPlayNextSong,
  onPlaySong,
}: NowPlayingViewProps) {
  return (
    <section className="now-playing-layout">
      <div className="now-playing-main">
        <div className="now-playing-art">
          <MusicalNoteIcon />
          <p>Solo la canción actual se mantiene en un buffer local del navegador.</p>
        </div>
        <div className="now-playing-copy">
          <h2>{selectedSong?.title ?? 'No hay ninguna canción seleccionada'}</h2>
          <p>{selectedSong?.artist ?? 'Carga canciones desde el servidor para iniciar la reproducción'}</p>
          <div className="now-playing-meta">
            <span>{selectedSong?.album ?? 'Sin álbum disponible'}</span>
            <span>•</span>
            <span>{selectedSong?.genre ?? 'Sin género disponible'}</span>
            <span>•</span>
            <span>{selectedSong?.duration ?? 'Duración desconocida'}</span>
          </div>
          <div className="now-playing-status-row">
            <span className="status-pill">
              {isPlaybackLoading
                ? 'Preparando canción actual'
                : isPlaying
                  ? 'Reproduciendo desde el buffer local'
                  : hasBufferedSong
                    ? 'Buffer local listo'
                    : 'Sin buffer local activo'}
            </span>
          </div>
        </div>
        <div className="now-playing-controls">
          <button
            type="button"
            className="ghost-icon"
            disabled={!hasPreviousSong}
            onClick={onPlayPreviousSong}
          >
            <BackwardIcon />
          </button>
          <button
            type="button"
            className="play-button-xl"
            disabled={!selectedSong || isPlaybackLoading}
            onClick={isPlaying ? onPausePlayback : onResumePlayback}
          >
            {isPlaying ? <PauseCircleIcon /> : <PlayCircleIcon />}
          </button>
          <button
            type="button"
            className="ghost-icon"
            disabled={!hasBufferedSong}
            onClick={onStopPlayback}
          >
            <StopCircleIcon />
          </button>
          <button
            type="button"
            className="ghost-icon"
            disabled={!hasNextSong}
            onClick={onPlayNextSong}
          >
            <ForwardIcon />
          </button>
        </div>
        <p className="now-playing-helper">
          Usa los controles del reproductor inferior para adelantar o retroceder
          cuantas veces quieras mientras esta canción siga en el buffer local.
        </p>
        {playbackError && (
          <div className="feedback-card feedback-card--error now-playing-feedback">
            <p>{playbackError}</p>
          </div>
        )}
      </div>

      <aside className="up-next-panel">
        <div className="panel-title-row">
          <h3>Cola</h3>
          <QueueListIcon />
        </div>
        <div className="up-next-list">
          {queueSongs.length > 0 ? (
            queueSongs.map((song, index) => (
              <button
                key={song.id}
                type="button"
                className={index === currentQueueIndex ? 'up-next-item is-current' : 'up-next-item'}
                onClick={() => onPlaySong(song.id)}
              >
                <div>
                  <small className="queue-marker">
                    {index < currentQueueIndex
                      ? 'Anterior'
                      : index === currentQueueIndex
                        ? 'Sonando ahora'
                        : 'Siguiente'}
                  </small>
                  <strong>{song.title}</strong>
                  <span>{song.artist}</span>
                </div>
                <time>{song.duration}</time>
              </button>
            ))
          ) : (
            <div className="feedback-card">
              <p>No hay canciones en cola</p>
            </div>
          )}
        </div>
      </aside>
    </section>
  )
}
