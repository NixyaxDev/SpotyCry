import type { Song } from '../types/music'
import { AudioPlayer } from '../features/playback/components/AudioPlayer'

type PlayerBarProps = {
  song: Song | null
  audioUrl: string | null
  playbackLoading: boolean
  playbackError: string | null
  isPlaying: boolean
  onAudioPlay: () => void
  onAudioPause: () => void
  onStopPlayback: () => void
}

export function PlayerBar({
  song,
  audioUrl,
  playbackLoading,
  playbackError,
  isPlaying,
  onAudioPlay,
  onAudioPause,
  onStopPlayback,
}: PlayerBarProps) {
  return (
    <footer className="player-bar">
      <div className="player-now">
        {song ? (
          <>
            <img src={song.cover} alt={song.title} />
            <div>
              <strong>{song.title}</strong>
              <span>{song.artist}</span>
            </div>
          </>
        ) : (
          <div className="player-empty-copy">
            <strong>No song selected</strong>
            <span>Load songs from the server to start playing</span>
          </div>
        )}
      </div>

      <div className="player-center">
        <AudioPlayer
          audioUrl={audioUrl}
          onPlay={onAudioPlay}
          onPause={onAudioPause}
          onEnded={onStopPlayback}
        />
        {playbackLoading && <span className="player-status">Buffering audio...</span>}
        {playbackError && <span className="player-status player-status--error">{playbackError}</span>}
      </div>

      <div className="player-side">
        {isPlaying && (
          <button type="button" className="stop-button" onClick={onStopPlayback}>
            <span className="material-symbols-outlined">stop_circle</span>
          </button>
        )}
        <button type="button">
          <span className="material-symbols-outlined">favorite</span>
        </button>
        <button type="button">
          <span className="material-symbols-outlined">volume_up</span>
        </button>
      </div>
    </footer>
  )
}
