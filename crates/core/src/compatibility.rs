use serde::{Deserialize, Serialize};

/// 配伍禁忌严重等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncompatibilitySeverity {
    /// 严禁配伍 (如十八反、剧毒妊娠禁用)
    Severe,
    /// 慎用配伍 (如十九畏、妊娠慎用)
    Warning,
    /// 提示参考 (相恶降低疗效等)
    Info,
}

/// 禁忌类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncompatibilityType {
    /// 十八反
    ShibaFan,
    /// 十九畏
    ShijiuWei,
    /// 妊娠用药禁忌
    RenshenJinji,
    /// 相恶相杀提示
    XiangE,
}

/// 禁忌匹配结果告警
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityAlert {
    /// 涉及药味 1
    pub herb_a: String,
    /// 涉及药味 2 (若为单药妊娠禁忌则为空)
    pub herb_b: Option<String>,
    /// 禁忌类型
    pub incompatibility_type: IncompatibilityType,
    /// 严重级别
    pub severity: IncompatibilitySeverity,
    /// 歌诀 / 经典出处依据
    pub source_rhyme: String,
    /// 临床药理及风险阐微
    pub explanation: String,
}

/// 静态配伍规则定义
struct RuleItem {
    herb_a: &'static str,
    aliases_a: &'static [&'static str],
    herb_b: &'static str,
    aliases_b: &'static [&'static str],
    incomp_type: IncompatibilityType,
    severity: IncompatibilitySeverity,
    rhyme: &'static str,
    explanation: &'static str,
}

/// 单药禁忌规则 (如妊娠禁用/慎用)
struct SingleHerbRule {
    herb: &'static str,
    aliases: &'static [&'static str],
    severity: IncompatibilitySeverity,
    rhyme: &'static str,
    explanation: &'static str,
}

/// 中药十八反规则表
/// "本草明言十八反，半蒌贝蔹及攻乌，藻戟遂芫俱战草，诸参辛芍叛藜芦。"
const SHIBA_FAN_RULES: &[RuleItem] = &[
    // 乌头反半夏、瓜蒌、贝母、白蔹、白及
    RuleItem {
        herb_a: "乌头",
        aliases_a: &["川乌", "草乌", "附子", "天雄", "乌头"],
        herb_b: "半夏",
        aliases_b: &["半夏", "法半夏", "姜半夏", "清半夏", "竹沥半夏"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "半蒌贝蔹及攻乌",
        explanation: "乌头（附子）大辛大热纯阳，半夏辛温化痰，两者配伍毒性剧增，历代本草列为相反第一大忌。",
    },
    RuleItem {
        herb_a: "乌头",
        aliases_a: &["川乌", "草乌", "附子", "天雄", "乌头"],
        herb_b: "瓜蒌",
        aliases_b: &["瓜蒌", "全瓜蒌", "瓜蒌皮", "瓜蒌仁", "天花粉", "花粉"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "半蒌贝蔹及攻乌",
        explanation: "乌头辛热温经散寒，瓜蒌（天花粉）苦寒清润生津，寒热相反且药性相激，易增心律毒性。",
    },
    RuleItem {
        herb_a: "乌头",
        aliases_a: &["川乌", "草乌", "附子", "天雄", "乌头"],
        herb_b: "贝母",
        aliases_b: &["川贝母", "浙贝母", "贝母", "土贝母"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "半蒌贝蔹及攻乌",
        explanation: "贝母微寒润肺止咳，与大热大毒之川草乌、附子同用相冲突，严重降低乌头碱代谢并加剧毒副作用。",
    },
    RuleItem {
        herb_a: "乌头",
        aliases_a: &["川乌", "草乌", "附子", "天雄", "乌头"],
        herb_b: "白蔹",
        aliases_b: &["白蔹"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "半蒌贝蔹及攻乌",
        explanation: "白蔹苦寒清热消肿，与辛烈温阳之乌头相配容易发生拮抗与剧烈胃肠毒副反应。",
    },
    RuleItem {
        herb_a: "乌头",
        aliases_a: &["川乌", "草乌", "附子", "天雄", "乌头"],
        herb_b: "白及",
        aliases_b: &["白及", "白芨"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "半蒌贝蔹及攻乌",
        explanation: "白及质粘涩收敛生肌止血，乌头辛燥走窜搜风逐寒，两药药性相违走守失据。",
    },
    // 甘草反海藻、大戟、甘遂、芫花
    RuleItem {
        herb_a: "甘草",
        aliases_a: &["甘草", "炙甘草", "甘草片"],
        herb_b: "海藻",
        aliases_b: &["海藻"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "藻戟遂芫俱战草",
        explanation: "甘草补脾益气缓急，海藻消痰软坚泄水利尿。两者同用常使心肌收缩异常并加重电解质紊乱。",
    },
    RuleItem {
        herb_a: "甘草",
        aliases_a: &["甘草", "炙甘草", "甘草片"],
        herb_b: "大戟",
        aliases_b: &["大戟", "京大戟", "红大戟"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "藻戟遂芫俱战草",
        explanation: "甘草能缓和药性，大戟为峻下逐水峻烈药。甘草与大戟同用破坏泻下作用并显著诱发肠胃剧毒。",
    },
    RuleItem {
        herb_a: "甘草",
        aliases_a: &["甘草", "炙甘草", "甘草片"],
        herb_b: "甘遂",
        aliases_b: &["甘遂"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "藻戟遂芫俱战草",
        explanation: "甘遂专逐经隧之水湿，甘草助湿且滞药力，二者合用毒性猛烈增高，古人戒惧甚严。",
    },
    RuleItem {
        herb_a: "甘草",
        aliases_a: &["甘草", "炙甘草", "甘草片"],
        herb_b: "芫花",
        aliases_b: &["芫花", "醋芫花"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "藻戟遂芫俱战草",
        explanation: "芫花攻水饮宿痰，甘草与其同服产生强烈的胃肠黏膜刺激与神经毒性。",
    },
    // 藜芦反诸参、细辛、芍药
    RuleItem {
        herb_a: "藜芦",
        aliases_a: &["藜芦"],
        herb_b: "诸参",
        aliases_b: &["人参", "党参", "红参", "西洋参", "南沙参", "北沙参", "丹参", "玄参", "苦参"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "诸参辛芍叛藜芦",
        explanation: "藜芦催吐有大毒，人参、沙参、丹参、玄参等诸参补虚扶正或养阴活血，合用引发生命危险。",
    },
    RuleItem {
        herb_a: "藜芦",
        aliases_a: &["藜芦"],
        herb_b: "细辛",
        aliases_b: &["细辛"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "诸参辛芍叛藜芦",
        explanation: "细辛辛香通窍温肺，与大毒之藜芦同用，促使毒性成分极速入络中枢引发中毒。",
    },
    RuleItem {
        herb_a: "藜芦",
        aliases_a: &["藜芦"],
        herb_b: "芍药",
        aliases_b: &["白芍", "赤芍", "芍药"],
        incomp_type: IncompatibilityType::ShibaFan,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "诸参辛芍叛藜芦",
        explanation: "芍药酸寒柔肝敛阴，藜芦辛烈涌吐，阴阳升降完全背道相冲。",
    },
];

/// 中药十九畏规则表
/// "硫黄原是火中精，朴硝一见便相争。水银莫与砒霜见，狼毒最怕密陀僧..."
const SHIJIU_WEI_RULES: &[RuleItem] = &[
    RuleItem {
        herb_a: "硫黄",
        aliases_a: &["硫黄", "硫磺"],
        herb_b: "朴硝",
        aliases_b: &["朴硝", "芒硝", "玄明粉", "马牙硝"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Warning,
        rhyme: "硫黄原是火中精，朴硝一见便相争",
        explanation: "硫黄辛热补火助阳，朴硝苦寒润燥泄热，二药水火悬殊，同处一剂易激化药力相伤。",
    },
    RuleItem {
        herb_a: "水银",
        aliases_a: &["水银"],
        herb_b: "砒霜",
        aliases_b: &["砒霜", "砒石", "红砒", "白砒"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Severe,
        rhyme: "水银莫与砒霜见",
        explanation: "水银与砒霜皆为剧烈含重金属毒性矿物药，二毒相搏易析出高毒可溶物，严禁同用。",
    },
    RuleItem {
        herb_a: "狼毒",
        aliases_a: &["狼毒"],
        herb_b: "密陀僧",
        aliases_b: &["密陀僧"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Warning,
        rhyme: "狼毒最怕密陀僧",
        explanation: "狼毒辛烈破积攻毒，密陀僧含铅质重收湿，配伍相畏削减攻效且增铅毒蓄积。",
    },
    RuleItem {
        herb_a: "巴豆",
        aliases_a: &["巴豆", "巴豆霜"],
        herb_b: "牵牛子",
        aliases_b: &["牵牛子", "黑丑", "白丑"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Warning,
        rhyme: "巴豆性烈最为急，牵牛逢之即不宜",
        explanation: "巴豆大辛大热走肠胃峻下，牵牛苦寒行水攻下，寒热峻泄同用损伤脾胃元气极甚。",
    },
    RuleItem {
        herb_a: "丁香",
        aliases_a: &["丁香", "公丁香", "母丁香"],
        herb_b: "郁金",
        aliases_b: &["郁金", "广郁金", "川郁金"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Warning,
        rhyme: "丁香莫与郁金见",
        explanation: "丁香温中降逆，郁金行气凉血开郁，二药芳香走窜性相逆行，相畏削弱疏导之功。",
    },
    RuleItem {
        herb_a: "人参",
        aliases_a: &["人参", "红参", "党参", "西洋参"],
        herb_b: "五灵脂",
        aliases_b: &["五灵脂"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Warning,
        rhyme: "官桂善能调冷气，若逢石脂便相欺；人参最怕五灵脂",
        explanation:
            "人参大补元气健脾养胃，五灵脂行气活血散瘀止痛。古人认为同用相互克制降低补气之效。",
    },
    RuleItem {
        herb_a: "肉桂",
        aliases_a: &["肉桂", "官桂", "桂心"],
        herb_b: "赤石脂",
        aliases_b: &["赤石脂"],
        incomp_type: IncompatibilityType::ShijiuWei,
        severity: IncompatibilitySeverity::Warning,
        rhyme: "官桂善能调冷气，若逢石脂便相欺",
        explanation: "肉桂辛热温运通脉，赤石脂甘温涩肠固下，相配石脂沉涩滞遏肉桂温行之力。",
    },
];

/// 妊娠禁用与慎用名录
const PREGNANCY_RULES: &[SingleHerbRule] = &[
    SingleHerbRule {
        herb: "麝香",
        aliases: &["麝香", "当门子"],
        severity: IncompatibilitySeverity::Severe,
        rhyme: "妊娠禁用：大毒走窜催产之品",
        explanation: "麝香芳香走窜通经达络开窍，极易刺激子宫强烈收缩诱发流产，孕妇绝对禁用。",
    },
    SingleHerbRule {
        herb: "巴豆",
        aliases: &["巴豆", "巴豆霜"],
        severity: IncompatibilitySeverity::Severe,
        rhyme: "妊娠禁用：峻烈下胎之品",
        explanation: "巴豆大热大毒峻下攻积，易伤胎元致堕胎，孕妇严禁沾染。",
    },
    SingleHerbRule {
        herb: "牵牛子",
        aliases: &["牵牛子", "黑丑", "白丑"],
        severity: IncompatibilitySeverity::Severe,
        rhyme: "妊娠禁用：峻下破水之品",
        explanation: "破水通便走下窍，动血下胎，孕妇禁用。",
    },
    SingleHerbRule {
        herb: "水蛭",
        aliases: &["水蛭", "蚂蟥"],
        severity: IncompatibilitySeverity::Severe,
        rhyme: "妊娠禁用：破血逐瘀之品",
        explanation: "入血分破顽血逐瘀积，对胚胎着床具有强力破坏作用，孕妇禁用。",
    },
    SingleHerbRule {
        herb: "虻虫",
        aliases: &["虻虫"],
        severity: IncompatibilitySeverity::Severe,
        rhyme: "妊娠禁用：破血削坚之品",
        explanation: "破血行瘀通经，易损伤胎元引起大出血流产，孕妇禁用。",
    },
    SingleHerbRule {
        herb: "桃仁",
        aliases: &["桃仁"],
        severity: IncompatibilitySeverity::Warning,
        rhyme: "妊娠慎用：活血通经之品",
        explanation: "苦甘平活血祛瘀润肠，具破血行经之力，非血瘀实证妊娠期慎用。",
    },
    SingleHerbRule {
        herb: "红花",
        aliases: &["红花", "藏红花", "西红花"],
        severity: IncompatibilitySeverity::Warning,
        rhyme: "妊娠慎用：活血动血之品",
        explanation: "辛温活血通经散瘀，用量稍大即可刺激子宫收缩，妊娠期慎用。",
    },
    SingleHerbRule {
        herb: "附子",
        aliases: &["附子", "黑顺片", "白附片"],
        severity: IncompatibilitySeverity::Warning,
        rhyme: "妊娠慎用：大辛大热走散之品",
        explanation: "大辛大热性猛有毒，易动胎气并诱发虚脱，非危重虚寒亡阳证慎用。",
    },
];

/// 检查传入的一组药味列表中是否存在配伍禁忌
pub fn check_herb_compatibility(herbs: &[String]) -> Vec<CompatibilityAlert> {
    let mut alerts = Vec::new();
    if herbs.is_empty() {
        return alerts;
    }

    // 规范化药味名，并去重
    let cleaned: Vec<String> = herbs
        .iter()
        .map(|h| h.trim().to_string())
        .filter(|h| !h.is_empty())
        .collect();

    // 辅助闭包：判断输入列表中是否命中规则中的特定药味
    let matches_herb = |input: &str, target: &str, aliases: &[&str]| -> bool {
        if input == target || input.contains(target) || target.contains(input) {
            return true;
        }
        for alias in aliases {
            if input == *alias || input.contains(*alias) || alias.contains(input) {
                return true;
            }
        }
        false
    };

    // 1. 检查两两配伍禁忌 (十八反与十九畏)
    let all_pair_rules: Vec<&RuleItem> = SHIBA_FAN_RULES
        .iter()
        .chain(SHIJIU_WEI_RULES.iter())
        .collect();

    for rule in all_pair_rules {
        let mut hit_a = None;
        let mut hit_b = None;

        for herb in &cleaned {
            if hit_a.is_none() && matches_herb(herb, rule.herb_a, rule.aliases_a) {
                hit_a = Some(herb.clone());
            }
            if hit_b.is_none() && matches_herb(herb, rule.herb_b, rule.aliases_b) {
                hit_b = Some(herb.clone());
            }
        }

        // 当药味 A 与药味 B 同时存在于方中时发出警报
        if let (Some(a), Some(b)) = (hit_a, hit_b) {
            alerts.push(CompatibilityAlert {
                herb_a: a,
                herb_b: Some(b),
                incompatibility_type: rule.incomp_type,
                severity: rule.severity,
                source_rhyme: rule.rhyme.to_string(),
                explanation: rule.explanation.to_string(),
            });
        }
    }

    // 2. 检查单药妊娠禁忌
    for rule in PREGNANCY_RULES {
        for herb in &cleaned {
            if matches_herb(herb, rule.herb, rule.aliases) {
                alerts.push(CompatibilityAlert {
                    herb_a: herb.clone(),
                    herb_b: None,
                    incompatibility_type: IncompatibilityType::RenshenJinji,
                    severity: rule.severity,
                    source_rhyme: rule.rhyme.to_string(),
                    explanation: rule.explanation.to_string(),
                });
                break;
            }
        }
    }

    alerts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shiba_fan_detection() {
        let formula = vec!["川乌".to_string(), "半夏".to_string(), "白术".to_string()];
        let alerts = check_herb_compatibility(&formula);
        assert!(!alerts.is_empty());
        assert_eq!(
            alerts[0].incompatibility_type,
            IncompatibilityType::ShibaFan
        );
        assert_eq!(alerts[0].severity, IncompatibilitySeverity::Severe);
    }

    #[test]
    fn test_shijiu_wei_detection() {
        let formula = vec!["人参".to_string(), "五灵脂".to_string(), "当归".to_string()];
        let alerts = check_herb_compatibility(&formula);
        assert!(!alerts.is_empty());
        assert_eq!(
            alerts[0].incompatibility_type,
            IncompatibilityType::ShijiuWei
        );
    }

    #[test]
    fn test_pregnancy_contraindication() {
        let formula = vec!["当归".to_string(), "麝香".to_string()];
        let alerts = check_herb_compatibility(&formula);
        assert!(!alerts.is_empty());
        let hit_renshen = alerts
            .iter()
            .any(|a| a.incompatibility_type == IncompatibilityType::RenshenJinji);
        assert!(hit_renshen);
    }

    #[test]
    fn test_safe_formula() {
        let formula = vec![
            "桂枝".to_string(),
            "白芍".to_string(),
            "生姜".to_string(),
            "大枣".to_string(),
            "甘草".to_string(),
        ];
        let alerts = check_herb_compatibility(&formula);
        assert!(alerts.is_empty());
    }
}
