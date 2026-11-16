export function getDeviceIconSvg(kind: string): string {
  const k = kind.toLowerCase();
  switch (k) {
    case 'climate':
      return `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <path d="M14 14.76V3.5a2 2 0 0 0-4 0v11.26a4.5 4.5 0 1 0 4 0Z"/>
        <circle cx="12" cy="17.5" r="1.5" fill="currentColor" stroke="none"/>
        <path d="M17 6h3M17 10h2"/>
      </svg>`;
    case 'lighting':
      return `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <circle cx="12" cy="12" r="4.25"/>
        <path d="M12 2.5v2.2M12 19.3v2.2M4.93 4.93l1.55 1.55M17.52 17.52l1.55 1.55M2.5 12h2.2M19.3 12h2.2M4.93 19.07l1.55-1.55M17.52 6.48l1.55-1.55"/>
      </svg>`;
    case 'switch':
      return `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <rect x="3" y="6" width="18" height="12" rx="6"/>
        <circle cx="15" cy="12" r="3.2" fill="currentColor" stroke="none"/>
      </svg>`;
    case 'security':
      return `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <rect x="4.5" y="10.5" width="15" height="10" rx="3"/>
        <path d="M8 10.5V7.5a4 4 0 0 1 8 0v3"/>
        <circle cx="12" cy="15.5" r="1.2" fill="currentColor" stroke="none"/>
      </svg>`;
    case 'energy':
      return `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <path d="M13 2 4.5 13.5H11L10 22l8.5-11.5H12L13 2Z"/>
      </svg>`;
    default:
      return `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <circle cx="12" cy="12" r="8"/>
        <circle cx="12" cy="12" r="2.5" fill="currentColor" stroke="none"/>
      </svg>`;
  }
}

export const TRASH_ICON_SVG = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
  <path d="M3 6h18"/>
  <path d="M8 6V4a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1v2"/>
  <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/>
</svg>`;

export const PLAY_ICON_SVG = `<svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M6 4.75a1.25 1.25 0 0 1 1.9-.1l11.2 7.25a1.25 1.25 0 0 1 0 2.1L7.9 21.25A1.25 1.25 0 0 1 6 20.2V4.75Z"/>
</svg>`;
