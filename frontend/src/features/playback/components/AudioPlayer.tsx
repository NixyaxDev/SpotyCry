import { useEffect, useRef } from 'react'

export type AudioPlayerCommand =
  | {
      type: 'play' | 'pause'
      token: number
    }
  | null

type AudioPlayerProps = {
  audioUrl: string | null
  command?: AudioPlayerCommand
  onPlay?: () => void
  onPause?: () => void
  onEnded?: () => void
}

export function AudioPlayer({
  audioUrl,
  command = null,
  onPlay,
  onPause,
  onEnded,
}: AudioPlayerProps) {
  const audioRef = useRef<HTMLAudioElement | null>(null)
  const ignorePauseRef = useRef(false)

  useEffect(() => {
    if (!audioRef.current) {
      return
    }

    if (!audioUrl) {
      ignorePauseRef.current = true
      audioRef.current.pause()
      audioRef.current.removeAttribute('src')
      audioRef.current.load()
      window.setTimeout(() => {
        ignorePauseRef.current = false
      }, 0)
      return
    }

    audioRef.current
      .play()
      .catch(() => {
        // Autoplay can be blocked by the browser; controls remain available.
      })
  }, [audioUrl])

  useEffect(() => {
    if (!audioRef.current || !command) {
      return
    }

    if (command.type === 'pause') {
      ignorePauseRef.current = true
      audioRef.current.pause()
      window.setTimeout(() => {
        ignorePauseRef.current = false
      }, 0)
      return
    }

    audioRef.current
      .play()
      .catch(() => {
        // Autoplay can be blocked by the browser; controls remain available.
      })
  }, [command])

  return (
    <audio
      ref={audioRef}
      className="audio-player"
      controls
      src={audioUrl ?? undefined}
      onPlay={onPlay}
      onPause={() => {
        if (
          !ignorePauseRef.current &&
          audioRef.current &&
          audioRef.current.currentTime < audioRef.current.duration
        ) {
          onPause?.()
        }
      }}
      onEnded={onEnded}
    />
  )
}
