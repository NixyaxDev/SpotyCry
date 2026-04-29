import type { SongListItem } from '../types'

type SongListProps = {
  songs: SongListItem[]
  onPlay: (songId: string) => void
  isPlaybackLoading: boolean
  activeSongId: string | null
}

export function SongList({
  songs,
  onPlay,
  isPlaybackLoading,
  activeSongId,
}: SongListProps) {
  return (
    <div className="songs-table-shell">
      <table className="songs-table">
        <thead>
          <tr>
            <th>#</th>
            <th>Title</th>
            <th>Artist</th>
            <th>Genre</th>
            <th>Play</th>
            <th>
              <span className="material-symbols-outlined">schedule</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {songs.map((song, index) => (
            <tr key={song.id} className={song.id === activeSongId ? 'is-current' : ''}>
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
              <td>
                <button
                  type="button"
                  className="table-play-button"
                  onClick={() => onPlay(song.id)}
                  disabled={isPlaybackLoading}
                >
                  <span className="material-symbols-outlined fillable">
                    {song.id === activeSongId ? 'pause_circle' : 'play_circle'}
                  </span>
                </button>
              </td>
              <td>{song.duration}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
