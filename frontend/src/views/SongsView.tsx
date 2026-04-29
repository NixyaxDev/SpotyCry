import { SongSearch } from '../features/songs/components/SongSearch'
import { SongList } from '../features/songs/components/SongList'
import type { SongListItem } from '../features/songs/types'

type SongsViewProps = {
  songs: SongListItem[]
  loading: boolean
  error: string | null
  onReload: () => void
  searchValue: string
  onSearchChange: (value: string) => void
}

export function SongsView({
  songs,
  loading,
  error,
  onReload,
  searchValue,
  onSearchChange,
}: SongsViewProps) {
  return (
    <>
      <section className="panel">
        <div className="panel-title-row">
          <h3>All Songs</h3>
        </div>
        <SongSearch
          initialValue={searchValue}
          onSearchChange={onSearchChange}
          loading={loading}
        />

        {loading && <div className="feedback-card">Loading songs...</div>}

        {!loading && error && (
          <div className="feedback-card feedback-card--error">
            <p>{error}</p>
            <button type="button" className="primary-button" onClick={onReload}>
              Try again
            </button>
          </div>
        )}

        {!loading && !error && songs.length === 0 && (
          <div className="feedback-card">
            <p>{searchValue.trim().length > 0 ? 'No songs found' : 'No songs available'}</p>
            <button type="button" className="primary-button" onClick={onReload}>
              Refresh
            </button>
          </div>
        )}

        {!loading && !error && songs.length > 0 && <SongList songs={songs} />}
      </section>
    </>
  )
}
