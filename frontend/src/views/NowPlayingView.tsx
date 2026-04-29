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
          <p>Only the current song is buffered locally in the browser.</p>
        </div>
        <div className="now-playing-copy">
          <h2>{selectedSong?.title ?? 'No song selected'}</h2>
          <p>{selectedSong?.artist ?? 'Load songs from the server to begin playback'}</p>
          <div className="now-playing-meta">
            <span>{selectedSong?.album ?? 'No album available'}</span>
            <span>•</span>
            <span>{selectedSong?.genre ?? 'No genre available'}</span>
            <span>•</span>
            <span>{selectedSong?.duration ?? 'Unknown duration'}</span>
          </div>
          <div className="now-playing-status-row">
            <span className="status-pill">
              {isPlaybackLoading
                ? 'Buffering current song'
                : isPlaying
                  ? 'Playing from local buffer'
                  : hasBufferedSong
                    ? 'Buffered locally and ready'
                    : 'No active local buffer'}
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
          Use the audio controls in the bottom player to seek forward or backward as many
          times as you want while this current song remains buffered locally.
        </p>
        {playbackError && (
          <div className="feedback-card feedback-card--error now-playing-feedback">
            <p>{playbackError}</p>
          </div>
        )}
      </div>

      <aside className="up-next-panel">
        <div className="panel-title-row">
          <h3>Queue</h3>
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
                      ? 'Previous'
                      : index === currentQueueIndex
                        ? 'Now Playing'
                        : 'Next'}
                  </small>
                  <strong>{song.title}</strong>
                  <span>{song.artist}</span>
                </div>
                <time>{song.duration}</time>
              </button>
            ))
          ) : (
            <div className="feedback-card">
              <p>No songs queued</p>
            </div>
          )}
        </div>
      </aside>
    </section>
  )
}
