import { Component, For } from 'solid-js';
import { CategoryTreeItem } from '../services/api';
import styles from './LibraryPanel.module.css';

export interface BookInfo {
  id: string;
  title: string;
  author: string;
  dynasty: string;
  description: string;
  category: string;
  entryCount: number;
  coverColor: string;
  coverText: string;
}

interface LibraryPanelProps {
  categories: CategoryTreeItem[];
  books: BookInfo[];
  selectedBookId: string;
  onSelectBook: (bookId: string) => void;
  onFilterCategory: (category: string) => void;
}

export const LibraryPanel: Component<LibraryPanelProps> = (props) => {
  return (
    <div class={styles.panel}>
      <div class={styles.header}>
        <h2 class={styles.title}>图书馆 · 按书查阅</h2>
        <select
          class={styles.filter}
          onChange={(e) => props.onFilterCategory(e.currentTarget.value)}
        >
          <option value="">全部典籍分类</option>
          <For each={props.categories}>
            {(category) => (
              <option value={category.id}>{category.name_zh}</option>
            )}
          </For>
        </select>
      </div>
      <div class={styles.bookList}>
        <For each={props.books}>
          {(book) => (
            <div
              class={`${styles.bookCard} ${props.selectedBookId === book.id ? styles.active : ''}`}
              onClick={() => props.onSelectBook(book.id)}
            >
              <div
                class={styles.cover}
                style={{ background: book.coverColor }}
              >
                {book.coverText}
              </div>
              <div class={styles.info}>
                <div class={styles.bookHeader}>
                  <h3 class={styles.bookTitle}>{book.title}</h3>
                  <span class={styles.entryCount}>{book.entryCount} 卷/篇</span>
                </div>
                <div class={styles.authorInfo}>
                  [{book.dynasty}] {book.author}
                </div>
                <p class={styles.description}>{book.description}</p>
                <div class={styles.categoryBadge}>{book.category}</div>
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  );
};
