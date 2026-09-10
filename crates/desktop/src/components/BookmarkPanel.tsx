import { Component, For } from 'solid-js';
import styles from './BookmarkPanel.module.css';

export interface BookmarkPanelProps {
  bookmarks: Array<{ id: string; title: string; book: string }>;
  onSelectEntry: (id: string) => void;
}

export const BookmarkPanel: Component<BookmarkPanelProps> = (props) => {
  return (
    <div class={styles.container}>
      <h1 class={styles.title}>我的收藏</h1>
      
      <div class={styles.content}>
        <For each={props.bookmarks} fallback={
          <div class={styles.emptyState}>
            <div class={styles.emptyIcon}>🔖</div>
            <div class={styles.emptyTitle}>暂无收藏条目</div>
            <div class={styles.emptyDesc}>阅读时点击 ⭐ 即可收藏</div>
          </div>
        }>
          {(bookmark) => (
            <button class={styles.bookmarkCard} onClick={() => props.onSelectEntry(bookmark.id)}>
              <div class={styles.cardHeader}>
                <h3 class={styles.entryTitle}>{bookmark.title}</h3>
                <span class={styles.starIcon}>⭐</span>
              </div>
              <div class={styles.entryMeta}>
                <span class={styles.entryBook}>来源：{bookmark.book}</span>
              </div>
            </button>
          )}
        </For>
      </div>
    </div>
  );
};
