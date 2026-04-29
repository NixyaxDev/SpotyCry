import type { Song } from '../types/music'
import { AudioPlayer } from '../features/playback/components/AudioPlayer'
import type { AudioPlayerCommand } from '../features/playback/components/AudioPlayer'
import { BackwardIcon, ForwardIcon, StopCircleIcon } from '../shared/icons'

type PlayerBarProps = {
  song: Song | null
  audioUrl: string | null
  audioCommand: AudioPlayerCommand
  hasBufferedSong: boolean
  hasPreviousSong: boolean
  hasNextSong: boolean
  onAudioPlay: () => void
  onAudioPause: () => void
  onStopPlayback: () => void
  onAudioEnded: () => void
  onPlayPreviousSong: () => void
  onPlayNextSong: () => void
}

export function PlayerBar({
  song,
  audioUrl,
  audioCommand,
  hasBufferedSong,
  hasPreviousSong,
  hasNextSong,
  onAudioPlay,
  onAudioPause,
  onStopPlayback,
  onAudioEnded,
  onPlayPreviousSong,
  onPlayNextSong,
}: PlayerBarProps) {
  return (
    <footer className="player-bar">
      <div className="player-now">
        {song ? (
          <>
            <div>
              <strong>{song.title}</strong>
              <span>{song.artist}</span>
            </div>
          </>
        ) : (
          <div className="player-empty-copy">
            <strong>No hay ninguna canción seleccionada</strong>
            <span>Carga canciones desde el servidor para comenzar a reproducir</span>
          </div>
        )}
      </div>

      <div className="player-center">
        <AudioPlayer
          audioUrl={audioUrl}
          command={audioCommand}
          onPlay={onAudioPlay}
          onPause={onAudioPause}
          onEnded={onAudioEnded}
        />
      </div>

      <div className="player-side">
        <button type="button" onClick={onPlayPreviousSong} disabled={!hasPreviousSong}>
          <BackwardIcon />
        </button>
        {hasBufferedSong && (
          <button type="button" className="stop-button" onClick={onStopPlayback}>
            <StopCircleIcon />
          </button>
        )}
        <button type="button" onClick={onPlayNextSong} disabled={!hasNextSong}>
          <ForwardIcon />
        </button>
      </div>
    </footer>
  )
}
