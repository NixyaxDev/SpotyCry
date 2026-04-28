export function TopBar() {
  return (
    <header className="topbar">
      <div className="search-shell">
        <span className="material-symbols-outlined">search</span>
        <input placeholder="Search your feelings..." />
      </div>
      <div className="topbar-actions">
        <button type="button" aria-label="Notifications">
          <span className="material-symbols-outlined">notifications</span>
        </button>
        <button type="button" aria-label="Settings">
          <span className="material-symbols-outlined">settings</span>
        </button>
        <button type="button" aria-label="Account">
          <span className="material-symbols-outlined">account_circle</span>
        </button>
      </div>
    </header>
  )
}
