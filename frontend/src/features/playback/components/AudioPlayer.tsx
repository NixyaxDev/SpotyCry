import { useEffect, useRef } from 'react'

type AudioPlayerProps = {
  audioUrl: string | null
  onPlay?: () => void
  onPause?: () => void
  onEnded?: () => void
}

export function AudioPlayer({
  audioUrl,
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
