import { invoke } from "@tauri-apps/api/core";

export interface CorpusConditions {
  zhengxing: string[];
  zhifa: string[];
  bingzheng: string[];
  zhengzhuang: string[];
  fangming: string[];
  yaoming: string[];
  xuewei: string[];
  jingluo: string[];
  siqi: string[];
  wuwei: string[];
  guijing: string[];
  keywords: string[];
}

export interface CorpusImage {
  path: string;
  title: string;
  caption?: string;
  type: string;
  source?: string;
}

export interface CorpusMetadata {
  id: string;
  book: string;
  chapter: string;
  section_title: string;
  source_version: string;
  author: string;
  dynasty: string;
  type: string;
  conditions: CorpusConditions;
  weight: number;
  tags: string[];
  images: CorpusImage[];
}

export interface TcmEntryDetail {
  metadata: CorpusMetadata;
  original_text: string;
  commentary_text?: string;
  summary_text?: string;
  images: CorpusImage[];
}

export interface SearchResultItem {
  id: string;
  book: string;
  chapter: string;
  section_title: string;
  category: string;
  subcategory: string;
  item_type: string;
  weight: number;
  matched_dimensions: string[];
  snippet: string;
  relative_path: string;
}

export interface SubcategoryTreeItem {
  id: string;
  name_zh: string;
  count: number;
}

export interface CategoryTreeItem {
  id: string;
  name_zh: string;
  count: number;
  subcategories: SubcategoryTreeItem[];
}

export interface CorpusStatus {
  total_entries: number;
  total_categories: number;
  is_ready: boolean;
  root_path: string;
}

export interface SearchQuery {
  keyword?: string;
  category?: string;
  subcategory?: string;
  zhengxing?: string;
  zhifa?: string;
  bingzheng?: string;
  zhengzhuang?: string;
  fangming?: string;
  yaoming?: string;
  xuewei?: string;
  jingluo?: string;
  siqi?: string;
  wuwei?: string;
  guijing?: string;
  limit?: number;
  offset?: number;
}

// IPC Wrappers
export async function checkHealth(): Promise<string> {
  return invoke("check_health");
}

export async function appWindowMinimize(): Promise<void> {
  return invoke("app_window_minimize");
}

export async function appWindowToggleMaximize(): Promise<boolean> {
  return invoke("app_window_toggle_maximize");
}

export async function appWindowIsMaximized(): Promise<boolean> {
  return invoke("app_window_is_maximized");
}

export async function appWindowClose(): Promise<void> {
  return invoke("app_window_close");
}

export async function appWindowStartDragging(): Promise<void> {
  return invoke("app_window_start_dragging");
}

export async function getCorpusStatus(): Promise<CorpusStatus> {
  return invoke("get_corpus_status");
}

export async function listCategories(): Promise<CategoryTreeItem[]> {
  return invoke("list_categories");
}

export async function searchEntries(query: SearchQuery): Promise<SearchResultItem[]> {
  return invoke("search_entries", { query });
}

export async function getEntryDetail(id: string): Promise<TcmEntryDetail> {
  return invoke("get_entry_detail", { id });
}

export interface BookChapterEntryItem {
  id: string;
  title: string;
  section_title: string;
  weight: number;
  category: string;
  subcategory: string;
}

export interface BookChapterTreeItem {
  chapter_name: string;
  count: number;
  entries: BookChapterEntryItem[];
}

export interface BookSummaryItem {
  book_name: string;
  entry_count: number;
  chapter_count: number;
  chapters: BookChapterTreeItem[];
}

export async function listBooks(): Promise<BookSummaryItem[]> {
  return invoke("list_books");
}

export async function getBookChapters(book: string): Promise<BookSummaryItem> {
  return invoke("get_book_chapters", { book });
}

export async function resolveImagePath(id: string, relativePath: string): Promise<string> {
  return invoke("resolve_image_path", { id, relativePath });
}

export async function loadImageDataUri(id: string, relativePath: string): Promise<string> {
  return invoke("load_image_data_uri", { id, relativePath });
}

export type IncompatibilitySeverity = "severe" | "warning" | "info";

export type IncompatibilityType =
  | "shiba_fan"
  | "shijiu_wei"
  | "renshen_jinji"
  | "xiang_e";

export interface CompatibilityAlert {
  herb_a: string;
  herb_b?: string | null;
  incompatibility_type: IncompatibilityType;
  severity: IncompatibilitySeverity;
  source_rhyme: string;
  explanation: string;
}

export async function checkHerbCompatibility(
  herbs: string[]
): Promise<CompatibilityAlert[]> {
  return invoke("check_compatibility", { herbs });
}

export type DiffOp = "equal" | "delete" | "insert";

export interface DiffChunk {
  op: DiffOp;
  text: string;
}

export interface TextDiffResult {
  chunks: DiffChunk[];
  text_a: string;
  text_b: string;
  similarity: number;
  equal_chars: number;
  deleted_chars: number;
  inserted_chars: number;
}

export async function diffTextVersions(
  textA: string,
  textB: string
): Promise<TextDiffResult> {
  return invoke("diff_text_versions", { textA, textB });
}
