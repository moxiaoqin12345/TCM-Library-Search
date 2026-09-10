import { Component, For, Show, createSignal, onMount, onCleanup } from "solid-js";
import { SearchResultItem, searchEntries } from "../services/api";
import styles from "./TermPopover.module.css";

interface TermPopoverProps {
  term: string;
  x: number;
  y: number;
  onClose: () => void;
  onSelectEntry: (id: string) => void;
  onSearchGlobal: (keyword: string) => void;
}

export const TermPopover: Component<TermPopoverProps> = (props) => {
  const [relatedItems, setRelatedItems] = createSignal<SearchResultItem[]>([]);
  const [isLoading, setIsLoading] = createSignal<boolean>(true);

  // 点击外部自动关闭
  const handleDocClick = (e: MouseEvent) => {
    const target = e.target as HTMLElement;
    if (!target.closest(`.${styles.card}`)) {
      props.onClose();
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      props.onClose();
    }
  };

  onMount(async () => {
    document.addEventListener("mousedown", handleDocClick);
    window.addEventListener("keydown", handleKeyDown);

    try {
      setIsLoading(true);
      const res = await searchEntries({
        keyword: props.term.trim(),
        limit: 4,
      });
      setRelatedItems(res);
    } catch (e) {
      console.warn("Failed to query related entries for term:", e);
    } finally {
      setIsLoading(false);
    }
  });

  onCleanup(() => {
    document.removeEventListener("mousedown", handleDocClick);
    window.removeEventListener("keydown", handleKeyDown);
  });

  // 视口边界自适应计算
  const safePosition = () => {
    const cardWidth = 280;
    const cardHeight = 260;
    const padding = 16;

    let left = props.x + 8;
    let top = props.y + 12;

    if (left + cardWidth > window.innerWidth - padding) {
      left = Math.max(padding, window.innerWidth - cardWidth - padding);
    }
    if (top + cardHeight > window.innerHeight - padding) {
      top = Math.max(padding, props.y - cardHeight - 8);
    }

    return { left: `${left}px`, top: `${top}px` };
  };

  return (
    <div
      class={styles.popoverOverlay}
      style={safePosition()}
      onClick={(e) => e.stopPropagation()}
    >
      <div class={styles.card}>
        <div class={styles.header}>
          <div class={styles.titleGroup}>
            <span class={styles.termBadge}>术语速查</span>
            <span class={styles.termName}>{props.term}</span>
          </div>
          <button
            type="button"
            class={styles.closeBtn}
            onClick={props.onClose}
            title="关闭 (Esc)"
          >
            ✕
          </button>
        </div>

        <div class={styles.body}>
          <div class={styles.definition}>
            中医药专业经典术语，可于全库中对照方剂、药性或辨证脉络研察。
          </div>

          <div class={styles.sectionLabel}>关联典籍篇目 ({relatedItems().length})</div>

          <Show
            when={!isLoading()}
            fallback={<div class={styles.loading}>正在查询文库释义与条目...</div>}
          >
            <div class={styles.relatedList}>
              <For
                each={relatedItems()}
                fallback={<div class={styles.loading}>文库暂无该词专属条目</div>}
              >
                {(item) => (
                  <div
                    class={styles.relatedItem}
                    onClick={() => {
                      props.onSelectEntry(item.id);
                      props.onClose();
                    }}
                    title={`阅读《${item.book}》· ${item.section_title}`}
                  >
                    <span class={styles.relatedTitle}>{item.section_title}</span>
                    <span class={styles.relatedBook}>{item.book}</span>
                  </div>
                )}
              </For>
            </div>
          </Show>
        </div>

        <div class={styles.footer}>
          <button
            type="button"
            class={styles.footerAction}
            onClick={() => {
              props.onSearchGlobal(props.term);
              props.onClose();
            }}
          >
            🔍 全局深层检索此词 →
          </button>
        </div>
      </div>
    </div>
  );
};
