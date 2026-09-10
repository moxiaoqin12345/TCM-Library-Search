import { createSignal, createMemo, createEffect, For, Show } from 'solid-js';
import styles from './SearchPanel.module.css';
import type { CategoryTreeItem, SearchResultItem } from '../services/api';

interface SearchPanelProps {
  categories: CategoryTreeItem[];
  results: SearchResultItem[];
  selectedEntryId: string;
  isLoading: boolean;
  onSearch: (keyword: string, category?: string, subcategory?: string) => void;
  onSelectEntry: (id: string) => void;
}

export function SearchPanel(props: SearchPanelProps) {
  const [keyword, setKeyword] = createSignal('');
  const [selectedCategory, setSelectedCategory] = createSignal('');
  const [selectedSubcategory, setSelectedSubcategory] = createSignal('');
  const [_selectedDimension, setSelectedDimension] = createSignal('');
  const [selectedSort, setSelectedSort] = createSignal('relevance');

  const fireSearch = () => {
    props.onSearch(
      keyword(),
      selectedCategory() || undefined,
      selectedSubcategory() || undefined,
    );
  };

  const handleKeywordInput = (e: InputEvent) => {
    setKeyword((e.target as HTMLInputElement).value);
    fireSearch();
  };

  const handleKeywordKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter') {
      fireSearch();
    }
  };

  const handleCategoryChange = (e: Event) => {
    setSelectedCategory((e.target as HTMLSelectElement).value);
    setSelectedSubcategory('');
    fireSearch();
  };

  const handleSubcategoryChange = (e: Event) => {
    setSelectedSubcategory((e.target as HTMLSelectElement).value);
    fireSearch();
  };

  // 联动子分类列表
  const subcategories = createMemo(() => {
    if (!selectedCategory()) return [];
    const cat = props.categories.find(c => c.id === selectedCategory());
    return cat?.subcategories || [];
  });

  // 分类变更时自动触发搜索
  createEffect(() => {
    selectedCategory();
    selectedSubcategory();
  });

  return (
    <div class={styles.panel}>
      <div class={styles.header}>
        <h3 class={styles.title}>全局检索</h3>

        <div class={styles.searchInputWrapper}>
          <span class={styles.searchIcon}>🔍</span>
          <input
            type="text"
            class={styles.searchInput}
            placeholder="搜索本草、方剂、经穴、证型、典籍..."
            value={keyword()}
            onInput={handleKeywordInput}
            onKeyDown={handleKeywordKeyDown}
          />
        </div>

        <div class={styles.filters}>
          <div class={styles.filterRow}>
            <select class={styles.select} value={selectedCategory()} onChange={handleCategoryChange}>
              <option value="">全部分类</option>
              <For each={props.categories}>
                {(cat) => <option value={cat.id}>{cat.name_zh} ({cat.count})</option>}
              </For>
            </select>

            <select class={styles.select} value={selectedSubcategory()} onChange={handleSubcategoryChange}>
              <option value="">全部子类</option>
              <For each={subcategories()}>
                {(sub) => <option value={sub.id}>{sub.name_zh} ({sub.count})</option>}
              </For>
            </select>
          </div>

          <div class={styles.filterRow}>
            <select class={styles.select} onChange={(e) => setSelectedDimension((e.target as HTMLSelectElement).value)}>
              <option value="">维度筛选</option>
              <option value="zhengxing">证型</option>
              <option value="zhifa">治法</option>
              <option value="bingzheng">病症</option>
              <option value="zhengzhuang">症状</option>
              <option value="fangming">方名</option>
              <option value="yaoming">药名</option>
              <option value="xuewei">腧穴</option>
              <option value="jingluo">经络</option>
              <option value="siqi">四气</option>
              <option value="wuwei">五味</option>
              <option value="guijing">归经</option>
            </select>

            <select class={styles.select} value={selectedSort()} onChange={(e) => setSelectedSort((e.target as HTMLSelectElement).value)}>
              <option value="relevance">按相关度</option>
              <option value="weight">按权重</option>
              <option value="book">按书名</option>
            </select>
          </div>
        </div>
      </div>

      <div class={styles.resultsHeader}>
        <span class={styles.resultsCount}>检索结果 ({props.results.length})</span>
        <span class={styles.sortInfo}>
          {selectedSort() === 'relevance' ? '按相关度排序' : selectedSort() === 'weight' ? '按权重排序' : '按书名排序'}
        </span>
      </div>

      <div class={styles.resultsList}>
        <Show when={!props.isLoading} fallback={
          <div class={styles.loading}>正在加载典籍经目...</div>
        }>
          <For each={props.results}>
            {(item) => (
              <div
                class={`${styles.entryCard} ${props.selectedEntryId === item.id ? styles.active : ''}`}
                onClick={() => props.onSelectEntry(item.id)}
              >
                <div class={styles.cardHeader}>
                  <span class={styles.entryTitle}>{item.section_title}</span>
                  <span class={styles.bookBadge}>{item.book}</span>
                </div>
                <p class={styles.snippet}>{item.snippet}</p>
                <Show when={item.matched_dimensions && item.matched_dimensions.length > 0}>
                  <div class={styles.dimensionTags}>
                    <For each={item.matched_dimensions}>
                      {(dim) => <span class={styles.tag}>{dim}</span>}
                    </For>
                  </div>
                </Show>
              </div>
            )}
          </For>
          <Show when={props.results.length === 0}>
            <div class={styles.empty}>未找到匹配结果</div>
          </Show>
        </Show>
      </div>
    </div>
  );
}
