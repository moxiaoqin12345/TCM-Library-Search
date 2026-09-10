import { TcmEntryDetail } from "./api";
import { BookmarkItem, loadBookmarks, saveBookmarks } from "./storage";

export interface BackupData {
  version: string;
  exportTime: string;
  appName: string;
  bookmarks: BookmarkItem[];
}

/**
 * 触发浏览器/WebView本地文件下载
 */
function downloadFile(filename: string, content: string, mimeType: string) {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/**
 * 将当前典籍条目导出为标准古籍格式的 Markdown 文档
 */
export function exportEntryToMarkdown(entry: TcmEntryDetail, bookmarkNote?: string): void {
  const meta = entry.metadata;
  const lines: string[] = [];

  lines.push(`---`);
  lines.push(`title: "${meta.section_title}"`);
  lines.push(`book: "${meta.book}"`);
  lines.push(`chapter: "${meta.chapter || ""}"`);
  lines.push(`author: "${meta.author || "先贤/古本"}"`);
  lines.push(`dynasty: "${meta.dynasty || "历代"}"`);
  lines.push(`source_version: "${meta.source_version || "经典古传"}"`);
  lines.push(`export_date: "${new Date().toISOString()}"`);
  lines.push(`---\n`);

  lines.push(`# 《${meta.book}》· ${meta.section_title}\n`);
  lines.push(`> 卷次：${meta.chapter || "篇目"} | 作者：[${meta.dynasty || "历代"}] ${meta.author || "先贤/古本"}\n`);

  if (bookmarkNote && bookmarkNote.trim()) {
    lines.push(`## 📌 研读心得与临床随记\n`);
    lines.push(`> ${bookmarkNote.trim()}\n`);
  }

  lines.push(`## 【 原文典经 】\n`);
  lines.push(`${entry.original_text}\n`);

  if (entry.commentary_text) {
    lines.push(`## 【 先贤阐微 · 古注评析 】\n`);
    lines.push(`${entry.commentary_text}\n`);
  }

  if (entry.summary_text) {
    lines.push(`## 【 白话提要 · 理法方药精解 】\n`);
    lines.push(`${entry.summary_text}\n`);
  }

  const filename = `${meta.book}_${meta.section_title}_研读文档.md`.replace(/[/\\?%*:|"<>]/g, "_");
  downloadFile(filename, lines.join("\n"), "text/markdown;charset=utf-8");
}

/**
 * 将所有收藏条目与研读心得批注导出为结构化 Markdown 研读集锦
 */
export function exportAllBookmarksToMarkdown(bookmarks: BookmarkItem[]): void {
  if (bookmarks.length === 0) return;

  const lines: string[] = [];
  lines.push(`# 中医典籍研读集锦与临床心得批注\n`);
  lines.push(`> 导出时间：${new Date().toLocaleString()} | 共收录 ${bookmarks.length} 篇\n`);
  lines.push(`---\n`);

  bookmarks.forEach((b, idx) => {
    lines.push(`### ${idx + 1}. ${b.title}`);
    lines.push(`- **典籍出处**：《${b.book}》${b.chapter ? `· ${b.chapter}` : ""}`);
    lines.push(`- **收藏日期**：${new Date(b.timestamp).toLocaleDateString()}`);
    if (b.note && b.note.trim()) {
      lines.push(`- **✍️ 临床心得**：\n> ${b.note.trim()}`);
    } else {
      lines.push(`- **✍️ 临床心得**：*(暂无随记)*`);
    }
    lines.push(`\n`);
  });

  const filename = `中医典籍文库_收藏与心得批注集锦_${new Date().toISOString().slice(0, 10)}.md`;
  downloadFile(filename, lines.join("\n"), "text/markdown;charset=utf-8");
}

/**
 * 将全部研读数据导出为 JSON 备份文件
 */
export function exportBackupJSON(): void {
  const bookmarks = loadBookmarks();
  const backup: BackupData = {
    version: "1.0",
    exportTime: new Date().toISOString(),
    appName: "TCM-Library-Search",
    bookmarks,
  };

  const filename = `tcm_library_backup_${new Date().toISOString().slice(0, 10)}.json`;
  downloadFile(filename, JSON.stringify(backup, null, 2), "application/json;charset=utf-8");
}

/**
 * 从上传的 JSON 文件解析并恢复书签与心得
 */
export async function importBackupJSON(file: File): Promise<{ success: boolean; count: number; error?: string }> {
  try {
    const text = await file.text();
    const data = JSON.parse(text) as BackupData;
    if (!data.bookmarks || !Array.isArray(data.bookmarks)) {
      return { success: false, count: 0, error: "备份文件格式不正确：缺少 bookmarks 列表" };
    }

    const existing = loadBookmarks();
    const mergedMap = new Map<string, BookmarkItem>();

    // 先存已有
    existing.forEach((b) => mergedMap.set(b.id, b));

    // 合并导入（若有则更新心得与时间戳）
    data.bookmarks.forEach((b) => {
      if (b && b.id && b.title) {
        const cur = mergedMap.get(b.id);
        if (cur) {
          mergedMap.set(b.id, {
            ...cur,
            note: b.note !== undefined ? b.note : cur.note,
            timestamp: Math.max(cur.timestamp, b.timestamp || 0),
          });
        } else {
          mergedMap.set(b.id, {
            id: b.id,
            title: b.title,
            book: b.book || "未知典籍",
            chapter: b.chapter,
            note: b.note,
            timestamp: b.timestamp || Date.now(),
          });
        }
      }
    });

    const mergedList = Array.from(mergedMap.values());
    saveBookmarks(mergedList);
    return { success: true, count: data.bookmarks.length };
  } catch (e: any) {
    return { success: false, count: 0, error: e?.message || "解析备份文件失败" };
  }
}
