use serde::{Deserialize, Serialize};

/// 文本比对差异操作符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffOp {
    /// 相同文字
    Equal,
    /// 删减/原底本独有（脱文）
    Delete,
    /// 新增/校本独有（衍文）
    Insert,
}

/// 比对切片单元
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffChunk {
    /// 差异类型
    pub op: DiffOp,
    /// 对应的字符切片正文
    pub text: String,
}

/// 综合异文互校对比报告
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextDiffResult {
    /// 字符粒度比对切片流
    pub chunks: Vec<DiffChunk>,
    /// 基准底本纯文本
    pub text_a: String,
    /// 参校本纯文本
    pub text_b: String,
    /// 相似度比例 (0.0 ~ 1.0)
    pub similarity: f64,
    /// 统计指标：相同字符数
    pub equal_chars: usize,
    /// 统计指标：底本独有（脱文）字符数
    pub deleted_chars: usize,
    /// 统计指标：校本独有（衍文）字符数
    pub inserted_chars: usize,
}

/// 基于经典 Myers / LCS 动态规划算法的字符级异文互校计算器
/// 针对古籍文言文优化：保留标点并精准高亮传抄异文与药量差异
pub fn diff_texts(text_a: &str, text_b: &str) -> TextDiffResult {
    let chars_a: Vec<char> = text_a.chars().collect();
    let chars_b: Vec<char> = text_b.chars().collect();

    let n = chars_a.len();
    let m = chars_b.len();

    // 极端边界快速短路
    if n == 0 && m == 0 {
        return TextDiffResult {
            chunks: Vec::new(),
            text_a: text_a.to_string(),
            text_b: text_b.to_string(),
            similarity: 1.0,
            equal_chars: 0,
            deleted_chars: 0,
            inserted_chars: 0,
        };
    }

    if n == 0 {
        return TextDiffResult {
            chunks: vec![DiffChunk {
                op: DiffOp::Insert,
                text: text_b.to_string(),
            }],
            text_a: text_a.to_string(),
            text_b: text_b.to_string(),
            similarity: 0.0,
            equal_chars: 0,
            deleted_chars: 0,
            inserted_chars: m,
        };
    }

    if m == 0 {
        return TextDiffResult {
            chunks: vec![DiffChunk {
                op: DiffOp::Delete,
                text: text_a.to_string(),
            }],
            text_a: text_a.to_string(),
            text_b: text_b.to_string(),
            similarity: 0.0,
            equal_chars: 0,
            deleted_chars: n,
            inserted_chars: 0,
        };
    }

    // 动态规划构建 LCS 长度表 (dp[i][j])
    // 为降低大文本开销，使用一维或扁平连续向量
    let mut dp = vec![0u32; (n + 1) * (m + 1)];
    let stride = m + 1;

    for i in 0..n {
        for j in 0..m {
            if chars_a[i] == chars_b[j] {
                dp[(i + 1) * stride + (j + 1)] = dp[i * stride + j] + 1;
            } else {
                let left = dp[(i + 1) * stride + j];
                let up = dp[i * stride + (j + 1)];
                dp[(i + 1) * stride + (j + 1)] = left.max(up);
            }
        }
    }

    // 回溯构造差异序列 (从末尾向前回溯，最后 reverse)
    let mut raw_ops = Vec::new();
    let mut i = n;
    let mut j = m;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && chars_a[i - 1] == chars_b[j - 1] {
            raw_ops.push((DiffOp::Equal, chars_a[i - 1]));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i * stride + j] == dp[i * stride + (j - 1)]) {
            raw_ops.push((DiffOp::Insert, chars_b[j - 1]));
            j -= 1;
        } else if i > 0 {
            raw_ops.push((DiffOp::Delete, chars_a[i - 1]));
            i -= 1;
        }
    }

    raw_ops.reverse();

    // 合并连续同类型操作为 DiffChunk
    let mut chunks: Vec<DiffChunk> = Vec::new();
    let mut equal_chars = 0;
    let mut deleted_chars = 0;
    let mut inserted_chars = 0;

    for (op, ch) in raw_ops {
        match op {
            DiffOp::Equal => equal_chars += 1,
            DiffOp::Delete => deleted_chars += 1,
            DiffOp::Insert => inserted_chars += 1,
        }

        if let Some(last) = chunks.last_mut() {
            if last.op == op {
                last.text.push(ch);
                continue;
            }
        }

        chunks.push(DiffChunk {
            op,
            text: ch.to_string(),
        });
    }

    let total_unique = (equal_chars * 2 + deleted_chars + inserted_chars) as f64;
    let similarity = if total_unique > 0.0 {
        ((equal_chars * 2) as f64) / total_unique
    } else {
        1.0
    };

    TextDiffResult {
        chunks,
        text_a: text_a.to_string(),
        text_b: text_b.to_string(),
        similarity: (similarity * 1000.0).round() / 1000.0,
        equal_chars,
        deleted_chars,
        inserted_chars,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_texts() {
        let text = "桂枝三两，芍药三两，甘草二两。";
        let res = diff_texts(text, text);
        assert_eq!(res.similarity, 1.0);
        assert_eq!(res.chunks.len(), 1);
        assert_eq!(res.chunks[0].op, DiffOp::Equal);
        assert_eq!(res.equal_chars, text.chars().count());
        assert_eq!(res.deleted_chars, 0);
        assert_eq!(res.inserted_chars, 0);
    }

    #[test]
    fn test_herb_dosage_diff() {
        // 赵开美本 vs 桂林古本 药量与加减互校
        let zhao_edition = "桂枝三两（去皮）　芍药三两　甘草二两（炙）";
        let guilin_edition = "桂枝三两　白芍三两　炙甘草二两";

        let res = diff_texts(zhao_edition, guilin_edition);
        assert!(res.similarity > 0.5 && res.similarity < 1.0);
        assert!(res.deleted_chars > 0);
        assert!(res.inserted_chars > 0);

        // 验证 chunks 拼接后还原两端
        let mut reconstructed_a = String::new();
        let mut reconstructed_b = String::new();
        for chunk in &res.chunks {
            match chunk.op {
                DiffOp::Equal => {
                    reconstructed_a.push_str(&chunk.text);
                    reconstructed_b.push_str(&chunk.text);
                }
                DiffOp::Delete => {
                    reconstructed_a.push_str(&chunk.text);
                }
                DiffOp::Insert => {
                    reconstructed_b.push_str(&chunk.text);
                }
            }
        }
        assert_eq!(reconstructed_a, zhao_edition);
        assert_eq!(reconstructed_b, guilin_edition);
    }

    #[test]
    fn test_empty_diff() {
        let res = diff_texts("", "");
        assert_eq!(res.similarity, 1.0);
        assert!(res.chunks.is_empty());
    }
}
