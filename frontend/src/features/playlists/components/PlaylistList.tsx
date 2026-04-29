import type { PlaylistDto } from '../types'
import { PlayIcon, QueueListIcon } from '../../../shared/icons'

type PlaylistListProps = {
  playlists: PlaylistDto[]
  onOpenPlaylist: (playlistId: string) => void
  onPlayPlaylist: (playlistId: string) => void
}

export function PlaylistList({
  playlists,
  onOpenPlaylist,
  onPlayPlaylist,
}: PlaylistListProps) {
  return (
    <section className="playlist-grid">
      {playlists.map((playlist) => (
        <article
          key={playlist.id}
          className="playlist-card"
          onClick={() => onOpenPlaylist(playlist.id)}
        >
          <div className="playlist-card-art">
            <QueueListIcon />
            <button
              type="button"
              className="floating-play"
              aria-label={`Reproducir ${playlist.name}`}
              onClick={(event) => {
                event.stopPropagation()
                onPlayPlaylist(playlist.id)
              }}
            >
              <PlayIcon />
            </button>
          </div>
          <div className="playlist-card-body">
            <h3>{playlist.name}</h3>
            <p>{playlist.song_ids.length} canciones</p>
          </div>
        </article>
      ))}
    </section>
  )
}
