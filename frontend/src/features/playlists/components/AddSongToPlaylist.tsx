import type { SongDto } from '../types'

type AddSongToPlaylistProps = {
  songs: SongDto[]
  selectedSongId: string
  onSelectedSongIdChange: (songId: string) => void
  onAddSong: () => Promise<void>
  loading: boolean
}

export function AddSongToPlaylist({
  songs,
  selectedSongId,
  onSelectedSongIdChange,
  onAddSong,
  loading,
}: AddSongToPlaylistProps) {
  return (
    <div className="playlist-control-card">
      <p className="eyebrow">Agregar canción</p>
      <div className="playlist-inline-form">
        <select
          value={selectedSongId}
          onChange={(event) => onSelectedSongIdChange(event.target.value)}
          disabled={loading || songs.length === 0}
        >
          <option value="">Selecciona una canción</option>
          {songs.map((song) => (
            <option key={song.id} value={song.id}>
              {song.title}
            </option>
          ))}
        </select>
        <button
          type="button"
          className="primary-button"
          onClick={() => {
            void onAddSong()
          }}
          disabled={loading || selectedSongId.length === 0 || songs.length === 0}
        >
          Agregar
        </button>
      </div>
    </div>
  )
}
