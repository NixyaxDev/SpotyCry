import { useEffect, useState } from 'react'
import { sendWebSocketRequest } from '../../../api/websocketClient'
import type { ServerSuccessResponse } from '../../../api/protocol'
import type { ListSongsData, SongDto } from '../types'

type UseSongsResult = {
  songs: SongDto[]
  loading: boolean
  error: string | null
  reload: () => void
}

export function useSongs(): UseSongsResult {
  const [songs, setSongs] = useState<SongDto[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [reloadToken, setReloadToken] = useState(0)

  useEffect(() => {
    let isMounted = true

    async function fetchSongs() {
      setLoading(true)
      setError(null)

      try {
        const response = await sendWebSocketRequest<ListSongsData>({
          request_id: `req-${Date.now()}`,
          action: 'list_songs',
          payload: {},
        })

        if (!isMounted) {
          return
        }

        if (response.status === 'error') {
          setError(response.error.message)
          setSongs([])
          return
        }

        const success = response as ServerSuccessResponse<ListSongsData>
        setSongs(success.data.songs)
      } catch (requestError) {
        if (!isMounted) {
          return
        }

        setSongs([])
        setError(
          requestError instanceof Error
            ? requestError.message
            : 'Unexpected error while loading songs',
        )
      } finally {
        if (isMounted) {
          setLoading(false)
        }
      }
    }

    fetchSongs()

    return () => {
      isMounted = false
    }
  }, [reloadToken])

  return {
    songs,
    loading,
    error,
    reload: () => setReloadToken((token) => token + 1),
  }
}
