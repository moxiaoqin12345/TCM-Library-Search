import { JSX } from "solid-js";

/**
 * 逃逸正则表达式特殊字符
 */
function escapeRegExp(string: string): string {
  return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * 结构化切分文本并给关键词与中医证治临床词添加高亮标签
 * 返回包含纯文本与 <mark> 的 JSX 节点数组，避免直接使用 innerHTML
 */
export function highlightTcmKeywords(
  text: string,
  keywords: string[],
  highlightClass: string
): (JSX.Element | string)[] {
  if (!text) return [""];

  // 过滤空串与过短字符，按长度降序排列（优先匹配长词，如“心肾不交”优先于“心”）
  const validKeywords = Array.from(
    new Set(
      keywords
        .map((k) => k.trim())
        .filter((k) => k.length >= 2)
    )
  ).sort((a, b) => b.length - a.length);

  if (validKeywords.length === 0) {
    return [text];
  }

  const pattern = new RegExp(`(${validKeywords.map(escapeRegExp).join("|")})`, "gi");
  const parts = text.split(pattern);

  return parts.map((part) => {
    const isMatched = validKeywords.some(
      (kw) => kw.toLowerCase() === part.toLowerCase()
    );
    if (isMatched) {
      return (
        <mark class={highlightClass}>
          {part}
        </mark>
      );
    }
    return part;
  });
}
