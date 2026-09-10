use serde::{Deserialize, Serialize};

/// 经络阴阳与流注大类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeridianCategory {
    /// 手三阴经 (肺、心包、心)
    ShouSanYin,
    /// 手三阳经 (大肠、三焦、小肠)
    ShouSanYang,
    /// 足三阳经 (胃、胆、膀胱)
    ZuSanYang,
    /// 足三阴经 (脾、肝、肾)
    ZuSanYin,
    /// 奇经八脉 (任脉、督脉等)
    QiJingBaMai,
}

/// 特定穴分类属性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecificAcupointType {
    /// 五输穴 (井、荥、输、经、合)
    WuShuXue,
    /// 原穴
    YuanXue,
    /// 络穴
    LuoXue,
    /// 郄穴
    XiXue,
    /// 背俞穴
    BeiShuXue,
    /// 募穴
    MuXue,
    /// 八会穴
    BaHuiXue,
    /// 八脉交会穴
    BaMaiJiaoHuiXue,
    /// 下合穴
    XiaHeXue,
    /// 四总穴 / 保健要穴
    SiZongXue,
    /// 常规经穴
    Normal,
}

/// 穴位临床数据模型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcupointInfo {
    /// 穴名 (如 "足三里")
    pub name: String,
    /// 国际标准代码 (如 "ST36")
    pub code: String,
    /// 所属经络 (如 "足阳明胃经")
    pub meridian_name: String,
    /// 解剖与骨度分寸定位
    pub location: String,
    /// 经典溯源 (如《灵枢·本输》)
    pub origin_classic: String,
    /// 特定穴属性
    pub specific_types: Vec<SpecificAcupointType>,
    /// 特定穴标签展示 (如 ["合穴", "胃下合穴", "四总穴", "回阳九针穴"])
    pub specific_tags: Vec<String>,
    /// 临床主治证病机
    pub indications: Vec<String>,
    /// 针灸刺灸法操作与禁忌
    pub manipulation: String,
    /// 交互拓扑在经络循行上的相对分寸位置 (0.0 起始 ~ 1.0 终止)
    pub flow_position: f64,
}

/// 经脉数据模型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeridianInfo {
    /// 经脉名称 (如 "手太阴肺经")
    pub name: String,
    /// 英文简写代码 (如 "LU")
    pub code: String,
    /// 经络大类
    pub category: MeridianCategory,
    /// 五行属性 (木、火、土、金、水、相火等)
    pub element: String,
    /// 脏腑表里配对经脉 (如 "手阳明大肠经")
    pub paired_meridian: String,
    /// 经脉气血旺衰子午流注时辰 (如 "寅时 (03:00 - 05:00)")
    pub peak_time: String,
    /// 经脉循行走向总括
    pub course_description: String,
    /// 经脉收录重点穴位集合
    pub acupoints: Vec<AcupointInfo>,
}

/// 全量十四经脉与核心特定穴知识库数据构建
pub fn get_meridian_knowledge_base() -> Vec<MeridianInfo> {
    vec![
        // 1. 手太阴肺经 (LU)
        MeridianInfo {
            name: "手太阴肺经".to_string(),
            code: "LU".to_string(),
            category: MeridianCategory::ShouSanYin,
            element: "金".to_string(),
            paired_meridian: "手阳明大肠经".to_string(),
            peak_time: "寅时 (03:00 - 05:00)".to_string(),
            course_description: "起于中焦，下络大肠，还循胃口，上膈属肺，从肺系横出腋下，下循臑内，行少阴心主之前，下肘中，循臂内上骨下廉，入寸口，上鱼，循鱼际，出大指之端。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "中府".to_string(),
                    code: "LU1".to_string(),
                    meridian_name: "手太阴肺经".to_string(),
                    location: "在胸部，横平第1肋间隙，锁骨下窝外侧，前正中线旁开6寸。".to_string(),
                    origin_classic: "《素问·水热穴论》".to_string(),
                    specific_types: vec![SpecificAcupointType::MuXue],
                    specific_tags: vec!["肺之募穴".to_string(), "手足太阴经交会穴".to_string()],
                    indications: vec!["咳嗽".to_string(), "气喘".to_string(), "胸痛".to_string(), "肩背痛".to_string()],
                    manipulation: "向外斜刺或平刺0.5-0.8寸，不可向内深刺，以免伤及肺脏引起气胸。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "尺泽".to_string(),
                    code: "LU5".to_string(),
                    meridian_name: "手太阴肺经".to_string(),
                    location: "在肘区，肘横纹上，肱二头肌腱桡侧凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["合穴 (水)".to_string()],
                    indications: vec!["咳嗽".to_string(), "气喘".to_string(), "咯血".to_string(), "潮热".to_string(), "咽喉肿痛".to_string(), "肘臂挛痛".to_string()],
                    manipulation: "直刺0.8-1.2寸，或点刺出血。".to_string(),
                    flow_position: 0.45,
                },
                AcupointInfo {
                    name: "孔最".to_string(),
                    code: "LU6".to_string(),
                    meridian_name: "手太阴肺经".to_string(),
                    location: "在前臂前区，腕掌侧远端横纹上7寸，尺泽与太渊的连线上。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::XiXue],
                    specific_tags: vec!["郄穴".to_string()],
                    indications: vec!["咯血".to_string(), "咳嗽".to_string(), "气喘".to_string(), "咽喉肿痛".to_string(), "肘臂挛痛".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.60,
                },
                AcupointInfo {
                    name: "列缺".to_string(),
                    code: "LU7".to_string(),
                    meridian_name: "手太阴肺经".to_string(),
                    location: "在前臂前区，桡骨茎突上方，腕掌侧远端横纹上1.5寸，肱桡肌与拇长展肌腱之间。".to_string(),
                    origin_classic: "《灵枢·经脉》".to_string(),
                    specific_types: vec![SpecificAcupointType::LuoXue, SpecificAcupointType::BaMaiJiaoHuiXue, SpecificAcupointType::SiZongXue],
                    specific_tags: vec!["络穴".to_string(), "八脉交会穴 (通任脉)".to_string(), "四总穴 (头项寻列缺)".to_string()],
                    indications: vec!["头痛".to_string(), "项强".to_string(), "咳嗽".to_string(), "气喘".to_string(), "咽喉肿痛".to_string(), "口眼㖞斜".to_string()],
                    manipulation: "向上斜刺0.5-0.8寸。".to_string(),
                    flow_position: 0.75,
                },
                AcupointInfo {
                    name: "太渊".to_string(),
                    code: "LU9".to_string(),
                    meridian_name: "手太阴肺经".to_string(),
                    location: "在腕前区，桡骨茎突与舟状骨之间，拇长展肌腱尺侧凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue, SpecificAcupointType::BaHuiXue],
                    specific_tags: vec!["输穴 (土)".to_string(), "原穴".to_string(), "脉会太渊".to_string()],
                    indications: vec!["咳嗽".to_string(), "气喘".to_string(), "无脉症".to_string(), "胸痹".to_string(), "腕痛".to_string()],
                    manipulation: "避开桡动脉，直刺0.3-0.5寸。".to_string(),
                    flow_position: 0.90,
                },
                AcupointInfo {
                    name: "少商".to_string(),
                    code: "LU11".to_string(),
                    meridian_name: "手太阴肺经".to_string(),
                    location: "在手指，拇指末节桡侧，指甲根角侧上方0.1寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (木)".to_string(), "十三鬼穴".to_string()],
                    indications: vec!["咽喉肿痛".to_string(), "高热昏迷".to_string(), "癫狂".to_string(), "中暑".to_string(), "小儿惊风".to_string()],
                    manipulation: "浅刺0.1寸，或三棱针点刺出血。".to_string(),
                    flow_position: 1.00,
                },
            ],
        },

        // 2. 手阳明大肠经 (LI)
        MeridianInfo {
            name: "手阳明大肠经".to_string(),
            code: "LI".to_string(),
            category: MeridianCategory::ShouSanYang,
            element: "金".to_string(),
            paired_meridian: "手太阴肺经".to_string(),
            peak_time: "卯时 (05:00 - 07:00)".to_string(),
            course_description: "起于大指次指之端，循指上廉，出合谷两骨之间，上入两筋之中，循臂上廉，入肘外廉，上臑外前廉，上肩，出髃骨之前廉，上出于柱骨之会上，下入缺盆，络肺，下膈，属大肠。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "商阳".to_string(),
                    code: "LI1".to_string(),
                    meridian_name: "手阳明大肠经".to_string(),
                    location: "在手指，食指末节桡侧，指甲根角侧上方0.1寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (金)".to_string()],
                    indications: vec!["齿痛".to_string(), "咽喉肿痛".to_string(), "高热昏迷".to_string(), "热病无汗".to_string()],
                    manipulation: "浅刺0.1寸，或点刺出血。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "合谷".to_string(),
                    code: "LI4".to_string(),
                    meridian_name: "手阳明大肠经".to_string(),
                    location: "在手背，第2掌骨桡侧的中点处。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue, SpecificAcupointType::SiZongXue],
                    specific_tags: vec!["原穴".to_string(), "四总穴 (面口合谷收)".to_string(), "四关穴".to_string()],
                    indications: vec!["头痛".to_string(), "齿痛".to_string(), "目赤肿痛".to_string(), "咽喉肿痛".to_string(), "口眼㖞斜".to_string(), "发热恶寒".to_string(), "滞产".to_string()],
                    manipulation: "直刺0.5-1.0寸。孕妇禁针，易促子宫收缩致流产。".to_string(),
                    flow_position: 0.20,
                },
                AcupointInfo {
                    name: "曲池".to_string(),
                    code: "LI11".to_string(),
                    meridian_name: "手阳明大肠经".to_string(),
                    location: "在肘区，尺泽与肱骨外上髁连线中点凹陷处。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["合穴 (土)".to_string(), "清热解表要穴".to_string()],
                    indications: vec!["发热".to_string(), "高血压".to_string(), "咽喉肿痛".to_string(), "齿痛".to_string(), "湿疹".to_string(), "瘾疹".to_string(), "上肢不遂".to_string()],
                    manipulation: "直刺1.0-1.5寸。".to_string(),
                    flow_position: 0.55,
                },
                AcupointInfo {
                    name: "肩髃".to_string(),
                    code: "LI15".to_string(),
                    meridian_name: "手阳明大肠经".to_string(),
                    location: "在三角肌区，肩峰外侧缘前端与肱骨大结节两骨间凹陷中。".to_string(),
                    origin_classic: "《灵枢·经脉》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["手阳明与阳跷脉交会穴".to_string()],
                    indications: vec!["肩臂挛痛".to_string(), "上肢不遂".to_string(), "风疹".to_string()],
                    manipulation: "直刺或向下斜刺0.8-1.5寸。".to_string(),
                    flow_position: 0.75,
                },
                AcupointInfo {
                    name: "迎香".to_string(),
                    code: "LI20".to_string(),
                    meridian_name: "手阳明大肠经".to_string(),
                    location: "在面部，鼻翼外缘中点旁，鼻唇沟中。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["手足阳明交会穴".to_string(), "鼻渊要穴".to_string()],
                    indications: vec!["鼻塞".to_string(), "鼽衄".to_string(), "口㖞".to_string(), "胆道蛔虫症".to_string()],
                    manipulation: "略向内上方斜刺或平刺0.3-0.5寸。".to_string(),
                    flow_position: 1.00,
                },
            ],
        },

        // 3. 足阳明胃经 (ST)
        MeridianInfo {
            name: "足阳明胃经".to_string(),
            code: "ST".to_string(),
            category: MeridianCategory::ZuSanYang,
            element: "土".to_string(),
            paired_meridian: "足太阴脾经".to_string(),
            peak_time: "辰时 (07:00 - 09:00)".to_string(),
            course_description: "起于鼻，交頞中，旁约太阳之脉，下循鼻外，入上齿中，还出挟口，环唇，下交承浆，却循颐后下廉，出大迎，循颊车，上耳前，过客主人，循发际，至额颅。支者下人迎，循喉咙，入缺盆，下膈，属胃，络脾。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "天枢".to_string(),
                    code: "ST25".to_string(),
                    meridian_name: "足阳明胃经".to_string(),
                    location: "在腹部，横平脐中，前正中线旁开2寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::MuXue],
                    specific_tags: vec!["大肠之募穴".to_string()],
                    indications: vec!["腹胀".to_string(), "泄泻".to_string(), "便秘".to_string(), "痢疾".to_string(), "月经不调".to_string()],
                    manipulation: "直刺1.0-1.5寸。".to_string(),
                    flow_position: 0.55,
                },
                AcupointInfo {
                    name: "足三里".to_string(),
                    code: "ST36".to_string(),
                    meridian_name: "足阳明胃经".to_string(),
                    location: "在小腿前外侧，犊鼻下3寸，犊鼻与解溪连线上。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::XiaHeXue, SpecificAcupointType::SiZongXue],
                    specific_tags: vec!["合穴 (土)".to_string(), "胃下合穴".to_string(), "四总穴 (肚腹三里留)".to_string(), "回阳九针穴".to_string()],
                    indications: vec!["胃痛".to_string(), "腹胀".to_string(), "呕吐".to_string(), "泄泻".to_string(), "下肢痿痹".to_string(), "虚劳羸瘦".to_string()],
                    manipulation: "直刺1.0-2.0寸。强壮保健常用温灸。".to_string(),
                    flow_position: 0.80,
                },
                AcupointInfo {
                    name: "丰隆".to_string(),
                    code: "ST40".to_string(),
                    meridian_name: "足阳明胃经".to_string(),
                    location: "在小腿前外侧，外踝尖上8寸，胫骨前肌外缘，距胫骨前缘2横指。".to_string(),
                    origin_classic: "《灵枢·经脉》".to_string(),
                    specific_types: vec![SpecificAcupointType::LuoXue],
                    specific_tags: vec!["络穴".to_string(), "化痰要穴".to_string()],
                    indications: vec!["痰多".to_string(), "哮喘".to_string(), "咳嗽".to_string(), "头痛".to_string(), "眩晕".to_string(), "癫狂".to_string(), "下肢瘫痪".to_string()],
                    manipulation: "直刺1.0-1.5寸。".to_string(),
                    flow_position: 0.88,
                },
                AcupointInfo {
                    name: "内庭".to_string(),
                    code: "ST44".to_string(),
                    meridian_name: "足阳明胃经".to_string(),
                    location: "在足背，第2、第3趾间，趾蹼缘后方赤白肉际处。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["荥穴 (水)".to_string(), "泻胃火要穴".to_string()],
                    indications: vec!["齿痛".to_string(), "咽喉肿痛".to_string(), "口㖞".to_string(), "热病".to_string(), "吐酸".to_string(), "腹泻".to_string()],
                    manipulation: "直刺或斜刺0.5-0.8寸。".to_string(),
                    flow_position: 0.98,
                },
            ],
        },

        // 4. 足太阴脾经 (SP)
        MeridianInfo {
            name: "足太阴脾经".to_string(),
            code: "SP".to_string(),
            category: MeridianCategory::ZuSanYin,
            element: "土".to_string(),
            paired_meridian: "足阳明胃经".to_string(),
            peak_time: "巳时 (09:00 - 11:00)".to_string(),
            course_description: "起于大指之端，循指内侧白肉际，过核骨后，上内踝前廉，上踹内，循胫骨后，交出厥阴之前，上膝股内前廉，入腹，属脾，络胃，上膈，挟咽，连舌本，散舌下。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "太白".to_string(),
                    code: "SP3".to_string(),
                    meridian_name: "足太阴脾经".to_string(),
                    location: "在跖区，第1跖趾关节近端赤白肉际凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue],
                    specific_tags: vec!["输穴 (土)".to_string(), "原穴".to_string()],
                    indications: vec!["胃痛".to_string(), "腹胀".to_string(), "肠鸣".to_string(), "泄泻".to_string(), "痢疾".to_string(), "身重".to_string()],
                    manipulation: "直刺0.5-0.8寸。".to_string(),
                    flow_position: 0.15,
                },
                AcupointInfo {
                    name: "三阴交".to_string(),
                    code: "SP6".to_string(),
                    meridian_name: "足太阴脾经".to_string(),
                    location: "在小腿内侧，内踝尖上3寸，胫骨内侧缘后际。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["足三阴经交会穴".to_string(), "妇科第一要穴".to_string()],
                    indications: vec!["月经不调".to_string(), "崩漏".to_string(), "带下".to_string(), "痛经".to_string(), "遗精".to_string(), "阳痿".to_string(), "小便不利".to_string(), "失眠".to_string()],
                    manipulation: "直刺1.0-1.5寸。孕妇禁针。".to_string(),
                    flow_position: 0.30,
                },
                AcupointInfo {
                    name: "阴陵泉".to_string(),
                    code: "SP9".to_string(),
                    meridian_name: "足太阴脾经".to_string(),
                    location: "在小腿内侧，胫骨内侧髁下缘与胫骨内侧缘之间的凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["合穴 (水)".to_string(), "利水祛湿要穴".to_string()],
                    indications: vec!["腹胀".to_string(), "水肿".to_string(), "小便不利".to_string(), "遗溺".to_string(), "泄泻".to_string(), "膝痛".to_string()],
                    manipulation: "直刺1.0-2.0寸。".to_string(),
                    flow_position: 0.45,
                },
            ],
        },

        // 5. 督脉 (GV)
        MeridianInfo {
            name: "督脉".to_string(),
            code: "GV".to_string(),
            category: MeridianCategory::QiJingBaMai,
            element: "阳".to_string(),
            paired_meridian: "任脉".to_string(),
            peak_time: "午时 / 阳气极盛".to_string(),
            course_description: "起于少腹以下骨中央，入属于脑，上巅，循额至鼻柱，总督一身之阳经。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "命门".to_string(),
                    code: "GV4".to_string(),
                    meridian_name: "督脉".to_string(),
                    location: "在脊柱区，第2腰椎棘突下凹陷中，后正中线上。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["温补肾阳要穴".to_string()],
                    indications: vec!["腰脊强痛".to_string(), "遗精".to_string(), "阳痿".to_string(), "泄泻".to_string(), "下肢痿痹".to_string()],
                    manipulation: "向上斜刺0.5-1.0寸，多用艾灸温煦。".to_string(),
                    flow_position: 0.20,
                },
                AcupointInfo {
                    name: "大椎".to_string(),
                    code: "GV14".to_string(),
                    meridian_name: "督脉".to_string(),
                    location: "在脊柱区，第7颈椎棘突下凹陷中，后正中线上。".to_string(),
                    origin_classic: "《素问·气府论》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["三阳五会".to_string(), "手足三阳及督脉交会穴".to_string(), "退热解表要穴".to_string()],
                    indications: vec!["热病".to_string(), "感冒".to_string(), "咳嗽".to_string(), "气喘".to_string(), "骨蒸潮热".to_string(), "癫痫".to_string()],
                    manipulation: "向上斜刺0.5-1.0寸。刺入不宜过深，防止伤及脊髓。".to_string(),
                    flow_position: 0.65,
                },
                AcupointInfo {
                    name: "百会".to_string(),
                    code: "GV20".to_string(),
                    meridian_name: "督脉".to_string(),
                    location: "在头部，前发际正中直上5寸，两耳尖连线中点处。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["三阳五会".to_string(), "升阳举陷要穴".to_string()],
                    indications: vec!["头痛".to_string(), "眩晕".to_string(), "脱肛".to_string(), "子宫脱垂".to_string(), "失眠".to_string(), "健忘".to_string(), "中风失语".to_string()],
                    manipulation: "平刺0.5-0.8寸。".to_string(),
                    flow_position: 0.85,
                },
            ],
        },

        // 6. 任脉 (CV)
        MeridianInfo {
            name: "任脉".to_string(),
            code: "CV".to_string(),
            category: MeridianCategory::QiJingBaMai,
            element: "阴".to_string(),
            paired_meridian: "督脉".to_string(),
            peak_time: "子时 / 阴气极盛".to_string(),
            course_description: "起于中极之下，以上毛际，循腹里，上关元，至咽喉，上颐循面入目，总任一身之阴经。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "关元".to_string(),
                    code: "CV4".to_string(),
                    meridian_name: "任脉".to_string(),
                    location: "在下腹部，前正中线上，脐中下3寸。".to_string(),
                    origin_classic: "《素问·气府论》".to_string(),
                    specific_types: vec![SpecificAcupointType::MuXue],
                    specific_tags: vec!["小肠之募穴".to_string(), "培元固本要穴".to_string(), "回阳九针穴".to_string()],
                    indications: vec!["虚劳羸瘦".to_string(), "少腹疼痛".to_string(), "遗尿".to_string(), "尿闭".to_string(), "遗精".to_string(), "阳痿".to_string(), "痛经".to_string(), "闭经".to_string()],
                    manipulation: "直刺1.0-1.5寸。需排空膀胱后进针。孕妇慎用。".to_string(),
                    flow_position: 0.16,
                },
                AcupointInfo {
                    name: "中脘".to_string(),
                    code: "CV12".to_string(),
                    meridian_name: "任脉".to_string(),
                    location: "在上腹部，前正中线上，脐中上4寸。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::MuXue, SpecificAcupointType::BaHuiXue],
                    specific_tags: vec!["胃之募穴".to_string(), "腑会中脘".to_string()],
                    indications: vec!["胃痛".to_string(), "腹胀".to_string(), "纳呆".to_string(), "呕吐".to_string(), "反胃".to_string(), "泄泻".to_string(), "痢疾".to_string()],
                    manipulation: "直刺1.0-1.5寸。".to_string(),
                    flow_position: 0.50,
                },
                AcupointInfo {
                    name: "膻中".to_string(),
                    code: "CV17".to_string(),
                    meridian_name: "任脉".to_string(),
                    location: "在胸部，横平第4肋间隙，前正中线上。".to_string(),
                    origin_classic: "《灵枢·根结》".to_string(),
                    specific_types: vec![SpecificAcupointType::MuXue, SpecificAcupointType::BaHuiXue],
                    specific_tags: vec!["心包之募穴".to_string(), "气会膻中".to_string()],
                    indications: vec!["咳嗽".to_string(), "气喘".to_string(), "胸痛".to_string(), "心悸".to_string(), "噎膈".to_string(), "产后少乳".to_string()],
                    manipulation: "平刺0.3-0.5寸。".to_string(),
                    flow_position: 0.70,
                },
            ],
        },
    ]
}

/// 根据穴位名称或别名查找穴位详情
pub fn find_acupoint(name: &str) -> Option<AcupointInfo> {
    let clean = name.trim();
    for meridian in get_meridian_knowledge_base() {
        for point in meridian.acupoints {
            if point.name == clean || point.code.eq_ignore_ascii_case(clean) {
                return Some(point);
            }
        }
    }
    None
}

/// 依据临床病症症状快速智能推荐对偶穴位组合 (穴位处方)
pub fn recommend_acupoints_for_symptom(symptom: &str) -> Vec<AcupointInfo> {
    let clean = symptom.trim();
    let mut hits = Vec::new();

    for meridian in get_meridian_knowledge_base() {
        for point in meridian.acupoints {
            if point
                .indications
                .iter()
                .any(|ind| ind.contains(clean) || clean.contains(ind))
            {
                hits.push(point);
            }
        }
    }

    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meridian_knowledge_base_load() {
        let base = get_meridian_knowledge_base();
        assert!(!base.is_empty());
        assert!(base.iter().any(|m| m.name == "足阳明胃经"));
    }

    #[test]
    fn test_find_acupoint() {
        let zusanli = find_acupoint("足三里");
        assert!(zusanli.is_some());
        let pt = zusanli.unwrap();
        assert_eq!(pt.code, "ST36");
        assert_eq!(pt.meridian_name, "足阳明胃经");
        assert!(pt.specific_tags.iter().any(|t| t.contains("肚腹三里留")));
    }

    #[test]
    fn test_recommend_acupoints() {
        let hits = recommend_acupoints_for_symptom("胃痛");
        assert!(!hits.is_empty());
        assert!(hits.iter().any(|p| p.name == "足三里"));
        assert!(hits.iter().any(|p| p.name == "中脘"));
    }
}
