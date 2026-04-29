import type { ComponentType, SVGProps } from 'react'
import type { Screen } from '../types/music'
import { MusicalNoteIcon, PlayCircleIcon, QueueListIcon } from '../shared/icons'

type SidebarProps = {
  screen: Screen
  onNavigate: (screen: Screen) => void
}

const navItems: Array<{
  icon: ComponentType<SVGProps<SVGSVGElement>>
  label: string
  screen: Screen
  matches: Screen[]
}> = [
  { icon: MusicalNoteIcon, label: 'Songs', screen: 'songs', matches: ['songs'] },
  {
    icon: QueueListIcon,
    label: 'Playlists',
    screen: 'playlists',
    matches: ['playlists', 'playlist-detail'],
  },
  {
    icon: PlayCircleIcon,
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
          <item.icon />
          <span>{item.label}</span>
        </button>
      ))}
      </nav>
    </aside>
  )
}
