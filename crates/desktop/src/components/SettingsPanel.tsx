import { Component } from 'solid-js';
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
      </div>

      <div class={styles.footer}>
        <p>中医典籍文库 v1.0.0 · 已收录条目：{props.totalEntries}</p>
      </div>
    </div>
  );
};
