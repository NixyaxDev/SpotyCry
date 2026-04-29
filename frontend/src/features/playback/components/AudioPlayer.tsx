import { useEffect, useRef } from 'react'

type AudioPlayerProps = {
  audioUrl: string | null
  onEnded?: () => void
}

export function AudioPlayer({ audioUrl, onEnded }: AudioPlayerProps) {
  const audioRef = useRef<HTMLAudioElement | null>(null)

  useEffect(() => {
    if (!audioRef.current) {
      return
    }

    if (!audioUrl) {
      audioRef.current.pause()
      audioRef.current.removeAttribute('src')
      audioRef.current.load()
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
      onEnded={onEnded}
    />
  )
}
