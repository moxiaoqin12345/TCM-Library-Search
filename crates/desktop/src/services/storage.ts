export interface BookmarkItem {
  id: string;
  title: string;
  book: string;
  chapter?: string;
  note?: string;
  timestamp: number;
}

export interface RecentEntryItem {
  id: string;
  title: string;
  book: string;
  subcategory: string;
  timestamp: number;
}

const BOOKMARKS_STORAGE_KEY = "tcm_reader_bookmarks_v1";
const RECENT_ENTRIES_STORAGE_KEY = "tcm_reader_recent_entries_v1";

/**
 * 从本地存储加载所有书签
 */
export function loadBookmarks(): BookmarkItem[] {
  try {
    const raw = localStorage.getItem(BOOKMARKS_STORAGE_KEY);
    if (!raw) return [];
    return JSON.parse(raw);
  } catch (e) {
    console.warn("Failed to load bookmarks from storage:", e);
    return [];
  }
}

/**
 * 保存书签到本地存储
 */
export function saveBookmarks(bookmarks: BookmarkItem[]): void {
  try {
    localStorage.setItem(BOOKMARKS_STORAGE_KEY, JSON.stringify(bookmarks));
  } catch (e) {
    console.warn("Failed to save bookmarks to storage:", e);
  }
}

/**
 * 添加或更新书签
 */
export function setBookmark(
  id: string,
  title: string,
  book: string,
  chapter?: string,
  note?: string
): BookmarkItem[] {
  const current = loadBookmarks();
  const existingIdx = current.findIndex((b) => b.id === id);

  let updated: BookmarkItem[];
  if (existingIdx >= 0) {
    updated = current.map((b, idx) =>
      idx === existingIdx
        ? {
            ...b,
            title,
            book,
            chapter: chapter || b.chapter,
            note: note !== undefined ? note : b.note,
            timestamp: Date.now(),
          }
        : b
    );
  } else {
    updated = [
      {
        id,
        title,
        book,
        chapter,
        note,
        timestamp: Date.now(),
      },
      ...current,
    ];
  }

  saveBookmarks(updated);
  return updated;
}

/**
 * 移除书签
 */
export function removeBookmark(id: string): BookmarkItem[] {
  const current = loadBookmarks();
  const updated = current.filter((b) => b.id !== id);
  saveBookmarks(updated);
  return updated;
}

/**
 * 保存或更新书签批注/心得
 */
export function updateBookmarkNote(id: string, note: string): BookmarkItem[] {
  const current = loadBookmarks();
  const updated = current.map((b) =>
    b.id === id ? { ...b, note, timestamp: Date.now() } : b
  );
  saveBookmarks(updated);
  return updated;
}

/**
 * 从本地存储加载最近阅读历史
 */
export function loadRecentEntries(): RecentEntryItem[] {
  try {
    const raw = localStorage.getItem(RECENT_ENTRIES_STORAGE_KEY);
    if (!raw) return [];
    return JSON.parse(raw);
  } catch (e) {
    console.warn("Failed to load recent entries:", e);
    return [];
  }
}

/**
 * 记录一次阅读历史（去重，保留最新 30 条）
 */
export function pushRecentEntry(
  id: string,
  title: string,
  book: string,
  subcategory: string
): RecentEntryItem[] {
  try {
    const current = loadRecentEntries().filter((e) => e.id !== id);
    const updated = [
      {
        id,
        title,
        book,
        subcategory,
        timestamp: Date.now(),
      },
      ...current,
    ].slice(0, 30);
    localStorage.setItem(RECENT_ENTRIES_STORAGE_KEY, JSON.stringify(updated));
    return updated;
  } catch (e) {
    console.warn("Failed to push recent entry:", e);
    return [];
  }
}
