import { useEffect, useMemo, useRef, useState } from 'react'
import type {
  ClientRequest,
  ServerErrorResponse,
  ServerSuccessResponse,
  StartPlaybackPayload,
} from '../../../api/protocol'
import type { Song } from '../../../types/music'
import type {
  AudioChunkEvent,
  StartPlaybackData,
  StopPlaybackData,
} from '../types'

const WS_URL = 'ws://127.0.0.1:8080'

type PlaybackState = {
  audioUrl: string | null
  mimeType: string | null
  loading: boolean
  error: string | null
  currentSongId: string | null
  isPlaying: boolean
  startPlayback: (song: Song) => Promise<void>
  stopPlayback: () => Promise<void>
  markAudioPlaying: () => void
  markAudioStopped: () => void
}

export function usePlayback(): PlaybackState {
  const [audioUrl, setAudioUrl] = useState<string | null>(null)
  const [mimeType, setMimeType] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [currentSongId, setCurrentSongId] = useState<string | null>(null)
  const [isPlaying, setIsPlaying] = useState(false)
  const objectUrlRef = useRef<string | null>(null)
  const socketRef = useRef<WebSocket | null>(null)
  const streamIdRef = useRef<string | null>(null)

  useEffect(() => {
    return () => {
      if (objectUrlRef.current) {
        URL.revokeObjectURL(objectUrlRef.current)
      }

      socketRef.current?.close()
    }
  }, [])

  async function startPlayback(song: Song) {
    if (streamIdRef.current && socketRef.current) {
      await stopPlayback()
    }

    setLoading(true)
    setError(null)
    setIsPlaying(false)

    if (objectUrlRef.current) {
      URL.revokeObjectURL(objectUrlRef.current)
      objectUrlRef.current = null
    }

    socketRef.current?.close()

    const socket = new WebSocket(WS_URL)
    socketRef.current = socket

    const chunks = new Map<number, Uint8Array>()
    let playbackMeta: StartPlaybackData | null = null

    const request: ClientRequest<StartPlaybackPayload> = {
      request_id: `req-${Date.now()}`,
      action: 'start_playback',
      payload: {
        song_id: song.id,
      },
    }

    return new Promise<void>((resolve, reject) => {
      socket.addEventListener('open', () => {
        socket.send(JSON.stringify(request))
      })

      socket.addEventListener('message', (event) => {
        try {
          const parsed = JSON.parse(event.data) as
            | ServerSuccessResponse<StartPlaybackData>
            | ServerErrorResponse
            | AudioChunkEvent

          if ('status' in parsed) {
            if (parsed.status === 'error') {
              setLoading(false)
              setError(parsed.error.message)
              reject(new Error(parsed.error.message))
              setIsPlaying(false)
              return
            }

            playbackMeta = parsed.data
            setMimeType(parsed.data.mime_type)
            setCurrentSongId(parsed.data.song_id)
            streamIdRef.current = parsed.data.stream_id
            return
          }

          if (parsed.type === 'audio_chunk') {
            chunks.set(parsed.chunk_index, decodeBase64Chunk(parsed.bytes_base64))

            if (parsed.is_last && playbackMeta) {
              const orderedChunks = [...chunks.entries()]
                .sort(([left], [right]) => left - right)
                .map(([, value]) => value.slice().buffer as ArrayBuffer)

              const blob = new Blob(orderedChunks, {
                type: playbackMeta.mime_type,
              })
              const nextAudioUrl = URL.createObjectURL(blob)

              objectUrlRef.current = nextAudioUrl
              setAudioUrl(nextAudioUrl)
              setLoading(false)
              resolve()
            }
          }
        } catch (streamError) {
          const message =
            streamError instanceof Error
              ? streamError.message
              : 'Could not process playback stream'

          setLoading(false)
          setError(message)
          setIsPlaying(false)
          reject(new Error(message))
          socket.close()
        }
      })

      socket.addEventListener('error', () => {
        const message = 'Could not connect to the playback stream'
        setLoading(false)
        setError(message)
        setIsPlaying(false)
        reject(new Error(message))
      })
    })
  }

  async function stopPlayback() {
    if (!streamIdRef.current || !currentSongId) {
      clearLocalPlaybackState()
      return
    }

    const streamId = streamIdRef.current
    const songId = currentSongId

    const request: ClientRequest<{ stream_id: string; song_id: string }> = {
      request_id: `req-${Date.now()}`,
      action: 'stop_playback',
      payload: {
        stream_id: streamId,
        song_id: songId,
      },
    }

    await new Promise<void>((resolve, reject) => {
      const controlSocket = new WebSocket(WS_URL)

      controlSocket.addEventListener('open', () => {
        controlSocket.send(JSON.stringify(request))
      })

      controlSocket.addEventListener('message', (event) => {
        try {
          const parsed = JSON.parse(event.data) as
            | ServerSuccessResponse<StopPlaybackData>
            | ServerErrorResponse

          if ('status' in parsed) {
            if (parsed.status === 'error') {
              setError(parsed.error.message)
              controlSocket.close()
              reject(new Error(parsed.error.message))
              return
            }

            socketRef.current?.close()
            clearLocalPlaybackState()
            controlSocket.close()
            resolve()
          }
        } catch (stopError) {
          controlSocket.close()
          reject(stopError)
        }
      })

      controlSocket.addEventListener('error', () => {
        controlSocket.close()
        reject(new Error('Could not stop playback on the server'))
      })
    })
  }

  return useMemo(
    () => ({
      audioUrl,
      mimeType,
      loading,
      error,
      currentSongId,
      isPlaying,
      startPlayback,
      stopPlayback,
      markAudioPlaying,
      markAudioStopped,
    }),
    [audioUrl, mimeType, loading, error, currentSongId, isPlaying],
  )

  function clearLocalPlaybackState() {
    if (objectUrlRef.current) {
      URL.revokeObjectURL(objectUrlRef.current)
      objectUrlRef.current = null
    }

    setAudioUrl(null)
    setMimeType(null)
    setLoading(false)
    setCurrentSongId(null)
    setIsPlaying(false)
    streamIdRef.current = null
  }

  function markAudioPlaying() {
    setIsPlaying(true)
    setError(null)
  }

  function markAudioStopped() {
    setIsPlaying(false)
  }
}

function decodeBase64Chunk(value: string): Uint8Array {
  if (value.length === 0) {
    return new Uint8Array()
  }

  const decoded = atob(value)
  const bytes = new Uint8Array(decoded.length)

  for (let index = 0; index < decoded.length; index += 1) {
    bytes[index] = decoded.charCodeAt(index)
  }

  return bytes
}
