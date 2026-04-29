import type { PlaylistSummaryDto } from '../types'

type PlaylistSummaryProps = {
  summary: PlaylistSummaryDto | null
}

export function PlaylistSummary({ summary }: PlaylistSummaryProps) {
  return (
    <div className="playlist-summary-grid">
      <article className="playlist-summary-card">
        <span className="eyebrow">Songs</span>
        <strong>{summary?.song_count ?? 0}</strong>
      </article>
      <article className="playlist-summary-card">
        <span className="eyebrow">Known Duration</span>
        <strong>{summary?.total_duration_seconds ?? 0}s</strong>
      </article>
      <article className="playlist-summary-card">
        <span className="eyebrow">Unknown Duration</span>
        <strong>{summary?.unknown_duration_count ?? 0}</strong>
      </article>
    </div>
  )
}
