import { useEffect, useState } from 'react'
import { sendWebSocketRequest } from '../../../api/websocketClient'
import type { ServerSuccessResponse } from '../../../api/protocol'
import type {
  AddSongToPlaylistPayload,
  CreatePlaylistPayload,
  FilterPlaylistSongsPayload,
  ListPlaylistsData,
  PlaylistDto,
  PlaylistResponseData,
  PlaylistSongsData,
  PlaylistSummaryData,
  PlaylistSummaryDto,
  RemoveSongFromPlaylistPayload,
  SongDto,
  SortPlaylistSongsPayload,
} from '../types'

type UsePlaylistsResult = {
  playlists: PlaylistDto[]
  loading: boolean
  error: string | null
  createError: string | null
  createLoading: boolean
  detailSongs: SongDto[] | null
  summary: PlaylistSummaryDto | null
  detailError: string | null
  actionLoading: boolean
  createPlaylist: (name: string) => Promise<boolean>
  addSongToPlaylist: (playlistId: string, songId: string) => Promise<boolean>
  removeSongFromPlaylist: (playlistId: string, songId: string) => Promise<boolean>
  filterPlaylistSongs: (
    playlistId: string,
    criteria: 'title' | 'artist' | 'genre',
    value: string,
  ) => Promise<void>
  sortPlaylistSongs: (
    playlistId: string,
    criteria: 'title' | 'artist' | 'duration',
    direction: 'asc' | 'desc',
  ) => Promise<void>
  loadPlaylistSummary: (playlistId: string) => Promise<void>
  clearDetailSongs: () => void
  reload: () => void
}

export function usePlaylists(): UsePlaylistsResult {
  const [playlists, setPlaylists] = useState<PlaylistDto[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [createError, setCreateError] = useState<string | null>(null)
  const [createLoading, setCreateLoading] = useState(false)
  const [detailSongs, setDetailSongs] = useState<SongDto[] | null>(null)
  const [summary, setSummary] = useState<PlaylistSummaryDto | null>(null)
  const [detailError, setDetailError] = useState<string | null>(null)
  const [actionLoading, setActionLoading] = useState(false)
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
            : 'Ocurrió un error inesperado al cargar las playlists',
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
    detailSongs,
    summary,
    detailError,
    actionLoading,
    createPlaylist,
    addSongToPlaylist,
    removeSongFromPlaylist,
    filterPlaylistSongs,
    sortPlaylistSongs,
    loadPlaylistSummary,
    clearDetailSongs: () => setDetailSongs(null),
    reload: () => setReloadToken((token) => token + 1),
  }

  async function createPlaylist(name: string) {
    const trimmedName = name.trim()

    if (trimmedName.length === 0) {
      setCreateError('El nombre de la playlist no puede estar vacío')
      return false
    }

    setCreateLoading(true)
    setCreateError(null)

    try {
      const response = await sendWebSocketRequest<PlaylistResponseData, CreatePlaylistPayload>({
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

      const success = response as ServerSuccessResponse<PlaylistResponseData>
      setPlaylists((currentPlaylists) => [...currentPlaylists, success.data.playlist])
      return true
    } catch (requestError) {
      setCreateError(
        requestError instanceof Error
          ? requestError.message
          : 'Ocurrió un error inesperado al crear la playlist',
      )
      return false
    } finally {
      setCreateLoading(false)
    }
  }

  async function addSongToPlaylist(playlistId: string, songId: string) {
    return updatePlaylist('add_song_to_playlist', {
      playlist_id: playlistId,
      song_id: songId,
    } satisfies AddSongToPlaylistPayload)
  }

  async function removeSongFromPlaylist(playlistId: string, songId: string) {
    return updatePlaylist('remove_song_from_playlist', {
      playlist_id: playlistId,
      song_id: songId,
    } satisfies RemoveSongFromPlaylistPayload)
  }

  async function filterPlaylistSongs(
    playlistId: string,
    criteria: 'title' | 'artist' | 'genre',
    value: string,
  ) {
    await loadPlaylistSongs('filter_playlist_songs', {
      playlist_id: playlistId,
      criteria,
      value,
    } satisfies FilterPlaylistSongsPayload)
  }

  async function sortPlaylistSongs(
    playlistId: string,
    criteria: 'title' | 'artist' | 'duration',
    direction: 'asc' | 'desc',
  ) {
    await loadPlaylistSongs('sort_playlist_songs', {
      playlist_id: playlistId,
      criteria,
      direction,
    } satisfies SortPlaylistSongsPayload)
  }

  async function loadPlaylistSummary(playlistId: string) {
    setActionLoading(true)
    setDetailError(null)

    try {
      const response = await sendWebSocketRequest<PlaylistSummaryData>({
        request_id: `req-${Date.now()}`,
        action: 'get_playlist_summary',
        payload: {
          playlist_id: playlistId,
        },
      })

      if (response.status === 'error') {
        setDetailError(response.error.message)
        setSummary(null)
        return
      }

      const success = response as ServerSuccessResponse<PlaylistSummaryData>
      setSummary(success.data.summary)
    } catch (requestError) {
      setDetailError(
        requestError instanceof Error
          ? requestError.message
          : 'Ocurrió un error inesperado al cargar el resumen de la playlist',
      )
      setSummary(null)
    } finally {
      setActionLoading(false)
    }
  }

  async function updatePlaylist<TPayload>(action: string, payload: TPayload) {
    setActionLoading(true)
    setDetailError(null)

    try {
      const response = await sendWebSocketRequest<PlaylistResponseData, TPayload>({
        request_id: `req-${Date.now()}`,
        action,
        payload,
      })

      if (response.status === 'error') {
        setDetailError(response.error.message)
        return false
      }

      const success = response as ServerSuccessResponse<PlaylistResponseData>
      setPlaylists((currentPlaylists) =>
        currentPlaylists.map((playlist) =>
          playlist.id === success.data.playlist.id ? success.data.playlist : playlist,
        ),
      )
      setDetailSongs(null)
      return true
    } catch (requestError) {
      setDetailError(
        requestError instanceof Error
          ? requestError.message
          : 'Ocurrió un error inesperado al actualizar la playlist',
      )
      return false
    } finally {
      setActionLoading(false)
    }
  }

  async function loadPlaylistSongs<TPayload>(action: string, payload: TPayload) {
    setActionLoading(true)
    setDetailError(null)

    try {
      const response = await sendWebSocketRequest<PlaylistSongsData, TPayload>({
        request_id: `req-${Date.now()}`,
        action,
        payload,
      })

      if (response.status === 'error') {
        setDetailError(response.error.message)
        setDetailSongs([])
        return
      }

      const success = response as ServerSuccessResponse<PlaylistSongsData>
      setDetailSongs(success.data.songs)
    } catch (requestError) {
      setDetailError(
        requestError instanceof Error
          ? requestError.message
          : 'Ocurrió un error inesperado al cargar las canciones de la playlist',
      )
      setDetailSongs([])
    } finally {
      setActionLoading(false)
    }
  }
}
