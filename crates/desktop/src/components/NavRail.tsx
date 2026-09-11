import { Component, For } from 'solid-js';
import styles from './NavRail.module.css';

export interface NavRailProps {
  activeNav: string;
  onNavChange: (navId: string) => void;
  onThemeToggle: () => void;
  currentTheme: string;
}

interface NavItem {
  id: string;
  label: string;
  icon: () => any;
}

const HomeIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
    <polyline points="9 22 9 12 15 12 15 22" />
  </svg>
);

const SearchIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <circle cx="11" cy="11" r="8" />
    <line x1="21" y1="21" x2="16.65" y2="16.65" />
  </svg>
);

const LibraryIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
  </svg>
);

const BookmarkIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />
  </svg>
);

const MeridianIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <circle cx="12" cy="5" r="2.5" />
    <circle cx="6" cy="12" r="2.5" />
    <circle cx="18" cy="12" r="2.5" />
    <circle cx="12" cy="19" r="2.5" />
    <path d="M12 7.5v9M8.2 13.5l7.6-3M8.2 10.5l7.6 3" stroke-dasharray="2 2" />
  </svg>
);

const SettingsIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <circle cx="12" cy="12" r="3" />
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
  </svg>
);

const ThemeIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <circle cx="12" cy="12" r="5" />
    <line x1="12" y1="1" x2="12" y2="3" />
    <line x1="12" y1="21" x2="12" y2="23" />
    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
    <line x1="1" y1="12" x2="3" y2="12" />
    <line x1="21" y1="12" x2="23" y2="12" />
    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
  </svg>
);

const GraphIcon = () => (
  <svg class={styles.icon} viewBox="0 0 24 24">
    <circle cx="6" cy="6" r="3" />
    <circle cx="18" cy="6" r="3" />
    <circle cx="12" cy="18" r="3" />
    <line x1="8.5" y1="7.5" x2="15.5" y2="7.5" />
    <line x1="7.5" y1="8.5" x2="10.5" y2="15.5" />
    <line x1="16.5" y1="8.5" x2="13.5" y2="15.5" />
  </svg>
);

export const NavRail: Component<NavRailProps> = (props) => {
  const topNavs: NavItem[] = [
    { id: 'home', label: '主页', icon: HomeIcon },
    { id: 'search', label: '检索', icon: SearchIcon },
    { id: 'library', label: '书库', icon: LibraryIcon },
    { id: 'meridian', label: '经穴', icon: MeridianIcon },
    { id: 'graph', label: '图谱', icon: GraphIcon },
    { id: 'bookmark', label: '收藏', icon: BookmarkIcon },
  ];

  return (
    <nav class={styles.navRail}>
      <div class={styles.brand}>📜</div>
      
      <div class={styles.navGroup}>
        <For each={topNavs}>
          {(nav) => (
            <button
              class={`${styles.navButton} ${props.activeNav === nav.id ? styles.active : ''}`}
              onClick={() => props.onNavChange(nav.id)}
            >
              <nav.icon />
              <span class={styles.label}>{nav.label}</span>
            </button>
          )}
        </For>
      </div>

      <div class={styles.spacer} />

      <div class={styles.navGroup}>
        <button class={styles.navButton} onClick={props.onThemeToggle}>
          <ThemeIcon />
          <span class={styles.label}>主题</span>
        </button>
        
        <button
          class={`${styles.navButton} ${props.activeNav === 'settings' ? styles.active : ''}`}
          onClick={() => props.onNavChange('settings')}
        >
          <SettingsIcon />
          <span class={styles.label}>设置</span>
        </button>
      </div>
    </nav>
  );
};
