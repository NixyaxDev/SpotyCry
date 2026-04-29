import { SignalIcon } from '../shared/icons'

export function TopBar() {
  return (
    <header className="topbar">
      <div className="topbar-copy">
        <SignalIcon />
        <p>Cola de reproducción y playlists respaldadas por el servidor</p>
      </div>
    </header>
  )
}
