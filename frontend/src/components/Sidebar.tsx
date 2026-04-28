import type { Screen } from '../types/music'

type SidebarProps = {
  screen: Screen
  onNavigate: (screen: Screen) => void
}

const navItems: Array<{
  icon: string
  label: string
  screen: Screen
  matches: Screen[]
}> = [
  { icon: 'music_note', label: 'Songs', screen: 'songs', matches: ['songs'] },
  {
    icon: 'queue_music',
    label: 'Playlists',
    screen: 'playlists',
    matches: ['playlists', 'playlist-detail'],
  },
  {
    icon: 'play_circle',
    label: 'Now Playing',
    screen: 'now-playing',
    matches: ['now-playing'],
  },
]

export function Sidebar({ screen, onNavigate }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="brand-block">
        <h1>SpotiCry</h1>
        <p>Emotional Sanctuary</p>
      </div>

      <nav className="sidebar-nav" aria-label="Primary">
        {navItems.map((item) => (
          <button
            key={item.screen}
            type="button"
            className={item.matches.includes(screen) ? 'nav-item active' : 'nav-item'}
            onClick={() => onNavigate(item.screen)}
          >
            <span className="material-symbols-outlined fillable">{item.icon}</span>
            <span>{item.label}</span>
          </button>
        ))}
      </nav>
    </aside>
  )
}
