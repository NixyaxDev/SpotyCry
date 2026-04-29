import { SignalIcon } from '../shared/icons'

export function TopBar() {
  return (
    <header className="topbar">
      <div className="topbar-copy">
        <SignalIcon />
        <p>Server-backed playback queue and playlists</p>
      </div>
    </header>
  )
}
