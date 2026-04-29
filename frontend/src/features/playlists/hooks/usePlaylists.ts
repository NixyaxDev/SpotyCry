import { useEffect, useState } from 'react'
import { sendWebSocketRequest } from '../../../api/websocketClient'
import type { ServerSuccessResponse } from '../../../api/protocol'
import type {
  CreatePlaylistData,
  CreatePlaylistPayload,
  ListPlaylistsData,
  PlaylistDto,
} from '../types'

type UsePlaylistsResult = {
  playlists: PlaylistDto[]
  loading: boolean
  error: string | null
  createError: string | null
  createLoading: boolean
  createPlaylist: (name: string) => Promise<boolean>
  reload: () => void
}

export function usePlaylists(): UsePlaylistsResult {
  const [playlists, setPlaylists] = useState<PlaylistDto[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [createError, setCreateError] = useState<string | null>(null)
  const [createLoading, setCreateLoading] = useState(false)
  const [reloadToken, setReloadToken] = useState(0)

  useEffect(() => {
    let isMounted = true

    async function fetchPlaylists() {
      setLoading(true)
      setError(null)

      try {
        const response = await sendWebSocketRequest<ListPlaylistsData>({
          request_id: `req-${Date.now()}`,
          action: 'list_playlists',
          payload: {},
        })

        if (!isMounted) {
          return
        }

        if (response.status === 'error') {
          setError(response.error.message)
          setPlaylists([])
          return
        }

        const success = response as ServerSuccessResponse<ListPlaylistsData>
        setPlaylists(success.data.playlists)
      } catch (requestError) {
        if (!isMounted) {
          return
        }

        setPlaylists([])
        setError(
          requestError instanceof Error
            ? requestError.message
            : 'Unexpected error while loading playlists',
        )
      } finally {
        if (isMounted) {
          setLoading(false)
        }
      }
    }

    fetchPlaylists()

    return () => {
      isMounted = false
    }
  }, [reloadToken])

  return {
    playlists,
    loading,
    error,
    createError,
    createLoading,
    createPlaylist,
    reload: () => setReloadToken((token) => token + 1),
  }

  async function createPlaylist(name: string) {
    const trimmedName = name.trim()

    if (trimmedName.length === 0) {
      setCreateError('Playlist name cannot be empty')
      return false
    }

    setCreateLoading(true)
    setCreateError(null)

    try {
      const response = await sendWebSocketRequest<
        CreatePlaylistData,
        CreatePlaylistPayload
      >({
        request_id: `req-${Date.now()}`,
        action: 'create_playlist',
        payload: {
          name: trimmedName,
        },
      })

      if (response.status === 'error') {
        setCreateError(response.error.message)
        return false
      }

      const success = response as ServerSuccessResponse<CreatePlaylistData>
      setPlaylists((currentPlaylists) => [...currentPlaylists, success.data.playlist])
      return true
    } catch (requestError) {
      setCreateError(
        requestError instanceof Error
          ? requestError.message
          : 'Unexpected error while creating playlist',
      )
      return false
    } finally {
      setCreateLoading(false)
    }
  }
}
