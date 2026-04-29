import { recentSongs } from '../data/mockData'
import { SectionHeader } from '../components/SectionHeader'
import { SongList } from '../features/songs/components/SongList'
import type { SongListItem } from '../features/songs/types'

type SongsViewProps = {
  songs: SongListItem[]
  loading: boolean
  error: string | null
  onReload: () => void
}

export function SongsView({ songs, loading, error, onReload }: SongsViewProps) {
  return (
    <>
      <SectionHeader
        title="Your Library"
        subtitle="A collection of melancholy and introspection."
        action={
          <button type="button" className="primary-button">
            <span className="material-symbols-outlined">shuffle</span>
            Shuffle
          </button>
        }
      />

      <section className="panel">
        <div className="panel-title-row">
          <h3>Recently Echoed</h3>
        </div>
        <div className="card-rail">
          {recentSongs.map((song) => (
            <article key={song.title} className="album-card">
              <div className="album-card-art">
                <img src={song.cover} alt={song.title} />
                <div className="overlay-play">
                  <span className="material-symbols-outlined fillable">play_circle</span>
                </div>
              </div>
              <h4>{song.title}</h4>
              <p>{song.artist}</p>
            </article>
          ))}
        </div>
      </section>

      <section className="panel">
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
            <p>No songs available</p>
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
