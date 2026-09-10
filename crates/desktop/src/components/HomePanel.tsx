import { Component, For } from 'solid-js';
import styles from './HomePanel.module.css';

export interface HomePanelProps {
  totalEntries: number;
  totalCategories: number;
  totalBooks: number;
  recentEntries: Array<{ id: string; title: string; book: string; subcategory: string }>;
  onSelectEntry: (id: string) => void;
  onNavigate: (navId: string) => void;
}

export const HomePanel: Component<HomePanelProps> = (props) => {
  return (
    <div class={styles.container}>
      <div class={styles.hero}>
        <h1 class={styles.title}>欢迎使用中医典籍文库</h1>
        <p class={styles.subtitle}>
          集收典籍 {props.totalBooks} 篇 · 覆盖 {props.totalCategories} 大分类 · 47 子类
        </p>
      </div>

      <div class={styles.statsGrid}>
        <div class={styles.statCard}>
          <div class={styles.statIcon}>📚</div>
          <div class={styles.statValue}>{props.totalBooks}</div>
          <div class={styles.statLabel}>书目</div>
        </div>
        <div class={styles.statCard}>
          <div class={styles.statIcon}>📋</div>
          <div class={styles.statValue}>{props.totalEntries}</div>
          <div class={styles.statLabel}>条目</div>
        </div>
        <div class={styles.statCard}>
          <div class={styles.statIcon}>🏷️</div>
          <div class={styles.statValue}>{props.totalCategories}</div>
          <div class={styles.statLabel}>分类</div>
        </div>
        <div class={styles.statCard}>
          <div class={styles.statIcon}>🔬</div>
          <div class={styles.statValue}>47</div>
          <div class={styles.statLabel}>维度</div>
        </div>
      </div>

      <div class={styles.recentSection}>
        <h2 class={styles.sectionTitle}>最近查阅</h2>
        <div class={styles.recentGrid}>
          <For each={props.recentEntries} fallback={<p class={styles.emptyText}>暂无最近查阅记录</p>}>
            {(entry) => (
              <button class={styles.recentCard} onClick={() => props.onSelectEntry(entry.id)}>
                <h3 class={styles.entryTitle}>{entry.title}</h3>
                <div class={styles.entryMeta}>
                  <span class={styles.entryBook}>{entry.book}</span>
                  <span class={styles.entryCategory}>{entry.subcategory}</span>
                </div>
              </button>
            )}
          </For>
        </div>
      </div>
    </div>
  );
};
