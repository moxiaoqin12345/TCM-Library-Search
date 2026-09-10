import { Component, For, Show, createSignal } from 'solid-js';
import { BookmarkItem } from '../services/storage';
import { exportAllBookmarksToMarkdown } from '../services/export';
import styles from './BookmarkPanel.module.css';

export interface BookmarkPanelProps {
  bookmarks: BookmarkItem[];
  onSelectEntry: (id: string) => void;
  onRemoveBookmark: (id: string) => void;
  onUpdateNote: (id: string, note: string) => void;
}

export const BookmarkPanel: Component<BookmarkPanelProps> = (props) => {
  // 正在编辑笔记的条目 ID
  const [editingId, setEditingId] = createSignal<string | null>(null);
  const [noteDraft, setNoteDraft] = createSignal<string>('');

  const handleStartEditNote = (e: MouseEvent, item: BookmarkItem) => {
    e.stopPropagation();
    setEditingId(item.id);
    setNoteDraft(item.note || '');
  };

  const handleSaveNote = (e: MouseEvent, id: string) => {
    e.stopPropagation();
    props.onUpdateNote(id, noteDraft().trim());
    setEditingId(null);
  };

  const handleCancelEdit = (e: MouseEvent) => {
    e.stopPropagation();
    setEditingId(null);
  };

  const formatDate = (timestamp: number) => {
    try {
      const d = new Date(timestamp);
      const m = d.getMonth() + 1;
      const day = d.getDate();
      return `${m}月${day}日`;
    } catch {
      return '';
    }
  };

  return (
    <div class={styles.container}>
      <div class={styles.header}>
        <h2 class={styles.title}>我的收藏与研读心得</h2>
        <div class={styles.headerActions}>
          <Show when={props.bookmarks.length > 0}>
            <button
              type="button"
              class={styles.exportBtn}
              onClick={() => exportAllBookmarksToMarkdown(props.bookmarks)}
              title="将所有收藏篇目与心得导出为 Markdown 集锦"
            >
              📥 导出集锦
            </button>
          </Show>
          <span class={styles.badge}>{props.bookmarks.length} 篇</span>
        </div>
      </div>

      <div class={styles.content}>
        <For
          each={props.bookmarks}
          fallback={
            <div class={styles.emptyState}>
              <div class={styles.emptyIcon}>🔖</div>
              <div class={styles.emptyTitle}>暂无收藏条目</div>
              <div class={styles.emptyDesc}>
                在右侧阅读典籍时，点击顶栏「☆ 收藏」即可沉淀心得并常温常新
              </div>
            </div>
          }
        >
          {(bookmark) => {
            const isEditing = () => editingId() === bookmark.id;
            return (
              <div
                class={styles.bookmarkCard}
                onClick={() => props.onSelectEntry(bookmark.id)}
              >
                <div class={styles.cardHeader}>
                  <h3 class={styles.entryTitle}>{bookmark.title}</h3>
                  <div class={styles.cardActions}>
                    <button
                      type="button"
                      class={styles.deleteBtn}
                      onClick={(e) => {
                        e.stopPropagation();
                        props.onRemoveBookmark(bookmark.id);
                      }}
                      title="取消收藏"
                    >
                      ✕
                    </button>
                  </div>
                </div>

                <div class={styles.entryMeta}>
                  <span class={styles.entryBook}>
                    《{bookmark.book}》{bookmark.chapter ? `· ${bookmark.chapter}` : ''}
                  </span>
                  <span class={styles.timeTag}>
                    {formatDate(bookmark.timestamp)}
                  </span>
                </div>

                {/* 研读批注与随记模块 */}
                <div class={styles.noteSection}>
                  <Show
                    when={isEditing()}
                    fallback={
                      <>
                        <Show when={bookmark.note}>
                          <div
                            class={styles.noteText}
                            onClick={(e) => handleStartEditNote(e, bookmark)}
                            title="点击修改批注心得"
                          >
                            ✍️ {bookmark.note}
                          </div>
                        </Show>
                        <Show when={!bookmark.note}>
                          <button
                            type="button"
                            class={styles.addNoteTrigger}
                            onClick={(e) => handleStartEditNote(e, bookmark)}
                          >
                            + 添加随记批注
                          </button>
                        </Show>
                      </>
                    }
                  >
                    <div
                      class={styles.noteInputRow}
                      onClick={(e) => e.stopPropagation()}
                    >
                      <input
                        type="text"
                        class={styles.noteInput}
                        placeholder="记录临床经验或辨证心得..."
                        value={noteDraft()}
                        onInput={(e) => setNoteDraft(e.currentTarget.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') handleSaveNote(e as any, bookmark.id);
                          if (e.key === 'Escape') handleCancelEdit(e as any);
                        }}
                        autofocus
                      />
                      <button
                        type="button"
                        class={styles.noteSaveBtn}
                        onClick={(e) => handleSaveNote(e, bookmark.id)}
                      >
                        保存
                      </button>
                      <button
                        type="button"
                        class={styles.deleteBtn}
                        onClick={handleCancelEdit}
                      >
                        取消
                      </button>
                    </div>
                  </Show>
                </div>
              </div>
            );
          }}
        </For>
      </div>
    </div>
  );
};
