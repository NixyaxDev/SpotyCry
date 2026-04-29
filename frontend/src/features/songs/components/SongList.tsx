import type { SongListItem } from '../types'

type SongListProps = {
  songs: SongListItem[]
}

export function SongList({ songs }: SongListProps) {
  return (
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
            <tr key={song.id}>
              <td>{index + 1}</td>
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
  )
}
