import { SectionHeader } from '../components/SectionHeader'
import type { Playlist } from '../types/music'

type PlaylistsViewProps = {
  playlists: Playlist[]
  onOpenPlaylist: () => void
}

export function PlaylistsView({
  playlists,
  onOpenPlaylist,
}: PlaylistsViewProps) {
  return (
    <>
      <SectionHeader
        title="Your Playlists"
        subtitle="Curated sanctuaries for every mood."
        stacked
      />

      <section className="playlist-grid">
        <button type="button" className="create-playlist-card">
          <span className="material-symbols-outlined">add</span>
          <span>Create New Playlist</span>
        </button>

        {playlists.map((playlist) => (
          <article
            key={playlist.id}
            className="playlist-card"
            onClick={onOpenPlaylist}
          >
            <div className="playlist-card-art">
              <img src={playlist.cover} alt={playlist.name} />
              <button type="button" className="floating-play" aria-label="Play playlist">
                <span className="material-symbols-outlined fillable">play_arrow</span>
              </button>
            </div>
            <div className="playlist-card-body">
              <h3>{playlist.name}</h3>
              <p>{playlist.tracks} songs</p>
            </div>
          </article>
        ))}
      </section>
    </>
  )
}
