import { Component, For, Show, createSignal } from 'solid-js';
import { BookSummaryItem, CategoryTreeItem } from '../services/api';
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
  activeBookTree?: BookSummaryItem | null;
  selectedEntryId: string;
  onSelectBook: (bookId: string) => void;
  onSelectEntry: (entryId: string) => void;
  onBackToBooks: () => void;
  onFilterCategory: (category: string) => void;
}

export const LibraryPanel: Component<LibraryPanelProps> = (props) => {
  // 折叠展开的章节名称集合
  const [collapsedChapters, setCollapsedChapters] = createSignal<Record<string, boolean>>({});

  const toggleChapter = (chapterName: string) => {
    setCollapsedChapters((prev) => ({
      ...prev,
      [chapterName]: !prev[chapterName],
    }));
  };

  const isCollapsed = (chapterName: string) => !!collapsedChapters()[chapterName];

  return (
    <div class={styles.panel}>
      <Show
        when={props.activeBookTree}
        fallback={
          // 视图 1: 书目选择列表 (书架模式)
          <>
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
                    class={`${styles.bookCard} ${
                      props.selectedBookId === book.id ? styles.active : ''
                    }`}
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
          </>
        }
      >
        {/* 视图 2: 所选典籍的篇章章节树 (Chapter Tree 点读模式) */}
        <div class={styles.chapterTreeContainer}>
          <div class={styles.treeHeader}>
            <button
              type="button"
              class={styles.backBtn}
              onClick={props.onBackToBooks}
              title="返回书架列表"
            >
              ← 返回书架
            </button>
            <div class={styles.selectedBookMeta}>
              <h2 class={styles.treeBookTitle}>
                {props.activeBookTree!.book_name}
              </h2>
            </div>
            <div class={styles.treeBookStats}>
              共 {props.activeBookTree!.chapter_count} 卷/篇 · {props.activeBookTree!.entry_count} 条正文
            </div>
          </div>

          <div class={styles.chapterList}>
            <For each={props.activeBookTree!.chapters}>
              {(chapter) => {
                const collapsed = () => isCollapsed(chapter.chapter_name);
                return (
                  <div class={styles.chapterGroup}>
                    <div
                      class={styles.chapterHeader}
                      onClick={() => toggleChapter(chapter.chapter_name)}
                    >
                      <div class={styles.chapterTitleGroup}>
                        <span class={styles.chapterArrow}>
                          {collapsed() ? '▶' : '▼'}
                        </span>
                        <span class={styles.chapterName}>
                          {chapter.chapter_name}
                        </span>
                      </div>
                      <span class={styles.chapterCount}>
                        {chapter.count} 篇
                      </span>
                    </div>

                    <Show when={!collapsed()}>
                      <div class={styles.chapterEntries}>
                        <For each={chapter.entries}>
                          {(entry) => (
                            <div
                              class={`${styles.chapterEntryItem} ${
                                props.selectedEntryId === entry.id
                                  ? styles.chapterEntryItemActive
                                  : ''
                              }`}
                              onClick={() => props.onSelectEntry(entry.id)}
                            >
                              <span class={styles.entryItemTitle}>
                                {entry.section_title || entry.title}
                              </span>
                              <Show when={entry.subcategory}>
                                <span class={styles.entrySubTag}>
                                  {entry.subcategory}
                                </span>
                              </Show>
                            </div>
                          )}
                        </For>
                      </div>
                    </Show>
                  </div>
                );
              }}
            </For>
          </div>
        </div>
      </Show>
    </div>
  );
};
