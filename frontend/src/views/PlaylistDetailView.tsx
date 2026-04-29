import type { Song } from '../types/music'

type PlaylistDetailViewProps = {
  songs: Song[]
  selectedSong: Song | null
}

const playlistCover =
  'https://lh3.googleusercontent.com/aida-public/AB6AXuAyrc6FpM7hRBM0d12cHAPehjZD7sSeArtp7vFA-iW9lpnOMPTrhrvcEnQfff0uYSkf8FY7VgowcTZE_oFt-fZ0F_MjwQJpRWq6YXmxvB0kKCsBEB0uDeq-w4OAsU444teqad-lmqA8uyiLLc_y1f8CYmbG3YJs7TFVorhjHxNG8usrAlnc6hFFZkF3XbyWlVTwMlJ_1Dutjek8H9MV9QQxOjKm4gV6qTt8Nk8Z5YeBGBOLUf8i9e2oYDSFzS999bNPWHBHlaAON04'

export function PlaylistDetailView({
  songs,
  selectedSong,
}: PlaylistDetailViewProps) {
  return (
    <>
      <section className="playlist-hero">
        <img className="playlist-hero-cover" src={playlistCover} alt="Late Night Vibes" />
        <div className="playlist-hero-copy">
          <p className="eyebrow">Public Playlist</p>
          <h2>Late Night Vibes</h2>
          <p className="playlist-description">
            A curated selection of melancholic downtempo tracks for introspective
            midnight drives and quiet moments of reflection.
          </p>
          <div className="playlist-meta">
            <span>24k Saves</span>
            <span>•</span>
            <span>42 Songs, 2 hr 35 min</span>
          </div>
          <div className="playlist-actions">
            <button type="button" className="play-button-large">
              <span className="material-symbols-outlined fillable">play_arrow</span>
            </button>
            <button type="button" className="icon-button">
              <span className="material-symbols-outlined">favorite_border</span>
            </button>
            <button type="button" className="icon-button">
              <span className="material-symbols-outlined">more_horiz</span>
            </button>
          </div>
        </div>
      </section>

      <section className="panel">
        <div className="track-list-header">
          <span>#</span>
          <span>Title</span>
          <span>Album</span>
          <span>
            <span className="material-symbols-outlined">schedule</span>
          </span>
        </div>
        <div className="track-list">
          {songs.length > 0 ? (
            songs.map((song, index) => (
              <article
                key={song.id}
                className={song.id === selectedSong?.id ? 'track-row is-playing' : 'track-row'}
              >
                <div className="track-index">
                  {song.id === selectedSong?.id ? (
                    <span className="material-symbols-outlined">equalizer</span>
                  ) : (
                    index + 1
                  )}
                </div>
                <div className="track-title">
                  <img src={song.cover} alt={song.title} />
                  <div>
                    <strong>{song.title}</strong>
                    <span>{song.artist}</span>
                  </div>
                </div>
                <div className="track-album">{song.album}</div>
                <div className="track-duration">{song.duration}</div>
              </article>
            ))
          ) : (
            <div className="feedback-card">
              <p>No songs available in the server catalog</p>
            </div>
          )}
        </div>
      </section>
    </>
  )
}
