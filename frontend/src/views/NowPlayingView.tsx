import type { Song } from '../types/music'

type NowPlayingViewProps = {
  selectedSong: Song
  upNext: Song[]
}

const heroCover =
  'https://lh3.googleusercontent.com/aida-public/AB6AXuBBxCSvndJYAv_cC0Ogi5-WgubOLHGcLag5nrT4JLtLrAqPbzFHjPADOTzCc5fqz3jRyDJrFm2veYIYlcepk_GJu5LxfxJhdBGjQbemOYfYwcraOA8ro7G5-Vui0XH7_vVrnpYzcDH7ze32-SQJY4x4cBQ9TCuaLCz9ZRhOLBXuxDXvgzm2cjoEHurDmhI8pzkaw01UwyEQMmNJTGQ4tYjZCf1HBC_F4jK1oaxhqXEBN5WAqrvrnFijK7y2XuCbP5pBSxSUdk9obhY'

export function NowPlayingView({
  selectedSong,
  upNext,
}: NowPlayingViewProps) {
  return (
    <section className="now-playing-layout">
      <div className="now-playing-main">
        <div className="now-playing-art">
          <img src={heroCover} alt="Echoes in the Rain" />
        </div>
        <div className="now-playing-copy">
          <h2>Echoes in the Rain</h2>
          <p>{selectedSong.artist}</p>
        </div>
        <div className="now-playing-controls">
          <button type="button" className="ghost-icon">
            <span className="material-symbols-outlined">shuffle</span>
          </button>
          <button type="button" className="ghost-icon">
            <span className="material-symbols-outlined fillable">skip_previous</span>
          </button>
          <button type="button" className="play-button-xl">
            <span className="material-symbols-outlined fillable">pause</span>
          </button>
          <button type="button" className="ghost-icon">
            <span className="material-symbols-outlined fillable">skip_next</span>
          </button>
          <button type="button" className="ghost-icon">
            <span className="material-symbols-outlined">repeat</span>
          </button>
        </div>
      </div>

      <aside className="up-next-panel">
        <div className="panel-title-row">
          <h3>Up Next</h3>
          <span className="material-symbols-outlined">queue</span>
        </div>
        <div className="up-next-list">
          {upNext.map((song) => (
            <article key={song.id} className="up-next-item">
              <img src={song.cover} alt={song.title} />
              <div>
                <strong>{song.title}</strong>
                <span>{song.artist}</span>
              </div>
              <time>{song.duration}</time>
            </article>
          ))}
        </div>
      </aside>
    </section>
  )
}
