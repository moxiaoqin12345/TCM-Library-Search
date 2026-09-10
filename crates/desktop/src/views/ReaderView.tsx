import {
  createSignal,
  For,
  Show,
  onMount,
} from "solid-js";
import {
  BookSummaryItem,
  CategoryTreeItem,
  getBookChapters,
  getEntryDetail,
  listBooks,
  listCategories,
  loadImageDataUri,
  searchEntries,
  SearchResultItem,
  TcmEntryDetail,
} from "../services/api";
import {
  currentFontFamily,
  currentFontSize,
  currentLineHeight,
  currentTheme,
  setCurrentFontFamily,
  setCurrentFontSize,
  setCurrentLineHeight,
  setCurrentTheme,
  toggleTheme,
} from "../theme/theme";
import Lightbox from "../components/Lightbox";
import { NavRail } from "../components/NavRail";
import { SearchPanel } from "../components/SearchPanel";
import { LibraryPanel, BookInfo } from "../components/LibraryPanel";
import { HomePanel } from "../components/HomePanel";
import { BookmarkPanel } from "../components/BookmarkPanel";
import { SettingsPanel } from "../components/SettingsPanel";
import styles from "./ReaderView.module.css";

interface ReaderViewProps {
  onTotalCountChange?: (count: number) => void;
}

export default function ReaderView(props: ReaderViewProps) {
  // 导航状态
  const [activeNav, setActiveNav] = createSignal<string>("search");

  // 典籍与检索数据状态
  const [categories, setCategories] = createSignal<CategoryTreeItem[]>([]);
  const [items, setItems] = createSignal<SearchResultItem[]>([]);
  const [selectedId, setSelectedId] = createSignal<string>("");
  const [activeEntry, setActiveEntry] = createSignal<TcmEntryDetail | null>(null);
  const [isLoading, setIsLoading] = createSignal<boolean>(false);
  const [isReadingLoading, setIsReadingLoading] = createSignal<boolean>(false);
  const [totalCount, setTotalCount] = createSignal<number>(0);

  // 阅读层显示控制（原文 / 古注 / 白话）
  const [showOriginal, setShowOriginal] = createSignal<boolean>(true);
  const [showCommentary, setShowCommentary] = createSignal<boolean>(true);
  const [showSummary, setShowSummary] = createSignal<boolean>(true);

  // 收藏与历史记录
  const [bookmarks, setBookmarks] = createSignal<Array<{ id: string; title: string; book: string }>>([]);
  const [recentEntries, setRecentEntries] = createSignal<
    Array<{ id: string; title: string; book: string; subcategory: string }>
  >([]);

  // 图书馆书目数据（基于典籍体系）
  const [books, setBooks] = createSignal<BookInfo[]>([
    {
      id: "yaodian_2025",
      title: "中国药典（2025年版）",
      author: "国家药典委员会",
      dynasty: "现代",
      description: "国家药品法定标准，收录常用中药材及中药饮片、成方制剂与单味制剂，具备权威性与临床指导价值。",
      category: "中药学",
      entryCount: 380,
      coverColor: "linear-gradient(135deg, #8b2515, #6b1b0f)",
      coverText: "药典",
    },
    {
      id: "shanghanlun",
      title: "伤寒论",
      author: "张仲景",
      dynasty: "东汉",
      description: "中医临床辨证论治奠基之作，创立六经辨证体系与经方配伍法则，被奉为医门之圭臬。",
      category: "方剂学",
      entryCount: 1,
      coverColor: "linear-gradient(135deg, #4a7065, #3d5c53)",
      coverText: "伤寒",
    },
    {
      id: "neijing_suwen",
      title: "黄帝内经·素问",
      author: "先贤遗著",
      dynasty: "先秦至汉",
      description: "中医学理论思想渊薮，详述阴阳五行、脏腑经络、养生摄生与天人相应理论根基。",
      category: "经典医籍",
      entryCount: 1,
      coverColor: "linear-gradient(135deg, #8b6914, #6b4f10)",
      coverText: "内经",
    },
    {
      id: "zhenjiuxue",
      title: "针灸学·经穴总览",
      author: "历代经穴集成",
      dynasty: "经典合编",
      description: "阐发十二经脉、奇经八脉走行动向与常用特定腧穴定位主治，融汇针石灸法精义。",
      category: "针灸推拿",
      entryCount: 1,
      coverColor: "linear-gradient(135deg, #357a8a, #285f6b)",
      coverText: "针灸",
    },
  ]);
  const [selectedBookId, setSelectedBookId] = createSignal<string>("yaodian_2025");
  // 当前正在点读的书籍章节树
  const [activeBookTree, setActiveBookTree] = createSignal<BookSummaryItem | null>(null);

  // 插图图片缓存与灯箱
  const [imageMap, setImageMap] = createSignal<Record<string, string>>({});
  const [lightboxImage, setLightboxImage] = createSignal<{
    src: string;
    title: string;
    caption?: string;
  } | null>(null);

  // 初始化加载分类、首批条目与动态典籍列表
  onMount(async () => {
    try {
      const cats = await listCategories();
      setCategories(cats);
      let total = 0;
      cats.forEach((c) => (total += c.count));
      setTotalCount(total);
      if (props.onTotalCountChange) {
        props.onTotalCountChange(total);
      }
    } catch (e) {
      console.warn("Failed to load categories:", e);
    }

    // 加载书目统计
    try {
      const rawBooks = await listBooks();
      if (rawBooks.length > 0) {
        const enrichedBooks: BookInfo[] = rawBooks.map((b) => {
          let author = "历代医家";
          let dynasty = "经典流传";
          let coverColor = "linear-gradient(135deg, #4a7065, #3d5c53)";
          let coverText = b.book_name.slice(0, 2);
          let description = `收录 ${b.chapter_count} 篇卷，共 ${b.entry_count} 条经文`;
          let category = "经典医籍";

          if (b.book_name.includes("药典")) {
            author = "国家药典委员会";
            dynasty = "现代";
            coverColor = "linear-gradient(135deg, #8b2515, #6b1b0f)";
            coverText = "药典";
            description = "国家法定标准，收录常用中药材饮片与制剂规范。";
            category = "中药学";
          } else if (b.book_name.includes("伤寒")) {
            author = "张仲景";
            dynasty = "东汉";
            coverColor = "linear-gradient(135deg, #8b6914, #6b4f10)";
            coverText = "伤寒";
            description = "六经辨证之祖，奠定理法方药体系。";
            category = "方剂学";
          } else if (b.book_name.includes("素问") || b.book_name.includes("内经")) {
            author = "先秦古圣";
            dynasty = "先秦至汉";
            coverColor = "linear-gradient(135deg, #357a8a, #285f6b)";
            coverText = "内经";
            description = "中医理论大宗，探寻阴阳天地造化。";
            category = "经典医籍";
          } else if (b.book_name.includes("针灸")) {
            author = "历代经穴集成";
            dynasty = "经典合编";
            coverColor = "linear-gradient(135deg, #4a7065, #3d5c53)";
            coverText = "针灸";
            description = "经络走向与要穴经注要籍。";
            category = "针灸推拿";
          }

          return {
            id: b.book_name,
            title: b.book_name,
            author,
            dynasty,
            description,
            category,
            entryCount: b.entry_count,
            coverColor,
            coverText,
          };
        });
        setBooks(enrichedBooks);
      }
    } catch (e) {
      console.warn("Failed to load books dynamically:", e);
    }

    // 首次自动检索
    doSearch("", undefined, undefined);
  });

  // 执行搜索
  const doSearch = async (keyword: string, category?: string, subcategory?: string) => {
    setIsLoading(true);
    try {
      const kw = keyword.trim() || undefined;
      const res = await searchEntries({
        category,
        subcategory,
        keyword: kw,
        limit: 150,
      });
      setItems(res);
      if (res.length > 0 && (!selectedId() || !res.some((r) => r.id === selectedId()))) {
        selectEntry(res[0].id);
      }
    } catch (err) {
      console.error("Search failed:", err);
    } finally {
      setIsLoading(false);
    }
  };

  // 选择条目并加载详情
  const selectEntry = async (id: string) => {
    setSelectedId(id);
    setIsReadingLoading(true);
    try {
      const detail = await getEntryDetail(id);
      setActiveEntry(detail);

      // 更新最近查阅列表（去重保留前 10 个）
      setRecentEntries((prev) => {
        const filtered = prev.filter((e) => e.id !== id);
        return [
          {
            id,
            title: detail.metadata.section_title,
            book: detail.metadata.book,
            subcategory: detail.metadata.chapter || detail.metadata.type,
          },
          ...filtered,
        ].slice(0, 10);
      });

      // 加载条目插图
      if (detail.images && detail.images.length > 0) {
        for (const img of detail.images) {
          if (img.path.startsWith("http")) {
            setImageMap((prev) => ({ ...prev, [img.path]: img.path }));
          } else {
            loadImageDataUri(id, img.path)
              .then((dataUri) => {
                setImageMap((prev) => ({ ...prev, [img.path]: dataUri }));
              })
              .catch(() => {});
          }
        }
      }
    } catch (err) {
      console.error("Failed to load detail:", err);
    } finally {
      setIsReadingLoading(false);
    }
  };

  // 收藏切换
  const toggleBookmark = () => {
    const cur = activeEntry();
    if (!cur) return;
    const id = cur.metadata.id;
    setBookmarks((prev) => {
      const exists = prev.some((b) => b.id === id);
      if (exists) {
        return prev.filter((b) => b.id !== id);
      } else {
        return [
          ...prev,
          {
            id,
            title: cur.metadata.section_title,
            book: cur.metadata.book,
          },
        ];
      }
    });
  };

  const isBookmarked = () => {
    const cur = activeEntry();
    if (!cur) return false;
    return bookmarks().some((b) => b.id === cur.metadata.id);
  };

  // 按书籍展开篇章目录树
  const handleSelectBook = async (bookId: string) => {
    setSelectedBookId(bookId);
    try {
      const tree = await getBookChapters(bookId);
      setActiveBookTree(tree);
      // 若有条目，自动预选首条正文阅读
      if (tree.chapters.length > 0 && tree.chapters[0].entries.length > 0) {
        selectEntry(tree.chapters[0].entries[0].id);
      }
    } catch (e) {
      console.warn("Failed to load book chapters:", e);
      // 容错降级触发分类搜索
      doSearch(bookId, undefined, undefined);
    }
  };

  const handleBackToBooks = () => {
    setActiveBookTree(null);
  };

  const openLightbox = (src: string, title: string, caption?: string) => {
    setLightboxImage({ src, title, caption });
  };

  return (
    <div class={styles.layoutContainer}>
      {/* 1. 左侧图标导航栏 */}
      <NavRail
        activeNav={activeNav()}
        onNavChange={(nav) => setActiveNav(nav)}
        onThemeToggle={toggleTheme}
        currentTheme={currentTheme()}
      />

      {/* 2. 中间面板（依据导航切换） */}
      <div class={styles.middlePanelWrapper}>
        <Show when={activeNav() === "home"}>
          <HomePanel
            totalEntries={totalCount()}
            totalCategories={categories().length}
            totalBooks={books().length}
            recentEntries={recentEntries()}
            onSelectEntry={(id) => {
              selectEntry(id);
              setActiveNav("search");
            }}
            onNavigate={(nav) => setActiveNav(nav)}
          />
        </Show>

        <Show when={activeNav() === "search"}>
          <SearchPanel
            categories={categories()}
            results={items()}
            selectedEntryId={selectedId()}
            isLoading={isLoading()}
            onSearch={(kw, cat, sub) => doSearch(kw, cat, sub)}
            onSelectEntry={(id) => selectEntry(id)}
          />
        </Show>

        <Show when={activeNav() === "library"}>
          <LibraryPanel
            categories={categories()}
            books={books()}
            selectedBookId={selectedBookId()}
            activeBookTree={activeBookTree()}
            selectedEntryId={selectedId()}
            onSelectBook={handleSelectBook}
            onSelectEntry={(id) => selectEntry(id)}
            onBackToBooks={handleBackToBooks}
            onFilterCategory={(cat) => doSearch("", cat || undefined, undefined)}
          />
        </Show>

        <Show when={activeNav() === "bookmark"}>
          <BookmarkPanel
            bookmarks={bookmarks()}
            onSelectEntry={(id) => {
              selectEntry(id);
              setActiveNav("search");
            }}
          />
        </Show>

        <Show when={activeNav() === "settings"}>
          <SettingsPanel
            fontFamily={currentFontFamily()}
            fontSize={currentFontSize()}
            lineHeight={currentLineHeight()}
            theme={currentTheme()}
            showOriginal={showOriginal()}
            showCommentary={showCommentary()}
            showSummary={showSummary()}
            totalEntries={totalCount()}
            onFontFamilyChange={setCurrentFontFamily}
            onFontSizeChange={setCurrentFontSize}
            onLineHeightChange={setCurrentLineHeight}
            onThemeChange={setCurrentTheme}
            onToggleOriginal={() => setShowOriginal(!showOriginal())}
            onToggleCommentary={() => setShowCommentary(!showCommentary())}
            onToggleSummary={() => setShowSummary(!showSummary())}
          />
        </Show>
      </div>

      {/* 3. 右侧阅读器工作区 */}
      <main class={styles.readerPane}>
        <Show
          when={activeEntry()}
          fallback={
            <div class={styles.emptyView}>
              <div class={styles.emptyIcon}>📖</div>
              <h3>请在左侧选择典籍篇目</h3>
              <p>可按全局检索、典籍书库或中医证治体系进行研读</p>
            </div>
          }
        >
          {/* 阅读器顶部工具栏 */}
          <header class={styles.readerHeader}>
            <div class={styles.headerMain}>
              <h2 class={styles.headerTitle}>
                {activeEntry()!.metadata.section_title}
              </h2>
              <div class={styles.headerSub}>
                <span>
                  《{activeEntry()!.metadata.book}》· {activeEntry()!.metadata.chapter}
                </span>
                <span>
                  作者：{activeEntry()!.metadata.author || "先贤/古本"} (
                  {activeEntry()!.metadata.dynasty || "历代"})
                </span>
              </div>
            </div>

            <div class={styles.headerActions}>
              <span class={styles.versionBadge}>
                {activeEntry()!.metadata.source_version || "经典古传"}
              </span>

              {/* 三层正文开关控制 */}
              <div class={styles.layerToggles}>
                <button
                  type="button"
                  class={`${styles.layerToggleBtn} ${
                    showOriginal() ? styles.layerToggleBtnActive : ""
                  }`}
                  onClick={() => setShowOriginal(!showOriginal())}
                  title="显示/隐藏【原文】"
                >
                  📖 原文
                </button>
                <button
                  type="button"
                  class={`${styles.layerToggleBtn} ${
                    showCommentary() ? styles.layerToggleBtnActive : ""
                  }`}
                  onClick={() => setShowCommentary(!showCommentary())}
                  title="显示/隐藏【古注】"
                >
                  📝 古注
                </button>
                <button
                  type="button"
                  class={`${styles.layerToggleBtn} ${
                    showSummary() ? styles.layerToggleBtnActive : ""
                  }`}
                  onClick={() => setShowSummary(!showSummary())}
                  title="显示/隐藏【白话提要】"
                >
                  💬 白话
                </button>
              </div>

              {/* 收藏按钮 */}
              <button
                type="button"
                class={`${styles.bookmarkBtn} ${
                  isBookmarked() ? styles.bookmarkBtnActive : ""
                }`}
                onClick={toggleBookmark}
                title={isBookmarked() ? "已收藏" : "加入收藏"}
              >
                {isBookmarked() ? "★ 已收藏" : "☆ 收藏"}
              </button>
            </div>
          </header>

          {/* 滚动阅读主体 */}
          <div class={styles.readerScroll}>
            <Show when={isReadingLoading()}>
              <div style={{ padding: "1.5rem", color: "var(--text-muted)" }}>
                ⏳ 正在载入典籍正文与名家阐微...
              </div>
            </Show>

            {/* 1. 原文卡片 */}
            <Show when={showOriginal()}>
              <div class={styles.originalCard}>
                <div class={styles.cardSectionTag}>【 原文典经 】</div>
                <div class={styles.originalText}>
                  {activeEntry()!.original_text}
                </div>
              </div>
            </Show>

            {/* 2. 图谱插图区 (若存在插图) */}
            <Show when={activeEntry()!.images && activeEntry()!.images.length > 0}>
              <div class={styles.imageSection}>
                <div class={styles.cardSectionTag}>【 典籍图谱 · 本草经穴图解 】</div>
                <div class={styles.imageGrid}>
                  <For each={activeEntry()!.images}>
                    {(img) => {
                      const resolvedSrc = () => imageMap()[img.path] || img.path;
                      return (
                        <div
                          class={styles.imageThumbBox}
                          onClick={() =>
                            openLightbox(resolvedSrc(), img.title, img.caption)
                          }
                          title="点击全屏放大研察"
                        >
                          <img
                            src={resolvedSrc()}
                            alt={img.title}
                            class={styles.imageThumb}
                            onError={(e) => {
                              e.currentTarget.style.display = "none";
                            }}
                          />
                          <div class={styles.imageCaption}>
                            {img.title}
                          </div>
                        </div>
                      );
                    }}
                  </For>
                </div>
              </div>
            </Show>

            {/* 3. 古注 / 阐微 */}
            <Show when={showCommentary() && activeEntry()!.commentary_text}>
              <div class={styles.commentaryCard}>
                <div class={styles.commentaryTag}>【 先贤阐微 · 古注评析 】</div>
                <div class={styles.commentaryText}>
                  {activeEntry()!.commentary_text}
                </div>
              </div>
            </Show>

            {/* 4. 白话提要 */}
            <Show when={showSummary() && activeEntry()!.summary_text}>
              <div class={styles.summaryCard}>
                <div class={styles.summaryTag}>【 白话提要 · 理法方药精解 】</div>
                <div class={styles.summaryText}>
                  {activeEntry()!.summary_text}
                </div>
              </div>
            </Show>
          </div>
        </Show>
      </main>

      {/* 灯箱模态层 */}
      <Show when={lightboxImage()}>
        <Lightbox
          src={lightboxImage()!.src}
          title={lightboxImage()!.title}
          caption={lightboxImage()!.caption}
          onClose={() => setLightboxImage(null)}
        />
      </Show>
    </div>
  );
}
