import { useState } from 'react'
import { AddSongToPlaylist } from '../features/playlists/components/AddSongToPlaylist'
import { PlaylistFilter } from '../features/playlists/components/PlaylistFilter'
import { PlaylistSort } from '../features/playlists/components/PlaylistSort'
import { PlaylistSummary } from '../features/playlists/components/PlaylistSummary'
import type { PlaylistSummaryDto, SongDto as PlaylistSongDto } from '../features/playlists/types'
import type { Song } from '../types/music'
import type { Playlist } from '../types/music'
import { ClockIcon, PlayIcon, QueueListIcon, SignalIcon } from '../shared/icons'

type PlaylistDetailViewProps = {
  playlist: Playlist | null
  songs: Song[]
  availableSongs: PlaylistSongDto[]
  summary: PlaylistSummaryDto | null
  error: string | null
  actionLoading: boolean
  onAddSong: (songId: string) => Promise<void>
  onRemoveSong: (songId: string) => Promise<void>
  onFilterSongs: (
    criteria: 'title' | 'artist' | 'genre',
    value: string,
  ) => Promise<void>
  onSortSongs: (
    criteria: 'title' | 'artist' | 'duration',
    direction: 'asc' | 'desc',
  ) => Promise<void>
  onPlayPlaylist: () => Promise<void>
  onResetSongView: () => void
  selectedSong: Song | null
}

export function PlaylistDetailView({
  playlist,
  songs,
  availableSongs,
  summary,
  error,
  actionLoading,
  onAddSong,
  onRemoveSong,
  onFilterSongs,
  onSortSongs,
  onPlayPlaylist,
  onResetSongView,
  selectedSong,
}: PlaylistDetailViewProps) {
  const [selectedSongId, setSelectedSongId] = useState('')
  const [filterCriteria, setFilterCriteria] = useState<'title' | 'artist' | 'genre'>('title')
  const [filterValue, setFilterValue] = useState('')
  const [sortCriteria, setSortCriteria] = useState<'title' | 'artist' | 'duration'>('title')
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc')

  return (
    <>
      <section className="playlist-hero">
        <div className="playlist-hero-symbol" aria-hidden="true">
          <QueueListIcon />
        </div>
        <div className="playlist-hero-copy">
          <p className="eyebrow">Public Playlist</p>
          <h2>{playlist?.name ?? 'Playlist Detail'}</h2>
          <p className="playlist-description">
            This playlist is stored in the server memory and currently contains the
            songs assigned to it in the active session.
          </p>
          <div className="playlist-meta">
            <span>{songs.length} Songs</span>
            <span>•</span>
            <span>Server-backed playlist</span>
          </div>
          <div className="playlist-actions">
            <button
              type="button"
              className="play-button-large"
              onClick={() => {
                void onPlayPlaylist()
              }}
              disabled={(playlist?.songIds.length ?? 0) === 0}
            >
              <PlayIcon />
            </button>
          </div>
        </div>
      </section>

      <PlaylistSummary summary={summary} />

      <section className="playlist-tools-grid">
        <AddSongToPlaylist
          songs={availableSongs}
          selectedSongId={selectedSongId}
          onSelectedSongIdChange={setSelectedSongId}
          onAddSong={handleAddSong}
          loading={actionLoading}
        />
        <PlaylistFilter
          criteria={filterCriteria}
          value={filterValue}
          onCriteriaChange={setFilterCriteria}
          onValueChange={setFilterValue}
          onApply={handleFilterSongs}
          onReset={handleResetSongView}
          loading={actionLoading}
        />
        <PlaylistSort
          criteria={sortCriteria}
          direction={sortDirection}
          onCriteriaChange={setSortCriteria}
          onDirectionChange={setSortDirection}
          onApply={handleSortSongs}
          onReset={handleResetSongView}
          loading={actionLoading}
        />
      </section>

      {error && <div className="feedback-card feedback-card--error"><p>{error}</p></div>}

      <section className="panel">
        <div className="track-list-header">
          <span>#</span>
          <span>Title</span>
          <span>Album</span>
          <span>Actions</span>
          <span>
            <ClockIcon />
          </span>
        </div>
        <div className="track-list">
          {songs.length > 0 ? (
            songs.map((song, index) => (
              <article
                key={song.id}
                className={song.id === selectedSong?.id ? 'track-row is-playing' : 'track-row'}
              >
                <div className="track-index">
                  {song.id === selectedSong?.id ? (
                    <SignalIcon />
                  ) : (
                    index + 1
                  )}
                </div>
                <div className="track-title">
                  <div>
                    <strong>{song.title}</strong>
                    <span>{song.artist}</span>
                  </div>
                </div>
                <div className="track-album">{song.album}</div>
                <div className="track-actions">
                  <button
                    type="button"
                    className="secondary-button secondary-button--compact"
                    onClick={() => {
                      void onRemoveSong(song.id)
                    }}
                    disabled={actionLoading}
                  >
                    Remove
                  </button>
                </div>
                <div className="track-duration">{song.duration}</div>
              </article>
            ))
          ) : (
            <div className="feedback-card">
              <p>No songs available in the server catalog</p>
            </div>
          )}
        </div>
      </section>
    </>
  )

  async function handleAddSong() {
    if (!selectedSongId) {
      return
    }

    await onAddSong(selectedSongId)
    setSelectedSongId('')
  }

  async function handleFilterSongs() {
    await onFilterSongs(filterCriteria, filterValue)
  }

  async function handleSortSongs() {
    await onSortSongs(sortCriteria, sortDirection)
  }

  function handleResetSongView() {
    setFilterValue('')
    setFilterCriteria('title')
    setSortCriteria('title')
    setSortDirection('asc')
    onResetSongView()
  }
}
