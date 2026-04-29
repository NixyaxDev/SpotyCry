import type { SongListItem } from '../types'
import { ClockIcon, PlayCircleIcon, StopCircleIcon } from '../../../shared/icons'

type SongListProps = {
  songs: SongListItem[]
  onPlay: (songId: string) => void
  isPlaybackLoading: boolean
  activeSongId: string | null
  isPlaying: boolean
}

export function SongList({
  songs,
  onPlay,
  isPlaybackLoading,
  activeSongId,
  isPlaying,
}: SongListProps) {
  return (
    <div className="songs-table-shell">
      <table className="songs-table">
        <thead>
          <tr>
            <th>#</th>
            <th>Título</th>
            <th>Artista</th>
            <th>Género</th>
            <th>Reproducir</th>
            <th>
              <ClockIcon />
            </th>
          </tr>
        </thead>
        <tbody>
          {songs.map((song, index) => (
            <tr key={song.id} className={song.id === activeSongId ? 'is-current' : ''}>
              <td>{index + 1}</td>
              <td>
                <div className="song-cell">
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
                  {song.id === activeSongId && isPlaying ? <StopCircleIcon /> : <PlayCircleIcon />}
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
