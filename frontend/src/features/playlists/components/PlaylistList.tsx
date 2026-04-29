import type { PlaylistDto } from '../types'

const playlistCoverFallback =
  'https://images.unsplash.com/photo-1511379938547-c1f69419868d?auto=format&fit=crop&w=800&q=80'

type PlaylistListProps = {
  playlists: PlaylistDto[]
  onOpenPlaylist: (playlistId: string) => void
}

export function PlaylistList({
  playlists,
  onOpenPlaylist,
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
            <img src={playlistCoverFallback} alt={playlist.name} />
            <button type="button" className="floating-play" aria-label="Open playlist">
              <span className="material-symbols-outlined fillable">queue_music</span>
            </button>
          </div>
          <div className="playlist-card-body">
            <h3>{playlist.name}</h3>
            <p>{playlist.song_ids.length} songs</p>
          </div>
        </article>
      ))}
    </section>
  )
}
