import { Component } from 'solid-js';
import { exportBackupJSON, importBackupJSON } from '../services/export';
import styles from './SettingsPanel.module.css';
import type { AppTheme, FontFamily, FontSize } from '../theme/theme';

export type LineHeight = 'compact' | 'comfortable' | 'loose';

export interface SettingsPanelProps {
  fontFamily: FontFamily;
  fontSize: FontSize;
  lineHeight: LineHeight;
  theme: AppTheme;
  showOriginal: boolean;
  showCommentary: boolean;
  showSummary: boolean;
  totalEntries: number;
  appVersion?: string;
  onCheckUpdate?: () => void;
  isCheckingUpdate?: boolean;
  lastUpdateCheckTime?: string;
  onFontFamilyChange: (v: FontFamily) => void;
  onFontSizeChange: (v: FontSize) => void;
  onLineHeightChange: (v: LineHeight) => void;
  onThemeChange: (v: AppTheme) => void;
  onToggleOriginal: () => void;
  onToggleCommentary: () => void;
  onToggleSummary: () => void;
}

export const SettingsPanel: Component<SettingsPanelProps> = (props) => {
  return (
    <div class={styles.container}>
      <h1 class={styles.title}>阅读与显示设置</h1>

      <div class={styles.settingsGroup}>
        <div class={styles.settingItem}>
          <label class={styles.settingLabel} for="font-family">阅读字体</label>
          <select
            id="font-family"
            class={styles.settingSelect}
            value={props.fontFamily}
            onChange={(e) => props.onFontFamilyChange(e.currentTarget.value as FontFamily)}
          >
            <option value="kaiti">楷体(KaiTi)</option>
            <option value="songti">宋体(SongTi)</option>
            <option value="sans">黑体(Sans)</option>
            <option value="fangsong">仿宋(FangSong)</option>
          </select>
        </div>

        <div class={styles.settingItem}>
          <label class={styles.settingLabel} for="font-size">字体大小</label>
          <select
            id="font-size"
            class={styles.settingSelect}
            value={props.fontSize}
            onChange={(e) => props.onFontSizeChange(e.currentTarget.value as FontSize)}
          >
            <option value="small">小(14px)</option>
            <option value="medium">中(16px·推荐)</option>
            <option value="large">大(18px)</option>
            <option value="xlarge">特大(20px)</option>
            <option value="xxlarge">超大(22px)</option>
          </select>
        </div>

        <div class={styles.settingItem}>
          <label class={styles.settingLabel} for="line-height">行间距</label>
          <select
            id="line-height"
            class={styles.settingSelect}
            value={props.lineHeight}
            onChange={(e) => props.onLineHeightChange(e.currentTarget.value as LineHeight)}
          >
            <option value="compact">紧凑(1.6)</option>
            <option value="comfortable">舒适(1.9·推荐)</option>
            <option value="loose">宽松(2.2)</option>
          </select>
        </div>

        <div class={styles.settingItem}>
          <label class={styles.settingLabel} for="app-theme">外观主题</label>
          <select
            id="app-theme"
            class={styles.settingSelect}
            value={props.theme}
            onChange={(e) => props.onThemeChange(e.currentTarget.value as AppTheme)}
          >
            <option value="rice-paper">🏮 仿古宣纸（浅色）</option>
            <option value="dark-bamboo">🎋 暮墨玄竹（深色）</option>
          </select>
        </div>

        <div class={styles.settingCheckboxGroup}>
          <div class={styles.settingLabel}>默认展示层</div>
          <div class={styles.checkboxes}>
            <label class={styles.checkboxLabel}>
              <input
                type="checkbox"
                checked={props.showOriginal}
                onChange={props.onToggleOriginal}
              />
              原文
            </label>
            <label class={styles.checkboxLabel}>
              <input
                type="checkbox"
                checked={props.showCommentary}
                onChange={props.onToggleCommentary}
              />
              古注
            </label>
            <label class={styles.checkboxLabel}>
              <input
                type="checkbox"
                checked={props.showSummary}
                onChange={props.onToggleSummary}
              />
              白话
            </label>
          </div>
        </div>

        {/* 软件版本与自动更新 */}
        <div class={styles.updateSection}>
          <div class={styles.settingLabel}>软件版本与在线更新</div>
          <div class={styles.updateCard}>
            <div class={styles.updateInfo}>
              <div class={styles.updateVersionRow}>
                <span class={styles.updateProduct}>中医典籍文库</span>
                <span class={styles.updateVersionPill}>v{props.appVersion || "0.2.0"}</span>
                <span class={styles.updateChannel}>GitHub Releases</span>
              </div>
              <p class={styles.updateDesc}>
                支持通过官方 GitHub Releases 自动拉取跨平台安全更新与最新中医算力规则库。
              </p>
              <div class={styles.updateCheckTime}>
                上次检测：{props.lastUpdateCheckTime || "尚未检测"}
              </div>
            </div>
            <div class={styles.updateActionArea}>
              <button
                type="button"
                class={styles.checkUpdateBtn}
                disabled={props.isCheckingUpdate}
                onClick={props.onCheckUpdate}
                title="检查 GitHub Releases 最新版本"
              >
                {props.isCheckingUpdate ? "🔄 正在连接..." : "🚀 检查更新"}
              </button>
            </div>
          </div>
        </div>

        {/* 研读数据备份与迁移 */}
        <div class={styles.backupSection}>
          <div class={styles.settingLabel}>研读数据备份与迁移</div>
          <div class={styles.backupActions}>
            <button
              type="button"
              class={styles.backupBtn}
              onClick={() => exportBackupJSON()}
              title="将所有书签与研读心得导出为本地 JSON 文件"
            >
              📤 备份全部心得 (JSON)
            </button>

            <label class={styles.backupBtn} style={{ cursor: "pointer" }}>
              📥 导入恢复心得 (JSON)
              <input
                type="file"
                accept=".json,application/json"
                style={{ display: "none" }}
                onChange={async (e) => {
                  const files = e.currentTarget.files;
                  if (files && files[0]) {
                    const res = await importBackupJSON(files[0]);
                    if (res.success) {
                      alert(`成功恢复 ${res.count} 条收藏与研读心得！`);
                      window.location.reload();
                    } else {
                      alert(`导入失败：${res.error}`);
                    }
                  }
                }}
              />
            </label>
          </div>
          <div class={styles.backupTip}>
            提示：备份文件包含您标注的所有条目及随记临床心得，支持在不同设备间无损迁移。
          </div>
        </div>
      </div>

      <div class={styles.footer}>
        <p>中医典籍文库 v{props.appVersion || "0.2.0"} · 已收录条目：{props.totalEntries}</p>
      </div>
    </div>
  );
};
