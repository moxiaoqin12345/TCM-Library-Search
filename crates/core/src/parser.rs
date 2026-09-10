use crate::error::{CoreError, Result};
use crate::models::{CorpusImage, CorpusMetadata, TcmEntryDetail};
use regex::Regex;

/// 解析 Markdown 条目内容（含 YAML Frontmatter 与三层正文）
pub fn parse_markdown_entry(content: &str) -> Result<TcmEntryDetail> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Err(CoreError::MarkdownFormat(
            "Missing YAML frontmatter starting delimiter '---'".into(),
        ));
    }

    let end_idx = trimmed[3..]
        .find("\n---")
        .ok_or_else(|| CoreError::MarkdownFormat("Unclosed YAML frontmatter delimiter".into()))?;

    let yaml_str = &trimmed[3..3 + end_idx];
    let body_str = trimmed[3 + end_idx + 4..].trim();

    let mut metadata: CorpusMetadata = serde_yaml::from_str(yaml_str).map_err(CoreError::Yaml)?;

    // 提取正文中的图片引用
    let inline_images = extract_markdown_images(body_str);

    // 解析三层互文：【原文】、【古注/阐微】、【白话提要】
    let (original_text, commentary_text, summary_text) = parse_three_tiers(body_str);

    // 合并 Frontmatter 声明的图片与正文图片 (避免重复)
    let mut all_images = metadata.images.clone();
    for img in inline_images {
        if !all_images.iter().any(|existing| existing.path == img.path) {
            all_images.push(img);
        }
    }
    metadata.images = all_images.clone();

    Ok(TcmEntryDetail {
        metadata,
        original_text,
        commentary_text,
        summary_text,
        images: all_images,
    })
}

/// 提取正文中 Markdown 语法标引的图片 `![alt](url "title")`
fn extract_markdown_images(body: &str) -> Vec<CorpusImage> {
    let re = Regex::new(r"!\[(.*?)\]\((.*?)(?:\s+[\x22'](.*?)[\x22'])?\)").unwrap();
    let mut images = Vec::new();

    for cap in re.captures_iter(body) {
        let alt = cap.get(1).map_or("", |m| m.as_str()).trim();
        let path = cap.get(2).map_or("", |m| m.as_str()).trim();
        let title_opt = cap.get(3).map(|m| m.as_str().trim().to_string());

        if !path.is_empty() {
            let img_type = if path.contains("herb")
                || path.contains("yaocai")
                || path.contains("plant")
            {
                "herb_specimen".to_string()
            } else if path.contains("tongue")
                || path.contains("shetai")
                || path.contains("shexiang")
            {
                "tongue_pulse".to_string()
            } else if path.contains("xue") || path.contains("jingluo") || path.contains("acupoint")
            {
                "acupoint_meridian".to_string()
            } else {
                "inline".to_string()
            };

            let title = if !alt.is_empty() {
                alt.to_string()
            } else if let Some(ref t) = title_opt {
                t.clone()
            } else {
                "典籍插图".to_string()
            };

            images.push(CorpusImage {
                path: path.to_string(),
                title,
                caption: title_opt,
                image_type: img_type,
                source: None,
            });
        }
    }

    images
}

/// 切分正文三层互文结构
fn parse_three_tiers(body: &str) -> (String, Option<String>, Option<String>) {
    let lines: Vec<&str> = body.lines().collect();

    let mut original_lines = Vec::new();
    let mut commentary_lines = Vec::new();
    let mut summary_lines = Vec::new();

    #[derive(PartialEq)]
    enum Section {
        None,
        Original,
        Commentary,
        Summary,
    }

    let mut current_sec = Section::None;

    for line in lines {
        let trimmed = line.trim();

        // 识别段落标记
        if is_original_marker(trimmed) {
            current_sec = Section::Original;
            continue;
        } else if is_commentary_marker(trimmed) {
            current_sec = Section::Commentary;
            continue;
        } else if is_summary_marker(trimmed) {
            current_sec = Section::Summary;
            continue;
        }

        match current_sec {
            Section::Original => original_lines.push(line),
            Section::Commentary => commentary_lines.push(line),
            Section::Summary => summary_lines.push(line),
            Section::None => {
                // 如果没有显式标记，默认作为原文正文（如简单的古籍开篇）
                if !trimmed.starts_with('#') {
                    original_lines.push(line);
                }
            }
        }
    }

    let original_text = original_lines.join("\n").trim().to_string();
    let commentary_text = if commentary_lines.is_empty() {
        None
    } else {
        Some(commentary_lines.join("\n").trim().to_string())
    };
    let summary_text = if summary_lines.is_empty() {
        None
    } else {
        Some(summary_lines.join("\n").trim().to_string())
    };

    (original_text, commentary_text, summary_text)
}

fn is_original_marker(line: &str) -> bool {
    line.contains("【原文】") || line.contains("**【原文】**") || line.contains("## 原文")
}

fn is_commentary_marker(line: &str) -> bool {
    line.contains("【古注】")
        || line.contains("【阐微】")
        || line.contains("【评析】")
        || line.contains("【注释】")
        || line.contains("**【古注】**")
        || line.contains("## 古注")
}

fn is_summary_marker(line: &str) -> bool {
    line.contains("【白话提要】")
        || line.contains("【白话】")
        || line.contains("【方解】")
        || line.contains("【提要】")
        || line.contains("**【白话提要】**")
        || line.contains("## 白话提要")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_baiziren_sample() {
        let md = r#"---
id: "baiziren_001"
book: "中国药典（2025年版）一部"
chapter: "安神药"
section_title: "柏子仁"
source_version: "《中华人民共和国药典》2025年版一部"
author: "国家药典委员会"
dynasty: "现代"
type: "yaowu"
conditions:
  zhengxing: ["心肾不交", "血虚"]
  zhifa: ["养心安神", "润肠通便"]
  bingzheng: ["虚烦不眠", "心悸怔忡"]
  yaoming: ["柏子仁"]
weight: 8
tags: ["中药学", "单味药"]
---

### 柏子仁

**【原文】**
柏子仁为柏科植物侧柏的干燥成熟种仁。性味甘，平；归心、肾、大肠经。
![侧柏植株图](images/baiziren_plant.webp "成熟球果")

**【古注】**
《本草纲目》：柏子仁性平而不寒不燥，味甘而补。

**【白话提要】**
简明：柏子仁甘平，养心安神、润肠通便。
"#;

        let entry = parse_markdown_entry(md).expect("Parsing should succeed");
        assert_eq!(entry.metadata.id, "baiziren_001");
        assert_eq!(entry.metadata.book, "中国药典（2025年版）一部");
        assert!(entry.original_text.contains("侧柏的干燥成熟种仁"));
        assert!(entry.commentary_text.as_ref().unwrap().contains("本草纲目"));
        assert!(entry.summary_text.as_ref().unwrap().contains("养心安神"));
        assert_eq!(entry.images.len(), 1);
        assert_eq!(entry.images[0].path, "images/baiziren_plant.webp");
        assert_eq!(entry.images[0].title, "侧柏植株图");
    }
}
