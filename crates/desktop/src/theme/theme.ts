import { createSignal, createEffect } from "solid-js";

export type AppTheme = "rice-paper" | "dark-bamboo";
export type FontFamily = "kaiti" | "songti" | "sans" | "fangsong";
export type FontSize = "small" | "medium" | "large" | "xlarge" | "xxlarge";
export type LineHeight = "compact" | "comfortable" | "loose";

const THEME_STORAGE_KEY = "tcm_reader_theme";
const FONT_FAMILY_KEY = "tcm_reader_font";
const FONT_SIZE_KEY = "tcm_reader_fontsize";
const LINE_HEIGHT_KEY = "tcm_reader_lineheight";

export const [currentTheme, setCurrentTheme] = createSignal<AppTheme>(
  (localStorage.getItem(THEME_STORAGE_KEY) as AppTheme) || "rice-paper"
);

export const [currentFontFamily, setCurrentFontFamily] = createSignal<FontFamily>(
  (localStorage.getItem(FONT_FAMILY_KEY) as FontFamily) || "kaiti"
);

export const [currentFontSize, setCurrentFontSize] = createSignal<FontSize>(
  (localStorage.getItem(FONT_SIZE_KEY) as FontSize) || "medium"
);

export const [currentLineHeight, setCurrentLineHeight] = createSignal<LineHeight>(
  (localStorage.getItem(LINE_HEIGHT_KEY) as LineHeight) || "comfortable"
);

export function toggleTheme() {
  const next = currentTheme() === "rice-paper" ? "dark-bamboo" : "rice-paper";
  setCurrentTheme(next);
}

// 监听并应用全局主题与字体属性
createEffect(() => {
  const theme = currentTheme();
  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem(THEME_STORAGE_KEY, theme);
});

createEffect(() => {
  const font = currentFontFamily();
  document.documentElement.setAttribute("data-font", font);
  localStorage.setItem(FONT_FAMILY_KEY, font);
});

createEffect(() => {
  const size = currentFontSize();
  document.documentElement.setAttribute("data-fontsize", size);
  localStorage.setItem(FONT_SIZE_KEY, size);
});

createEffect(() => {
  const lh = currentLineHeight();
  document.documentElement.setAttribute("data-lineheight", lh);
  localStorage.setItem(LINE_HEIGHT_KEY, lh);
});

