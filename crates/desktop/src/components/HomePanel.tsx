import { Component, For, createSignal } from 'solid-js';
import type { BookInfo } from './LibraryPanel';
import styles from './HomePanel.module.css';

export interface HomePanelProps {
  totalEntries: number;
  totalCategories: number;
  totalBooks: number;
  recentEntries: Array<{ id: string; title: string; book: string; subcategory: string }>;
  books: BookInfo[];
  onSelectEntry: (id: string) => void;
  onSelectBook: (bookId: string) => void;
  onQuickSearch: (keyword: string) => void;
  onOpenDiff: () => void;
  onNavigate: (navId: string) => void;
}

const CLASSIC_QUOTES = [
  {
    text: "凡大医治病，必当安神定志，无欲无求，先发大慈恻隐之心，誓愿普救含灵之苦。",
    author: "唐 · 孙思邈",
    source: "《备急千金要方 · 大医精诚》",
  },
  {
    text: "余每览越人入虢之诊，望齐侯之色，未尝不慨然叹其才秀也。怪当今居世之士，曾不留神医药，精究方术...",
    author: "东汉 · 张仲景",
    source: "《伤寒杂病论 · 自序》",
  },
  {
    text: "上古之人，其知道者，法于阴阳，和于术数，食饮有节，起居有常，不妄作劳，故能形与神俱，而尽终其天年，度百岁乃去。",
    author: "战国 · 佚名",
    source: "《黄帝内经 · 素问 · 上古天真论》",
  },
  {
    text: "经脉者，所以能决死生，处百病，调虚实，不可不通。",
    author: "战国 · 佚名",
    source: "《黄帝内经 · 灵枢 · 经脉》",
  },
  {
    text: "神农尝百草之滋味，知水泉之甘苦，令民知所避就。当此之时，一日而遇七十毒。",
    author: "西汉 · 刘安",
    source: "《淮南子 · 修务训》",
  },
];

const SUGGESTED_TAGS = [
  "桂枝汤",
  "柴胡",
  "足三里",
  "太阳中风",
  "失眠",
  "黄连",
  "补虚药",
  "头痛",
];

export const HomePanel: Component<HomePanelProps> = (props) => {
  const [searchInput, setSearchInput] = createSignal("");
  const [quoteIndex, setQuoteIndex] = createSignal(0);

  const handleNextQuote = () => {
    setQuoteIndex((prev) => (prev + 1) % CLASSIC_QUOTES.length);
  };

  const handleSearchSubmit = (e?: Event) => {
    if (e) e.preventDefault();
    const query = searchInput().trim();
    if (query) {
      props.onQuickSearch(query);
    } else {
      props.onNavigate("search");
    }
  };

  const handleTagClick = (tag: string) => {
    setSearchInput(tag);
    props.onQuickSearch(tag);
  };

  const currentQuote = () => CLASSIC_QUOTES[quoteIndex()];

  return (
    <div class={styles.container}>
      {/* 1. Hero 门户标语与大搜索区 */}
      <section class={styles.heroSection}>
        <div class={styles.sealBadge}>
          <span>岐黄</span>
          <span>正脉</span>
        </div>
        <h1 class={styles.portalTitle}>中医典籍知识文库</h1>
        <p class={styles.motto}>博涉知病 · 循经考方 · 探赜索隐 · 守正笃行</p>

        {/* 居中全局快速检索条 */}
        <form class={styles.searchForm} onSubmit={handleSearchSubmit}>
          <div class={styles.searchInputWrapper}>
            <span class={styles.searchIcon}>🔍</span>
            <input
              type="text"
              class={styles.searchInput}
              placeholder="检索方剂、本草、证候、经穴、经典条文（如：桂枝汤、足三里、失眠）..."
              value={searchInput()}
              onInput={(e) => setSearchInput(e.currentTarget.value)}
            />
            <button type="submit" class={styles.searchSubmitBtn}>
              全文检索
            </button>
          </div>
          {/* 热词胶囊 */}
          <div class={styles.tagsRow}>
            <span class={styles.tagsLabel}>研读热搜：</span>
            <For each={SUGGESTED_TAGS}>
              {(tag) => (
                <button
                  type="button"
                  class={styles.tagPill}
                  onClick={() => handleTagClick(tag)}
                >
                  {tag}
                </button>
              )}
            </For>
          </div>
        </form>

        {/* 文库收录宏观指标 */}
        <div class={styles.statsBar}>
          <div class={styles.statItem}>
            <span class={styles.statIcon}>📚</span>
            <span class={styles.statVal}>{props.totalBooks}</span>
            <span class={styles.statDesc}>经典典籍</span>
          </div>
          <div class={styles.statDivider} />
          <div class={styles.statItem}>
            <span class={styles.statIcon}>📋</span>
            <span class={styles.statVal}>{props.totalEntries}</span>
            <span class={styles.statDesc}>条目收录</span>
          </div>
          <div class={styles.statDivider} />
          <div class={styles.statItem}>
            <span class={styles.statIcon}>🏷️</span>
            <span class={styles.statVal}>{props.totalCategories}</span>
            <span class={styles.statDesc}>学术大类</span>
          </div>
          <div class={styles.statDivider} />
          <div class={styles.statItem}>
            <span class={styles.statIcon}>🌿</span>
            <span class={styles.statVal}>47</span>
            <span class={styles.statDesc}>专业子类</span>
          </div>
          <div class={styles.statDivider} />
          <div class={styles.statItem}>
            <span class={styles.statIcon}>🔬</span>
            <span class={styles.statVal}>11</span>
            <span class={styles.statDesc}>维辨证标签</span>
          </div>
        </div>
      </section>

      {/* 2. 每日医经箴言 */}
      <section class={styles.quoteSection}>
        <div class={styles.quoteCard}>
          <div class={styles.quoteMarkLeft}>“</div>
          <div class={styles.quoteContent}>
            <p class={styles.quoteText}>{currentQuote().text}</p>
            <div class={styles.quoteMeta}>
              <span class={styles.quoteAuthor}>{currentQuote().author}</span>
              <span class={styles.quoteSource}>{currentQuote().source}</span>
            </div>
          </div>
          <div class={styles.quoteActions}>
            <button
              class={styles.refreshQuoteBtn}
              onClick={handleNextQuote}
              title="换一箴言"
            >
              🔄 换一箴言
            </button>
          </div>
        </div>
      </section>

      {/* 3. 四大核心算力与交互工具 */}
      <section class={styles.section}>
        <div class={styles.sectionHeader}>
          <h2 class={styles.sectionTitle}>
            <span class={styles.titleDecor}>❖</span> 核心学术工作台与智能引擎
          </h2>
          <span class={styles.sectionHint}>多维结构计算 · 临床安全保障 · 异文精密比对</span>
        </div>
        <div class={styles.featureGrid}>
          {/* 特性 1: 多维结构检索 */}
          <div
            class={styles.featureCard}
            onClick={() => props.onNavigate("search")}
          >
            <div class={styles.featureIconBadge}>🔍</div>
            <div class={styles.featureBody}>
              <h3 class={styles.featureTitle}>11 维中医药临床检索</h3>
              <p class={styles.featureDesc}>
                证型、治法、病症、症状、方药、经穴 11 维特异性加权推演，精准定位经典论治篇章。
              </p>
            </div>
            <div class={styles.featureLink}>进入全局检索 →</div>
          </div>

          {/* 特性 2: 配伍禁忌雷达 */}
          <div
            class={styles.featureCard}
            onClick={() => props.onNavigate("search")}
          >
            <div class={styles.featureIconBadge}>⚡</div>
            <div class={styles.featureBody}>
              <h3 class={styles.featureTitle}>中药配伍禁忌实时雷达</h3>
              <p class={styles.featureDesc}>
                内置“十八反”、“十九畏”与“妊娠禁忌”规则库，阅读研读时自动碰撞核验，提示歌诀与辨析。
              </p>
            </div>
            <div class={styles.featureLink}>查验方药配伍 →</div>
          </div>

          {/* 特性 3: 异文校勘工作台 */}
          <div
            class={styles.featureCard}
            onClick={() => props.onOpenDiff()}
          >
            <div class={styles.featureIconBadge}>⚖️</div>
            <div class={styles.featureBody}>
              <h3 class={styles.featureTitle}>古今双篇异文校勘台</h3>
              <p class={styles.featureDesc}>
                基于 LCS 字符级对比算法，分屏并排对照宋本、古本文字衍脱与方剂药量增删出入。
              </p>
            </div>
            <div class={styles.featureLink}>调起校勘对比 →</div>
          </div>

          {/* 特性 4: 经络流注图谱 */}
          <div
            class={styles.featureCard}
            onClick={() => props.onNavigate("meridian")}
          >
            <div class={styles.featureIconBadge}>🌿</div>
            <div class={styles.featureBody}>
              <h3 class={styles.featureTitle}>十二经脉气血流注图谱</h3>
              <p class={styles.featureDesc}>
                子午流注时辰走向可视化，五输穴、原络郄俞募特定要穴分类检索与临床对症智能选穴。
              </p>
            </div>
            <div class={styles.featureLink}>研读经络穴位 →</div>
          </div>
        </div>
      </section>

      {/* 4. 经典医籍书架 */}
      <section class={styles.section}>
        <div class={styles.sectionHeader}>
          <h2 class={styles.sectionTitle}>
            <span class={styles.titleDecor}>❖</span> 馆藏经典经籍
          </h2>
          <button
            class={styles.sectionMoreBtn}
            onClick={() => props.onNavigate("library")}
          >
            浏览全部典籍 →
          </button>
        </div>
        <div class={styles.bookshelfGrid}>
          <For each={props.books}>
            {(book) => (
              <div
                class={styles.bookCard}
                onClick={() => props.onSelectBook(book.id)}
              >
                <div
                  class={styles.bookCover}
                  style={{ background: book.coverColor }}
                >
                  <div class={styles.bookCoverBorder}>
                    <div class={styles.bookCoverText}>{book.coverText}</div>
                  </div>
                  <span class={styles.bookDynastyBadge}>{book.dynasty}</span>
                </div>
                <div class={styles.bookInfo}>
                  <h4 class={styles.bookTitle}>{book.title}</h4>
                  <p class={styles.bookAuthor}>
                    {book.author} · {book.category}
                  </p>
                  <p class={styles.bookSummary}>{book.description}</p>
                  <div class={styles.bookMeta}>
                    <span class={styles.bookEntriesCount}>
                      共 {book.entryCount} 篇
                    </span>
                    <span class={styles.bookReadLink}>开启研读</span>
                  </div>
                </div>
              </div>
            )}
          </For>
        </div>
      </section>

      {/* 5. 最近研读足迹 */}
      <section class={styles.section}>
        <div class={styles.sectionHeader}>
          <h2 class={styles.sectionTitle}>
            <span class={styles.titleDecor}>❖</span> 最近研读足迹
          </h2>
          <span class={styles.sectionHint}>即时断点续读 · 随身批注心得</span>
        </div>
        <div class={styles.recentGrid}>
          <For
            each={props.recentEntries}
            fallback={
              <div class={styles.emptyRecent}>
                <span class={styles.emptyIcon}>📖</span>
                <p>暂无最近研读记录，点击上方典籍或搜索关键词即可开启探赜研读。</p>
              </div>
            }
          >
            {(entry) => (
              <div
                class={styles.recentCard}
                onClick={() => props.onSelectEntry(entry.id)}
              >
                <div class={styles.recentTop}>
                  <span class={styles.recentBook}>《{entry.book}》</span>
                  <span class={styles.recentCat}>{entry.subcategory}</span>
                </div>
                <h4 class={styles.recentTitle}>{entry.title}</h4>
                <div class={styles.recentAction}>继续阅读 →</div>
              </div>
            )}
          </For>
        </div>
      </section>
    </div>
  );
};

