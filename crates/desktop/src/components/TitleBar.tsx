import { createSignal, onMount, Show } from "solid-js";
import {
  appWindowClose,
  appWindowIsMaximized,
  appWindowMinimize,
  appWindowStartDragging,
  appWindowToggleMaximize,
} from "../services/api";
import { getAppVersion } from "../services/updater";
import {
  currentFontSize,
  currentTheme,
  FontSize,
  setCurrentFontSize,
  toggleTheme,
} from "../theme/theme";
import styles from "./TitleBar.module.css";

interface TitleBarProps {
  totalCount?: number;
  onCheckUpdate?: () => void;
  isCheckingUpdate?: boolean;
}

export default function TitleBar(props: TitleBarProps) {
  const [isMaximized, setIsMaximized] = createSignal(false);
  const [appVersion, setAppVersion] = createSignal("0.2.0");

  onMount(async () => {
    try {
      const max = await appWindowIsMaximized();
      setIsMaximized(max);
    } catch {
      // 浏览器预览静默降级
    }

    try {
      const ver = await getAppVersion();
      setAppVersion(ver);
    } catch {}
  });

  const handleMinimize = () => appWindowMinimize().catch(() => {});
  const handleToggleMaximize = async () => {
    try {
      const state = await appWindowToggleMaximize();
      setIsMaximized(state);
    } catch {}
  };
  const handleClose = () => appWindowClose().catch(() => {});
  const handleMouseDown = (e: MouseEvent) => {
    if (e.buttons === 1 && (e.target as HTMLElement).tagName !== "BUTTON" && (e.target as HTMLElement).tagName !== "SELECT") {
      appWindowStartDragging().catch(() => {});
    }
  };

  const handleFontSizeChange = (e: Event) => {
    const val = (e.target as HTMLSelectElement).value as FontSize;
    setCurrentFontSize(val);
  };

  return (
    <header class={styles.titleBar} onMouseDown={handleMouseDown}>
      <div class={styles.brand}>
        <span class={styles.logo}>📜</span>
        <span class={styles.title}>中医典籍文库</span>
        <span class={styles.versionBadge}>v{appVersion()}</span>
        {props.totalCount ? (
          <span class={styles.badge}>{props.totalCount} 篇条目</span>
        ) : null}
      </div>

      <div class={styles.actions}>
        {/* 字体大小下拉调节器（满足用户指令：下拉调节而不是点击滚动切换） */}
        <div class={styles.selectWrapper}>
          <span class={styles.selectLabel}>字号</span>
          <select
            class={styles.toolSelect}
            value={currentFontSize()}
            onChange={handleFontSizeChange}
            title="调节正文字号"
          >
            <option value="small">小 (14px)</option>
            <option value="medium">中 (16px)</option>
            <option value="large">大 (18px)</option>
            <option value="xlarge">特大 (20px)</option>
            <option value="xxlarge">超大 (22px)</option>
          </select>
        </div>

        {/* 主题切换 (宣纸 / 暗竹) */}
        <button
          type="button"
          class={styles.toolBtn}
          onClick={toggleTheme}
          title="切换日间宣纸 / 暮墨玄竹主题"
        >
          {currentTheme() === "rice-paper" ? "🏮 仿古宣纸" : "🎋 暮墨玄竹"}
        </button>

        {/* 检查更新快捷按钮 */}
        <Show when={props.onCheckUpdate}>
          <button
            type="button"
            class={styles.toolBtn}
            onClick={props.onCheckUpdate}
            title="检查软件在线更新 (GitHub Releases)"
          >
            {props.isCheckingUpdate ? "🔄 检查中..." : "🚀 检查更新"}
          </button>
        </Show>

        {/* 窗口控制按钮 */}
        <div class={styles.windowControls}>
          <button
            type="button"
            class={styles.winBtn}
            onClick={handleMinimize}
            title="最小化"
          >
            ─
          </button>
          <button
            type="button"
            class={styles.winBtn}
            onClick={handleToggleMaximize}
            title={isMaximized() ? "还原" : "最大化"}
          >
            {isMaximized() ? "❐" : "□"}
          </button>
          <button
            type="button"
            class={`${styles.winBtn} ${styles.winBtnClose}`}
            onClick={handleClose}
            title="关闭"
          >
            ✕
          </button>
        </div>
      </div>
    </header>
  );
}
