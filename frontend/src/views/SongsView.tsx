import { recentSongs } from '../data/mockData'
import type { Song } from '../types/music'
import { SectionHeader } from '../components/SectionHeader'

type SongsViewProps = {
  songs: Song[]
  selectedSong: Song
}

export function SongsView({ songs, selectedSong }: SongsViewProps) {
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
        <div className="songs-table-shell">
          <table className="songs-table">
            <thead>
              <tr>
                <th>#</th>
                <th>Title</th>
                <th>Artist</th>
                <th>Genre</th>
                <th>
                  <span className="material-symbols-outlined">schedule</span>
                </th>
              </tr>
            </thead>
            <tbody>
              {songs.map((song, index) => (
                <tr key={song.id} className={song.id === selectedSong.id ? 'is-current' : ''}>
                  <td>{song.id === selectedSong.id ? '•' : index + 1}</td>
                  <td>
                    <div className="song-cell">
                      <img src={song.cover} alt={song.title} />
                      <div>
                        <strong>{song.title}</strong>
                        <span>{song.album}</span>
                      </div>
                    </div>
                  </td>
                  <td>{song.artist}</td>
                  <td>{song.genre}</td>
                  <td>{song.duration}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
    </>
  )
}
