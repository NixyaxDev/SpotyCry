import type { Song } from '../types/music'

type PlayerBarProps = {
  song: Song
}

export function PlayerBar({ song }: PlayerBarProps) {
  return (
    <footer className="player-bar">
      <div className="player-now">
        <img src={song.cover} alt={song.title} />
        <div>
          <strong>{song.title}</strong>
          <span>{song.artist}</span>
        </div>
      </div>

      <div className="player-center">
        <div className="player-actions">
          <button type="button">
            <span className="material-symbols-outlined">shuffle</span>
          </button>
          <button type="button">
            <span className="material-symbols-outlined fillable">skip_previous</span>
          </button>
          <button type="button" className="player-main-button">
            <span className="material-symbols-outlined fillable">play_arrow</span>
          </button>
          <button type="button">
            <span className="material-symbols-outlined fillable">skip_next</span>
          </button>
          <button type="button">
            <span className="material-symbols-outlined">repeat</span>
          </button>
        </div>
        <div className="progress-row">
          <span>1:24</span>
          <div className="progress-track">
            <div className="progress-fill" />
          </div>
          <span>4:15</span>
        </div>
      </div>

      <div className="player-side">
        <button type="button">
          <span className="material-symbols-outlined">favorite</span>
        </button>
        <button type="button">
          <span className="material-symbols-outlined">volume_up</span>
        </button>
      </div>
    </footer>
  )
}
