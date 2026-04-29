import { useEffect, useState } from 'react'
import { sendWebSocketRequest } from '../../../api/websocketClient'
import type { ServerSuccessResponse } from '../../../api/protocol'
import type { ListSongsData, SearchSongsPayload, SongDto } from '../types'

type UseSongsResult = {
  songs: SongDto[]
  loading: boolean
  error: string | null
  reload: () => void
  searchCriteria: SearchSongsPayload['criteria']
  setSearchCriteria: (criteria: SearchSongsPayload['criteria']) => void
  searchValue: string
  setSearchValue: (value: string) => void
}

export function useSongs(): UseSongsResult {
  const [songs, setSongs] = useState<SongDto[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [reloadToken, setReloadToken] = useState(0)
  const [searchCriteria, setSearchCriteria] = useState<SearchSongsPayload['criteria']>('title')
  const [searchValue, setSearchValue] = useState('')

  useEffect(() => {
    let isMounted = true

    async function fetchSongs() {
      setLoading(true)
      setError(null)

      try {
        const trimmedValue = searchValue.trim()
        const response =
          trimmedValue.length === 0
            ? await sendWebSocketRequest<ListSongsData>({
                request_id: `req-${Date.now()}`,
                action: 'list_songs',
                payload: {},
              })
            : await sendWebSocketRequest<ListSongsData, SearchSongsPayload>({
                request_id: `req-${Date.now()}`,
                action: 'search_songs',
                payload: {
                  criteria: searchCriteria,
                  value: searchValue,
                },
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
            : 'Ocurrió un error inesperado al cargar las canciones',
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
  }, [reloadToken, searchCriteria, searchValue])

  return {
    songs,
    loading,
    error,
    reload: () => setReloadToken((token) => token + 1),
    searchCriteria,
    setSearchCriteria,
    searchValue,
    setSearchValue,
  }
}
