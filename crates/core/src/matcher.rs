use crate::index::CorpusIndex;
use crate::models::{ManifestEntry, SearchQuery, SearchResultItem};

/// 执行检索匹配并输出加权排序结果
pub fn search_corpus(index: &CorpusIndex, query: &SearchQuery) -> Vec<SearchResultItem> {
    let mut scored_items: Vec<(u32, Vec<String>, &ManifestEntry)> = Vec::new();

    let kw_lower = query
        .keyword
        .as_ref()
        .map(|k| k.trim().to_lowercase())
        .filter(|k| !k.is_empty());

    for entry in index.all_entries() {
        // 1. 分类过滤
        if let Some(ref cat) = query.category {
            if !cat.is_empty() && cat != "all" && &entry.category != cat {
                continue;
            }
        }
        if let Some(ref subcat) = query.subcategory {
            if !subcat.is_empty() && &entry.subcategory != subcat {
                continue;
            }
        }

        let mut matched_dimensions = Vec::new();
        let mut score: u32 = entry.weight;

        // 2. 11 维中医药临床结构化维度匹配
        let cond = &entry.conditions;

        if let Some(ref q_val) = query.yaoming {
            if match_dimension(&cond.yaoming, q_val) {
                matched_dimensions.push(format!("药名:{}", q_val));
                score += 30;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.fangming {
            if match_dimension(&cond.fangming, q_val) {
                matched_dimensions.push(format!("方名:{}", q_val));
                score += 30;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.zhengxing {
            if match_dimension(&cond.zhengxing, q_val) {
                matched_dimensions.push(format!("证型:{}", q_val));
                score += 25;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.zhifa {
            if match_dimension(&cond.zhifa, q_val) {
                matched_dimensions.push(format!("治法:{}", q_val));
                score += 20;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.bingzheng {
            if match_dimension(&cond.bingzheng, q_val) {
                matched_dimensions.push(format!("病症:{}", q_val));
                score += 20;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.zhengzhuang {
            if match_dimension(&cond.zhengzhuang, q_val) {
                matched_dimensions.push(format!("症状:{}", q_val));
                score += 15;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.xuewei {
            if match_dimension(&cond.xuewei, q_val) {
                matched_dimensions.push(format!("腧穴:{}", q_val));
                score += 25;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.jingluo {
            if match_dimension(&cond.jingluo, q_val) {
                matched_dimensions.push(format!("经络:{}", q_val));
                score += 20;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.siqi {
            if match_dimension(&cond.siqi, q_val) {
                matched_dimensions.push(format!("四气:{}", q_val));
                score += 10;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.wuwei {
            if match_dimension(&cond.wuwei, q_val) {
                matched_dimensions.push(format!("五味:{}", q_val));
                score += 10;
            } else {
                continue;
            }
        }

        if let Some(ref q_val) = query.guijing {
            if match_dimension(&cond.guijing, q_val) {
                matched_dimensions.push(format!("归经:{}", q_val));
                score += 10;
            } else {
                continue;
            }
        }

        // 3. 开放关键词检索 (标题、书名、章节、关键词数组)
        if let Some(ref kw) = kw_lower {
            let mut kw_matched = false;

            if entry.title.to_lowercase().contains(kw) {
                matched_dimensions.push("标题命中".to_string());
                score += 40;
                kw_matched = true;
            } else if entry.book.to_lowercase().contains(kw) {
                matched_dimensions.push("书名命中".to_string());
                score += 15;
                kw_matched = true;
            } else if entry.chapter.to_lowercase().contains(kw) {
                matched_dimensions.push("篇目命中".to_string());
                score += 20;
                kw_matched = true;
            } else if entry
                .conditions
                .keywords
                .iter()
                .any(|k| k.to_lowercase().contains(kw))
            {
                matched_dimensions.push("主题词命中".to_string());
                score += 25;
                kw_matched = true;
            } else if match_any_conditions(&entry.conditions, kw) {
                matched_dimensions.push("临床属性命中".to_string());
                score += 20;
                kw_matched = true;
            }

            if !kw_matched {
                continue;
            }
        }

        scored_items.push((score, matched_dimensions, entry));
    }

    // 4. 按评分降序，次级按 ID 排序
    scored_items.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| b.2.weight.cmp(&a.2.weight))
            .then_with(|| a.2.id.cmp(&b.2.id))
    });

    // 5. 分页处理
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(100);

    scored_items
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|(score, dims, entry)| {
            let snippet = build_snippet(entry, &dims, score);
            SearchResultItem {
                id: entry.id.clone(),
                book: entry.book.clone(),
                chapter: entry.chapter.clone(),
                section_title: entry.title.clone(),
                category: entry.category.clone(),
                subcategory: entry.subcategory.clone(),
                item_type: entry.item_type.clone(),
                weight: score,
                matched_dimensions: dims,
                snippet,
                relative_path: entry.path.clone(),
            }
        })
        .collect()
}

fn match_dimension(values: &[String], target: &str) -> bool {
    let t = target.trim().to_lowercase();
    values.iter().any(|v| {
        let v_lower = v.trim().to_lowercase();
        v_lower == t || v_lower.contains(&t) || t.contains(&v_lower)
    })
}

fn match_any_conditions(cond: &crate::models::CorpusConditions, kw: &str) -> bool {
    let lists: [&[String]; 11] = [
        &cond.yaoming,
        &cond.fangming,
        &cond.zhengxing,
        &cond.zhifa,
        &cond.bingzheng,
        &cond.zhengzhuang,
        &cond.xuewei,
        &cond.jingluo,
        &cond.siqi,
        &cond.wuwei,
        &cond.guijing,
    ];
    lists
        .iter()
        .any(|list| list.iter().any(|item| item.to_lowercase().contains(kw)))
}

fn build_snippet(entry: &ManifestEntry, dims: &[String], _score: u32) -> String {
    let cond = &entry.conditions;
    let mut parts = Vec::new();

    if !dims.is_empty() {
        parts.push(format!("【命中】{}", dims.join(" · ")));
    }

    if !cond.zhifa.is_empty() {
        parts.push(format!("治法: {}", cond.zhifa.join("、")));
    }
    if !cond.zhengxing.is_empty() {
        parts.push(format!("证型: {}", cond.zhengxing.join("、")));
    }
    if !cond.bingzheng.is_empty() {
        parts.push(format!("主治: {}", cond.bingzheng.join("、")));
    }

    if parts.is_empty() {
        format!("《{}》{}", entry.book, entry.chapter)
    } else {
        parts.join(" ｜ ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_by_yaoming() {
        let json = r#"{
            "schema_version": 2.1,
            "name": "Test Library",
            "categories": [],
            "total": 2,
            "entries": [
                {
                    "id": "guizhitang_001",
                    "book": "伤寒论",
                    "category": "fangji",
                    "subcategory": "jingfang",
                    "path": "library/fangji/jingfang/guizhitang.md",
                    "title": "桂枝汤",
                    "weight": 8,
                    "conditions": {
                        "fangming": ["桂枝汤"],
                        "yaoming": ["桂枝", "白芍", "生姜"]
                    }
                },
                {
                    "id": "baiziren_001",
                    "book": "药典",
                    "category": "zhongyao",
                    "subcategory": "anshen",
                    "path": "library/zhongyao/anshen/baiziren.md",
                    "title": "柏子仁",
                    "weight": 8,
                    "conditions": {
                        "yaoming": ["柏子仁"]
                    }
                }
            ]
        }"#;

        let index = CorpusIndex::from_manifest_str(json).unwrap();

        // 检索药名: 桂枝
        let q = SearchQuery {
            yaoming: Some("桂枝".into()),
            ..Default::default()
        };
        let results = search_corpus(&index, &q);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "guizhitang_001");

        // 模糊关键词: 柏子
        let q_kw = SearchQuery {
            keyword: Some("柏子".into()),
            ..Default::default()
        };
        let results_kw = search_corpus(&index, &q_kw);
        assert_eq!(results_kw.len(), 1);
        assert_eq!(results_kw[0].id, "baiziren_001");
    }
}
