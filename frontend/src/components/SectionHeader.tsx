import type { ReactNode } from 'react'

type SectionHeaderProps = {
  title: string
  subtitle: string
  stacked?: boolean
  action?: ReactNode
}

export function SectionHeader({
  title,
  subtitle,
  stacked = false,
  action,
}: SectionHeaderProps) {
  return (
    <section className={stacked ? 'section-header section-header--stacked' : 'section-header'}>
      <div>
        <p className="eyebrow">{title}</p>
        <h2>{subtitle}</h2>
      </div>
      {action}
    </section>
  )
}
