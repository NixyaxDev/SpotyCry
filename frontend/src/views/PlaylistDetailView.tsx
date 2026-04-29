import { useState } from 'react'
import { AddSongToPlaylist } from '../features/playlists/components/AddSongToPlaylist'
import { PlaylistFilter } from '../features/playlists/components/PlaylistFilter'
import { PlaylistSort } from '../features/playlists/components/PlaylistSort'
import { PlaylistSummary } from '../features/playlists/components/PlaylistSummary'
import type { PlaylistSummaryDto, SongDto as PlaylistSongDto } from '../features/playlists/types'
import type { Song } from '../types/music'
import type { Playlist } from '../types/music'

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
  onResetSongView: () => void
  selectedSong: Song | null
}

const playlistCover =
  'https://lh3.googleusercontent.com/aida-public/AB6AXuAyrc6FpM7hRBM0d12cHAPehjZD7sSeArtp7vFA-iW9lpnOMPTrhrvcEnQfff0uYSkf8FY7VgowcTZE_oFt-fZ0F_MjwQJpRWq6YXmxvB0kKCsBEB0uDeq-w4OAsU444teqad-lmqA8uyiLLc_y1f8CYmbG3YJs7TFVorhjHxNG8usrAlnc6hFFZkF3XbyWlVTwMlJ_1Dutjek8H9MV9QQxOjKm4gV6qTt8Nk8Z5YeBGBOLUf8i9e2oYDSFzS999bNPWHBHlaAON04'

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
        <img
          className="playlist-hero-cover"
          src={playlistCover}
          alt={playlist?.name ?? 'Playlist cover'}
        />
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
            <button type="button" className="play-button-large">
              <span className="material-symbols-outlined fillable">play_arrow</span>
            </button>
            <button type="button" className="icon-button">
              <span className="material-symbols-outlined">favorite_border</span>
            </button>
            <button type="button" className="icon-button">
              <span className="material-symbols-outlined">more_horiz</span>
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
            <span className="material-symbols-outlined">schedule</span>
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
                    <span className="material-symbols-outlined">equalizer</span>
                  ) : (
                    index + 1
                  )}
                </div>
                <div className="track-title">
                  <img src={song.cover} alt={song.title} />
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
