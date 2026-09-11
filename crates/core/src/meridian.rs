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

/// 经典配穴原则
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PairPrinciple {
    /// 原络配穴法 (主客配穴)
    YuanLuo,
    /// 俞募配穴法 (前后配穴)
    ShuMu,
    /// 八脉交会配穴 (上下相通)
    BaMaiJiaoHui,
    /// 表里经配穴
    BiaoLi,
    /// 同名经配穴 (上下同气相求)
    TongMing,
    /// 局部远端对偶配穴 (如四关、四总穴)
    JuBuYuanDuan,
}

/// 经典配穴处方模型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcupointPairFormula {
    /// 配穴名称 (如 "四关穴", "胃痛俞募对偶方")
    pub name: String,
    /// 配穴法则分类
    pub principle: PairPrinciple,
    /// 包含的穴位名称集合 (如 ["合谷", "太冲"])
    pub points: Vec<String>,
    /// 治法与临床功效 (如 "平肝息风，宣通气血")
    pub efficacy: String,
    /// 配伍机理解析
    pub mechanism: String,
    /// 主治病证
    pub indications: Vec<String>,
    /// 经典溯源 (如《针灸大全·四关说》)
    pub origin_classic: String,
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

        // 7. 手少阴心经 (HT)
        MeridianInfo {
            name: "手少阴心经".to_string(),
            code: "HT".to_string(),
            category: MeridianCategory::ShouSanYin,
            element: "君火".to_string(),
            paired_meridian: "手太阳小肠经".to_string(),
            peak_time: "午时 (11:00 - 13:00)".to_string(),
            course_description: "起于心中，出属心系，下膈，络小肠。其支者，从心系，上挟咽，系目系。其直者，复从心系，却上肺，下出腋下，下循臑内后廉，行太阴、心主之后，下肘内，循臂内后廉，抵掌后锐骨之端，入掌内后廉，循小指之内，出其端。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "极泉".to_string(),
                    code: "HT1".to_string(),
                    meridian_name: "手少阴心经".to_string(),
                    location: "在腋区，腋窝中央，腋动脉搏动处。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["心经起始穴".to_string(), "理气活血要穴".to_string()],
                    indications: vec!["心痛".to_string(), "心悸".to_string(), "胸闷".to_string(), "胁肋疼痛".to_string(), "瘰疬".to_string(), "肩臂挛痛".to_string()],
                    manipulation: "避开腋动脉，微向前上方斜刺0.5-1.0寸。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "通里".to_string(),
                    code: "HT5".to_string(),
                    meridian_name: "手少阴心经".to_string(),
                    location: "在前臂前区，腕掌侧远端横纹上1寸，尺侧腕屈肌腱桡侧缘。".to_string(),
                    origin_classic: "《灵枢·经脉》".to_string(),
                    specific_types: vec![SpecificAcupointType::LuoXue],
                    specific_tags: vec!["络穴".to_string(), "宁神开窍要穴".to_string()],
                    indications: vec!["心悸".to_string(), "怔忡".to_string(), "暴喑".to_string(), "舌强不语".to_string(), "腕臂痛".to_string()],
                    manipulation: "直刺0.3-0.5寸。".to_string(),
                    flow_position: 0.65,
                },
                AcupointInfo {
                    name: "神门".to_string(),
                    code: "HT7".to_string(),
                    meridian_name: "手少阴心经".to_string(),
                    location: "在腕前区，腕掌侧远端横纹尺侧端，尺侧腕屈肌腱桡侧凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue],
                    specific_tags: vec!["输穴 (土)".to_string(), "原穴".to_string(), "安神第一要穴".to_string()],
                    indications: vec!["心痛".to_string(), "心悸".to_string(), "怔忡".to_string(), "失眠".to_string(), "健忘".to_string(), "癫狂痫".to_string(), "胸胁痛".to_string()],
                    manipulation: "直刺0.3-0.5寸。".to_string(),
                    flow_position: 0.85,
                },
                AcupointInfo {
                    name: "少冲".to_string(),
                    code: "HT9".to_string(),
                    meridian_name: "手少阴心经".to_string(),
                    location: "在手指，小指末节桡侧，指甲根角侧上方0.1寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (木)".to_string(), "回阳救急".to_string()],
                    indications: vec!["心悸".to_string(), "心痛".to_string(), "胸胁痛".to_string(), "高热昏迷".to_string(), "癫狂".to_string()],
                    manipulation: "浅刺0.1寸或点刺出血。".to_string(),
                    flow_position: 1.00,
                },
            ],
        },

        // 8. 手太阳小肠经 (SI)
        MeridianInfo {
            name: "手太阳小肠经".to_string(),
            code: "SI".to_string(),
            category: MeridianCategory::ShouSanYang,
            element: "火".to_string(),
            paired_meridian: "手少阴心经".to_string(),
            peak_time: "未时 (13:00 - 15:00)".to_string(),
            course_description: "起于小指之端，循手外侧上腕，出踝中，直上循臂骨下廉，出肘内侧两筋之间，上循臑外后廉，出肩解，绕肩胛，交肩上，入缺盆，络心，循咽下膈，抵胃，属小肠。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "少泽".to_string(),
                    code: "SI1".to_string(),
                    meridian_name: "手太阳小肠经".to_string(),
                    location: "在手指，小指末节尺侧，指甲根角侧上方0.1寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (金)".to_string(), "通乳名穴".to_string()],
                    indications: vec!["乳痈".to_string(), "产后乳少".to_string(), "昏迷".to_string(), "热病".to_string(), "头痛".to_string(), "目翳".to_string()],
                    manipulation: "浅刺0.1寸或点刺出血。孕妇慎刺。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "后溪".to_string(),
                    code: "SI3".to_string(),
                    meridian_name: "手太阳小肠经".to_string(),
                    location: "在手背，微握拳，第5掌指关节尺侧近端赤白肉际凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::BaMaiJiaoHuiXue],
                    specific_tags: vec!["输穴 (木)".to_string(), "八脉交会穴 (通督脉)".to_string(), "通督镇痛要穴".to_string()],
                    indications: vec!["头项强痛".to_string(), "腰背痛".to_string(), "目赤肿痛".to_string(), "耳鸣".to_string(), "耳聋".to_string(), "癫狂痫".to_string(), "盗汗".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.20,
                },
                AcupointInfo {
                    name: "养老".to_string(),
                    code: "SI6".to_string(),
                    meridian_name: "手太阳小肠经".to_string(),
                    location: "在前臂后区，尺骨头桡侧骨缝凹陷中。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::XiXue],
                    specific_tags: vec!["郄穴".to_string(), "明目舒筋要穴".to_string()],
                    indications: vec!["目视不明".to_string(), "肩背肘臂痠痛".to_string(), "急性腰痛".to_string()],
                    manipulation: "向肘部斜刺0.5-0.8寸。".to_string(),
                    flow_position: 0.40,
                },
                AcupointInfo {
                    name: "天宗".to_string(),
                    code: "SI11".to_string(),
                    meridian_name: "手太阳小肠经".to_string(),
                    location: "在肩胛区，肩胛冈中点与肩胛骨下角连线上1/3与下2/3交点凹陷中。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_tags: vec!["肩周舒筋要穴".to_string()],
                    specific_types: vec![SpecificAcupointType::Normal],
                    indications: vec!["肩胛疼痛".to_string(), "肩背痹痛".to_string(), "乳痈".to_string(), "气喘".to_string()],
                    manipulation: "直刺或斜刺0.5-1.0寸。".to_string(),
                    flow_position: 0.70,
                },
                AcupointInfo {
                    name: "听宫".to_string(),
                    code: "SI19".to_string(),
                    meridian_name: "手太阳小肠经".to_string(),
                    location: "在面部，耳屏正中与下颌骨髁突之间的凹陷中。微张口取穴。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["手足少阳、手太阳交会穴".to_string(), "耳疾要穴".to_string()],
                    indications: vec!["耳鸣".to_string(), "耳聋".to_string(), "聤耳".to_string(), "齿痛".to_string(), "癫狂痫".to_string()],
                    manipulation: "张口，直刺1.0-1.5寸。留针时不可合口。".to_string(),
                    flow_position: 1.00,
                },
            ],
        },

        // 9. 足太阳膀胱经 (BL)
        MeridianInfo {
            name: "足太阳膀胱经".to_string(),
            code: "BL".to_string(),
            category: MeridianCategory::ZuSanYang,
            element: "水".to_string(),
            paired_meridian: "足少阴肾经".to_string(),
            peak_time: "申时 (15:00 - 17:00)".to_string(),
            course_description: "起于目内眦，上额，交巅。其支者，从巅至耳上角。其直者，从巅入络脑，还出别下项，循肩膊内，挟脊抵腰中，入循膂，络肾，属膀胱。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "睛明".to_string(),
                    code: "BL1".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在面部，目内眦内上方眶内侧壁凹陷中。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["手足太阳、足阳明、阴阳跷五脉交会".to_string(), "眼科主穴".to_string()],
                    indications: vec!["目赤肿痛".to_string(), "目眩".to_string(), "近视".to_string(), "夜盲".to_string(), "迎风流泪".to_string()],
                    manipulation: "患者闭目，轻推眼球向外，紧贴眶缘缓慢直刺0.5-1.0寸，不提插捻转，出针按压止血。".to_string(),
                    flow_position: 0.02,
                },
                AcupointInfo {
                    name: "攒竹".to_string(),
                    code: "BL2".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在面部，眉头凹陷中，额切迹处。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["头目止痛要穴".to_string()],
                    indications: vec!["头痛".to_string(), "眉棱骨痛".to_string(), "目视不明".to_string(), "面瘫".to_string(), "呃逆".to_string()],
                    manipulation: "平刺0.5-0.8寸，或向眉中透刺。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "肺俞".to_string(),
                    code: "BL13".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在脊柱区，第3胸椎棘突下，后正中线旁开1.5寸。".to_string(),
                    origin_classic: "《灵枢·背腧》".to_string(),
                    specific_types: vec![SpecificAcupointType::BeiShuXue],
                    specific_tags: vec!["肺之背俞穴".to_string(), "调补肺气要穴".to_string()],
                    indications: vec!["咳嗽".to_string(), "气喘".to_string(), "吐血".to_string(), "骨蒸潮热".to_string(), "盗汗".to_string(), "胸闷".to_string()],
                    manipulation: "向椎体方向斜刺0.5-0.8寸，不可深刺，防气胸。".to_string(),
                    flow_position: 0.25,
                },
                AcupointInfo {
                    name: "肾俞".to_string(),
                    code: "BL23".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在脊柱区，第2腰椎棘突下，后正中线旁开1.5寸。".to_string(),
                    origin_classic: "《灵枢·背腧》".to_string(),
                    specific_types: vec![SpecificAcupointType::BeiShuXue],
                    specific_tags: vec!["肾之背俞穴".to_string(), "先天之本调补要穴".to_string()],
                    indications: vec!["腰痛".to_string(), "遗精".to_string(), "阳痿".to_string(), "月经不调".to_string(), "耳鸣".to_string(), "水肿".to_string(), "小便不利".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.45,
                },
                AcupointInfo {
                    name: "委中".to_string(),
                    code: "BL40".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在膝后区，腘横纹中点。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::XiaHeXue, SpecificAcupointType::SiZongXue],
                    specific_tags: vec!["合穴 (土)".to_string(), "膀胱下合穴".to_string(), "四总穴 (腰背委中求)".to_string(), "刺血退热要穴".to_string()],
                    indications: vec!["腰背痛".to_string(), "下肢痿痹".to_string(), "腹痛".to_string(), "急性吐泻".to_string(), "中暑".to_string(), "丹毒".to_string()],
                    manipulation: "直刺1.0-1.5寸，或三棱针点刺腘静脉放血。".to_string(),
                    flow_position: 0.70,
                },
                AcupointInfo {
                    name: "昆仑".to_string(),
                    code: "BL60".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在踝区，外踝尖与跟腱之间的凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["经穴 (火)".to_string(), "下行降气要穴".to_string()],
                    indications: vec!["后头痛".to_string(), "项强".to_string(), "腰骶疼痛".to_string(), "足跟痛".to_string(), "难产".to_string(), "癫痫".to_string()],
                    manipulation: "直刺0.5-0.8寸。孕妇禁针。".to_string(),
                    flow_position: 0.92,
                },
                AcupointInfo {
                    name: "至阴".to_string(),
                    code: "BL67".to_string(),
                    meridian_name: "足太阳膀胱经".to_string(),
                    location: "在足趾，小趾末节外侧，趾甲根角侧后方0.1寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (金)".to_string(), "矫正胎位神穴".to_string()],
                    indications: vec!["胎位不正".to_string(), "滞产".to_string(), "头痛".to_string(), "目痛".to_string(), "鼻塞".to_string()],
                    manipulation: "浅刺0.1寸；矫正胎位多用艾炷灸或艾条温和灸15-20分钟。".to_string(),
                    flow_position: 1.00,
                },
            ],
        },

        // 10. 足少阴肾经 (KI)
        MeridianInfo {
            name: "足少阴肾经".to_string(),
            code: "KI".to_string(),
            category: MeridianCategory::ZuSanYin,
            element: "水".to_string(),
            paired_meridian: "足太阳膀胱经".to_string(),
            peak_time: "酉时 (17:00 - 19:00)".to_string(),
            course_description: "起于小趾之下，斜走足心，出于然谷之下，循内踝之后，别入跟中，以上踹内，出腘内廉，上股内后廉，贯脊属肾，络膀胱。其直者，从肾上贯肝膈，入肺中，循喉咙，挟舌本。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "涌泉".to_string(),
                    code: "KI1".to_string(),
                    meridian_name: "足少阴肾经".to_string(),
                    location: "在足底部，卷足时足前部凹陷处，约当足底第2、第3趾蹼缘与足跟连线的前1/3与后2/3交点上。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (木)".to_string(), "开窍苏厥要穴".to_string(), "引火归元".to_string()],
                    indications: vec!["昏迷".to_string(), "中暑".to_string(), "癫痫".to_string(), "头痛".to_string(), "眩晕".to_string(), "咯血".to_string(), "咽喉肿痛".to_string(), "便秘".to_string()],
                    manipulation: "直刺0.5-1.0寸；多用贴敷或艾灸引热下行。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "太溪".to_string(),
                    code: "KI3".to_string(),
                    meridian_name: "足少阴肾经".to_string(),
                    location: "在踝区，内踝尖与跟腱之间的凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue],
                    specific_tags: vec!["输穴 (土)".to_string(), "原穴".to_string(), "滋补肾阴第一要穴".to_string(), "回阳九针穴".to_string()],
                    indications: vec!["头痛".to_string(), "目眩".to_string(), "咽喉肿痛".to_string(), "牙痛".to_string(), "耳鸣".to_string(), "耳聋".to_string(), "气喘".to_string(), "消渴".to_string(), "遗精".to_string(), "腰脊痛".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.25,
                },
                AcupointInfo {
                    name: "照海".to_string(),
                    code: "KI6".to_string(),
                    meridian_name: "足少阴肾经".to_string(),
                    location: "在踝区，内踝尖下1寸，内踝下缘边际凹陷中。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::BaMaiJiaoHuiXue],
                    specific_tags: vec!["八脉交会穴 (通阴跷脉)".to_string(), "利咽滋阴名穴".to_string()],
                    indications: vec!["咽喉干痛".to_string(), "目赤肿痛".to_string(), "失眠".to_string(), "癫痫".to_string(), "便秘".to_string(), "小便频数".to_string(), "月经不调".to_string()],
                    manipulation: "直刺0.5-0.8寸。".to_string(),
                    flow_position: 0.40,
                },
                AcupointInfo {
                    name: "复溜".to_string(),
                    code: "KI7".to_string(),
                    meridian_name: "足少阴肾经".to_string(),
                    location: "在小腿内侧，内踝尖上2寸，跟腱的前缘。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["经穴 (金)".to_string(), "利水调汗要穴".to_string()],
                    indications: vec!["水肿".to_string(), "腹胀".to_string(), "腹泻".to_string(), "盗汗".to_string(), "无汗".to_string(), "下肢痿痹".to_string()],
                    manipulation: "直刺0.8-1.2寸。".to_string(),
                    flow_position: 0.50,
                },
            ],
        },

        // 11. 手厥阴心包经 (PC)
        MeridianInfo {
            name: "手厥阴心包经".to_string(),
            code: "PC".to_string(),
            category: MeridianCategory::ShouSanYin,
            element: "相火".to_string(),
            paired_meridian: "手少阳三焦经".to_string(),
            peak_time: "戌时 (19:00 - 21:00)".to_string(),
            course_description: "起于胸中，出属心包络，下膈，历络三焦。其支者，循胸出胁，下腋三寸，上抵腋下，循臑内，行太阴、少阴之间，入肘中，下臂，行两筋之间，入掌中，循中指，出其端。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "曲泽".to_string(),
                    code: "PC3".to_string(),
                    meridian_name: "手厥阴心包经".to_string(),
                    location: "在肘前区，肘微屈，肘横纹上，肱二头肌腱尺侧缘凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["合穴 (水)".to_string(), "清热止呕穴".to_string()],
                    indications: vec!["心痛".to_string(), "心悸".to_string(), "胃痛".to_string(), "呕吐".to_string(), "泄泻".to_string(), "热病".to_string(), "肘臂挛痛".to_string()],
                    manipulation: "直刺0.8-1.2寸，或点刺出血。".to_string(),
                    flow_position: 0.35,
                },
                AcupointInfo {
                    name: "郄门".to_string(),
                    code: "PC4".to_string(),
                    meridian_name: "手厥阴心包经".to_string(),
                    location: "在前臂前区，腕掌侧远端横纹上5寸，掌长肌腱与桡侧腕屈肌腱之间。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::XiXue],
                    specific_tags: vec!["郄穴".to_string(), "急症心胸止痛穴".to_string()],
                    indications: vec!["心痛".to_string(), "心悸".to_string(), "胸痛".to_string(), "咳血".to_string(), "呕血".to_string(), "疔疮".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.55,
                },
                AcupointInfo {
                    name: "内关".to_string(),
                    code: "PC6".to_string(),
                    meridian_name: "手厥阴心包经".to_string(),
                    location: "在前臂前区，腕掌侧远端横纹上2寸，掌长肌腱与桡侧腕屈肌腱之间。".to_string(),
                    origin_classic: "《灵枢·经脉》".to_string(),
                    specific_types: vec![SpecificAcupointType::LuoXue, SpecificAcupointType::BaMaiJiaoHuiXue, SpecificAcupointType::SiZongXue],
                    specific_tags: vec!["络穴".to_string(), "八脉交会穴 (通阴维脉)".to_string(), "四总穴 (心胸内关谋)".to_string(), "心胃神经综合第一要穴".to_string()],
                    indications: vec!["心痛".to_string(), "心悸".to_string(), "胸闷".to_string(), "胃痛".to_string(), "呕吐".to_string(), "呃逆".to_string(), "失眠".to_string(), "癫痫".to_string(), "晕车".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.75,
                },
                AcupointInfo {
                    name: "大陵".to_string(),
                    code: "PC7".to_string(),
                    meridian_name: "手厥阴心包经".to_string(),
                    location: "在腕前区，腕掌侧远端横纹中点，掌长肌腱与桡侧腕屈肌腱之间。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue],
                    specific_tags: vec!["输穴 (土)".to_string(), "原穴".to_string(), "十三鬼穴".to_string()],
                    indications: vec!["心痛".to_string(), "心悸".to_string(), "胃痛".to_string(), "呕吐".to_string(), "癫狂".to_string(), "胸胁痛".to_string(), "口臭".to_string()],
                    manipulation: "直刺0.3-0.5寸。".to_string(),
                    flow_position: 0.88,
                },
                AcupointInfo {
                    name: "劳宫".to_string(),
                    code: "PC8".to_string(),
                    meridian_name: "手厥阴心包经".to_string(),
                    location: "在掌区，横平第3掌指关节近端，第2、3掌骨之间偏于第3掌骨，握拳时中指尖下。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["荥穴 (火)".to_string(), "泻心火名穴".to_string()],
                    indications: vec!["昏迷".to_string(), "中暑".to_string(), "心痛".to_string(), "癫狂痫".to_string(), "口疮".to_string(), "口臭".to_string()],
                    manipulation: "直刺0.3-0.5寸。".to_string(),
                    flow_position: 0.95,
                },
            ],
        },

        // 12. 手少阳三焦经 (TE)
        MeridianInfo {
            name: "手少阳三焦经".to_string(),
            code: "TE".to_string(),
            category: MeridianCategory::ShouSanYang,
            element: "相火".to_string(),
            paired_meridian: "手厥阴心包经".to_string(),
            peak_time: "亥时 (21:00 - 23:00)".to_string(),
            course_description: "起于小指次指之端，上出两指之间，循手表腕，出臂外两骨之间，上贯肘，循臑外上肩，而交出少阳之后，入缺盆，布膻中，散络心包，下膈，循属三焦。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "中渚".to_string(),
                    code: "TE3".to_string(),
                    meridian_name: "手少阳三焦经".to_string(),
                    location: "在手背，第4、第5掌骨间，掌指关节后方凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["输穴 (木)".to_string(), "通利耳窍要穴".to_string()],
                    indications: vec!["头痛".to_string(), "耳鸣".to_string(), "耳聋".to_string(), "目赤肿痛".to_string(), "咽喉肿痛".to_string(), "热病".to_string(), "手指不能屈伸".to_string()],
                    manipulation: "直刺0.5-0.8寸。".to_string(),
                    flow_position: 0.15,
                },
                AcupointInfo {
                    name: "阳池".to_string(),
                    code: "TE4".to_string(),
                    meridian_name: "手少阳三焦经".to_string(),
                    location: "在腕后区，腕背侧远端横纹上，指伸肌腱的尺侧缘凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::YuanXue],
                    specific_tags: vec!["原穴".to_string(), "调畅少阳气机".to_string()],
                    indications: vec!["头痛".to_string(), "目赤肿痛".to_string(), "耳聋".to_string(), "咽喉肿痛".to_string(), "消渴".to_string(), "手腕疼痛".to_string()],
                    manipulation: "直刺0.3-0.5寸。".to_string(),
                    flow_position: 0.30,
                },
                AcupointInfo {
                    name: "外关".to_string(),
                    code: "TE5".to_string(),
                    meridian_name: "手少阳三焦经".to_string(),
                    location: "在前臂后区，腕背侧远端横纹上2寸，尺骨与桡骨间隙中点。".to_string(),
                    origin_classic: "《灵枢·经脉》".to_string(),
                    specific_types: vec![SpecificAcupointType::LuoXue, SpecificAcupointType::BaMaiJiaoHuiXue],
                    specific_tags: vec!["络穴".to_string(), "八脉交会穴 (通阳维脉)".to_string(), "和解少阳要穴".to_string()],
                    indications: vec!["热病".to_string(), "头痛".to_string(), "耳鸣".to_string(), "耳聋".to_string(), "瘰疬".to_string(), "胁肋痛".to_string(), "上肢痹痛瘫痪".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.45,
                },
                AcupointInfo {
                    name: "支沟".to_string(),
                    code: "TE6".to_string(),
                    meridian_name: "手少阳三焦经".to_string(),
                    location: "在前臂后区，腕背侧远端横纹上3寸，尺骨与桡骨间隙中点。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["经穴 (火)".to_string(), "通便止胁痛特效穴".to_string()],
                    indications: vec!["便秘".to_string(), "胁肋痛".to_string(), "耳鸣".to_string(), "耳聋".to_string(), "暴喑".to_string(), "瘰疬".to_string()],
                    manipulation: "直刺0.5-1.0寸。".to_string(),
                    flow_position: 0.55,
                },
                AcupointInfo {
                    name: "丝竹空".to_string(),
                    code: "TE23".to_string(),
                    meridian_name: "手少阳三焦经".to_string(),
                    location: "在面部，眉梢凹陷中。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["目疾偏头痛穴".to_string()],
                    indications: vec!["头痛".to_string(), "目眩".to_string(), "目赤肿痛".to_string(), "眼睑瞤动".to_string(), "癫痫".to_string()],
                    manipulation: "平刺0.5-1.0寸。禁灸。".to_string(),
                    flow_position: 1.00,
                },
            ],
        },

        // 13. 足少阳胆经 (GB)
        MeridianInfo {
            name: "足少阳胆经".to_string(),
            code: "GB".to_string(),
            category: MeridianCategory::ZuSanYang,
            element: "木".to_string(),
            paired_meridian: "足厥阴肝经".to_string(),
            peak_time: "子时 (23:00 - 01:00)".to_string(),
            course_description: "起于目锐眦，上抵头角，下耳后，循颈行手少阳之前，至肩上却交出手少阳之后，入缺盆。其支者，从耳后入耳中，出走耳前，至目锐眦后。其直者，从缺盆下腋，循胸，过季胁，下合髀厌中。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "风池".to_string(),
                    code: "GB20".to_string(),
                    meridian_name: "足少阳胆经".to_string(),
                    location: "在颈后区，枕骨之下，胸锁乳突肌与斜方肌上端之间的凹陷中。".to_string(),
                    origin_classic: "《灵枢·热病》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["足少阳与阳维脉交会穴".to_string(), "祛风解表头目主穴".to_string()],
                    indications: vec!["头痛".to_string(), "眩晕".to_string(), "目赤肿痛".to_string(), "鼻渊".to_string(), "耳鸣".to_string(), "感冒".to_string(), "颈项强痛".to_string(), "中风失语".to_string()],
                    manipulation: "针尖微向鼻尖方向斜刺0.8-1.2寸，不可向内上方深刺，以免误伤延髓。".to_string(),
                    flow_position: 0.35,
                },
                AcupointInfo {
                    name: "肩井".to_string(),
                    code: "GB21".to_string(),
                    meridian_name: "足少阳胆经".to_string(),
                    location: "在肩胛区，第7颈椎棘突与肩峰最外侧点连线的中点。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["手足少阳、阳维脉交会".to_string(), "通络散结降气要穴".to_string()],
                    indications: vec!["肩背痹痛".to_string(), "颈项强直".to_string(), "乳痈".to_string(), "乳汁不下".to_string(), "难产".to_string(), "瘰疬".to_string()],
                    manipulation: "直刺0.5-0.8寸，深刺极易刺伤肺尖造成气胸。孕妇禁针。".to_string(),
                    flow_position: 0.42,
                },
                AcupointInfo {
                    name: "环跳".to_string(),
                    code: "GB30".to_string(),
                    meridian_name: "足少阳胆经".to_string(),
                    location: "在臀区，股骨大转子最凸点与骶管裂孔连线的外1/3与内2/3交点处。侧卧屈股取穴。".to_string(),
                    origin_classic: "《素问·气府论》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["足少阳与足太阳交会穴".to_string(), "坐骨神经痛第一要穴".to_string(), "回阳九针穴".to_string()],
                    indications: vec!["腰腿痛".to_string(), "下肢痿痹".to_string(), "半身不遂".to_string(), "风疹".to_string()],
                    manipulation: "直刺2.0-3.0寸。".to_string(),
                    flow_position: 0.65,
                },
                AcupointInfo {
                    name: "风市".to_string(),
                    code: "GB31".to_string(),
                    meridian_name: "足少阳胆经".to_string(),
                    location: "在股部，髂胫束后缘，腘横纹上7寸。直立垂手，中指尖所指处。".to_string(),
                    origin_classic: "《肘后备急方》".to_string(),
                    specific_types: vec![SpecificAcupointType::Normal],
                    specific_tags: vec!["祛风止痒要穴".to_string()],
                    indications: vec!["下肢痿痹".to_string(), "麻木".to_string(), "半身不遂".to_string(), "全身瘙痒".to_string(), "瘾疹".to_string()],
                    manipulation: "直刺1.0-2.0寸。".to_string(),
                    flow_position: 0.72,
                },
                AcupointInfo {
                    name: "阳陵泉".to_string(),
                    code: "GB34".to_string(),
                    meridian_name: "足少阳胆经".to_string(),
                    location: "在小腿外侧，腓骨头前下方凹陷中。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::XiaHeXue, SpecificAcupointType::BaHuiXue],
                    specific_tags: vec!["合穴 (土)".to_string(), "胆下合穴".to_string(), "筋会阳陵泉".to_string(), "舒肝利胆名穴".to_string()],
                    indications: vec!["黄疸".to_string(), "胁痛".to_string(), "口苦".to_string(), "呕吐".to_string(), "下肢痿痹".to_string(), "膝膑肿痛".to_string(), "脚气".to_string(), "小儿惊风".to_string()],
                    manipulation: "直刺1.0-1.5寸。".to_string(),
                    flow_position: 0.82,
                },
                AcupointInfo {
                    name: "悬钟".to_string(),
                    code: "GB39".to_string(),
                    meridian_name: "足少阳胆经".to_string(),
                    location: "在小腿外侧，外踝尖上3寸，腓骨前缘。".to_string(),
                    origin_classic: "《针灸甲乙经》".to_string(),
                    specific_types: vec![SpecificAcupointType::BaHuiXue],
                    specific_tags: vec!["髓会绝骨 (悬钟)".to_string(), "补肾益髓要穴".to_string()],
                    indications: vec!["痴呆".to_string(), "中风".to_string(), "半身不遂".to_string(), "颈项强痛".to_string(), "胸胁胀痛".to_string(), "下肢痿痹".to_string()],
                    manipulation: "直刺1.0-1.5寸。".to_string(),
                    flow_position: 0.92,
                },
            ],
        },

        // 14. 足厥阴肝经 (LR)
        MeridianInfo {
            name: "足厥阴肝经".to_string(),
            code: "LR".to_string(),
            category: MeridianCategory::ZuSanYin,
            element: "木".to_string(),
            paired_meridian: "足少阳胆经".to_string(),
            peak_time: "丑时 (01:00 - 03:00)".to_string(),
            course_description: "起于大指丛毛之际，上循足跗上廉，去内踝一寸，上踝八寸，交出太阴之后，上腘内廉，循股阴，入毛中，过阴器，抵小腹，挟胃，属肝，络胆，上贯膈，布胁肋，循喉咙之后，上入颃颡，连目系，上出额，与督脉会于巅。".to_string(),
            acupoints: vec![
                AcupointInfo {
                    name: "大敦".to_string(),
                    code: "LR1".to_string(),
                    meridian_name: "足厥阴肝经".to_string(),
                    location: "在足趾，大趾末节外侧，趾甲根角侧后方0.1寸。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["井穴 (木)".to_string(), "理疝固崩要穴".to_string()],
                    indications: vec!["疝气".to_string(), "少腹痛".to_string(), "崩漏".to_string(), "月经过多".to_string(), "遗尿".to_string(), "癃闭".to_string(), "癫痫".to_string()],
                    manipulation: "斜刺0.1-0.2寸，或点刺出血；治疝多用艾灸。".to_string(),
                    flow_position: 0.05,
                },
                AcupointInfo {
                    name: "行间".to_string(),
                    code: "LR2".to_string(),
                    meridian_name: "足厥阴肝经".to_string(),
                    location: "在足背，第1、第2趾间，趾蹼缘后方赤白肉际处。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue],
                    specific_tags: vec!["荥穴 (火)".to_string(), "清泻肝胆之火要穴".to_string()],
                    indications: vec!["头痛".to_string(), "目赤肿痛".to_string(), "青盲".to_string(), "口㖞".to_string(), "胁痛".to_string(), "青光眼".to_string(), "月经不调".to_string(), "失眠".to_string()],
                    manipulation: "斜刺0.5-0.8寸。".to_string(),
                    flow_position: 0.20,
                },
                AcupointInfo {
                    name: "太冲".to_string(),
                    code: "LR3".to_string(),
                    meridian_name: "足厥阴肝经".to_string(),
                    location: "在足背，第1、第2跖骨间，跖骨底结合部前方凹陷中，或触及动脉搏动处。".to_string(),
                    origin_classic: "《灵枢·本输》".to_string(),
                    specific_types: vec![SpecificAcupointType::WuShuXue, SpecificAcupointType::YuanXue],
                    specific_tags: vec!["输穴 (土)".to_string(), "原穴".to_string(), "四关穴".to_string(), "平肝息风第一穴".to_string()],
                    indications: vec!["头痛".to_string(), "眩晕".to_string(), "失眠".to_string(), "郁证".to_string(), "高血压".to_string(), "目赤肿痛".to_string(), "口㖞".to_string(), "胁痛".to_string(), "小儿惊风".to_string()],
                    manipulation: "直刺0.5-0.8寸。".to_string(),
                    flow_position: 0.35,
                },
                AcupointInfo {
                    name: "期门".to_string(),
                    code: "LR14".to_string(),
                    meridian_name: "足厥阴肝经".to_string(),
                    location: "在胸部，第6肋间隙，前正中线旁开4寸。".to_string(),
                    origin_classic: "《伤寒论》".to_string(),
                    specific_types: vec![SpecificAcupointType::MuXue],
                    specific_tags: vec!["肝之募穴".to_string(), "足厥阴、太阴、阴维交会".to_string(), "疏肝理气主穴".to_string()],
                    indications: vec!["胸胁胀痛".to_string(), "呕吐".to_string(), "呃逆".to_string(), "吞酸".to_string(), "腹胀".to_string(), "乳痈".to_string(), "抑郁".to_string()],
                    manipulation: "斜刺0.5-0.8寸。不可深刺，防伤及肝脾脏器。".to_string(),
                    flow_position: 1.00,
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

/// 获取系统内建的经典配穴处方与机理知识库
pub fn get_canonical_acupoint_pairs() -> Vec<AcupointPairFormula> {
    vec![
        AcupointPairFormula {
            name: "四关开窍方".to_string(),
            principle: PairPrinciple::TongMing,
            points: vec!["合谷".to_string(), "太冲".to_string()],
            efficacy: "平肝息风，开窍镇痛，通调一身之气血。".to_string(),
            mechanism: "合谷为手阳明大肠经原穴，气之主宰；太冲为足厥阴肝经原穴，血之关键。二手二足，气血并调，升降协调，通经止痛。".to_string(),
            indications: vec!["头痛".to_string(), "失眠".to_string(), "眩晕".to_string(), "面瘫".to_string(), "郁证".to_string(), "癫狂".to_string()],
            origin_classic: "《标幽赋》：“寒热痹痛，开四关而即安。”".to_string(),
        },
        AcupointPairFormula {
            name: "公孙内关配穴 (心胸胃症)".to_string(),
            principle: PairPrinciple::BaMaiJiaoHui,
            points: vec!["内关".to_string(), "公孙".to_string()],
            efficacy: "和胃降逆，理气宽胸，宁心安神。".to_string(),
            mechanism: "公孙通冲脉，内关通阴维脉。二穴上下相呼应，主治心、胸、胃部疾患，是八脉交会穴中“公孙冲脉胃心胸，内关阴维下合通”之经典体现。".to_string(),
            indications: vec!["胃痛".to_string(), "呕吐".to_string(), "胸闷".to_string(), "心悸".to_string(), "腹胀".to_string(), "呃逆".to_string()],
            origin_classic: "《针灸大成·八脉图并治症说》".to_string(),
        },
        AcupointPairFormula {
            name: "后溪申脉配穴 (头项目项症)".to_string(),
            principle: PairPrinciple::BaMaiJiaoHui,
            points: vec!["后溪".to_string(), "申脉".to_string()],
            efficacy: "通督舒筋，清头明目，镇静安神。".to_string(),
            mechanism: "后溪通督脉，申脉通阳跷脉。两穴相配，善于疏通督脉与太阳经之经气，主治目内眦、颈项、耳、肩胛及腰脊疼痛。".to_string(),
            indications: vec!["颈项强痛".to_string(), "头痛".to_string(), "腰背痛".to_string(), "癫痫".to_string(), "目赤肿痛".to_string()],
            origin_classic: "《八脉交会八穴歌》：“后溪督脉寻脊膂，申脉阳跷骨外取。”".to_string(),
        },
        AcupointPairFormula {
            name: "胃腑俞募配穴方".to_string(),
            principle: PairPrinciple::ShuMu,
            points: vec!["中脘".to_string(), "胃俞".to_string()],
            efficacy: "调和胃气，降逆止痛，培补中焦。".to_string(),
            mechanism: "中脘为胃之募穴（前），胃俞为胃之背俞穴（后）。阴阳表里相应，“从阴引阳，从阳引阴”，专治胃腑虚实寒热诸疾。".to_string(),
            indications: vec!["胃痛".to_string(), "反胃".to_string(), "呕吐".to_string(), "腹胀".to_string(), "消化不良".to_string()],
            origin_classic: "《素问·阴阳应象大论》：“审其阴阳，以别柔刚，阳病治阴，阴病治阳。”".to_string(),
        },
        AcupointPairFormula {
            name: "肺脏俞募配穴方".to_string(),
            principle: PairPrinciple::ShuMu,
            points: vec!["中府".to_string(), "肺俞".to_string()],
            efficacy: "宣肺理气，止咳平喘，清热化痰。".to_string(),
            mechanism: "中府为肺募（胸前），肺俞为背俞（背后）。前后同调，宣降兼施，宣通肺卫，降逆平喘。".to_string(),
            indications: vec!["咳嗽".to_string(), "气喘".to_string(), "胸满痛".to_string(), "咳血".to_string(), "骨蒸潮热".to_string()],
            origin_classic: "《难经·六十七难》：“俞募皆主治疾。”".to_string(),
        },
        AcupointPairFormula {
            name: "肺大肠原络配穴方".to_string(),
            principle: PairPrinciple::YuanLuo,
            points: vec!["太渊".to_string(), "偏历".to_string()],
            efficacy: "宣肺解表，通腑化浊，利水通便。".to_string(),
            mechanism: "太渊为肺经原穴为主，偏历为大肠经络穴为客。肺与大肠相表里，主客相配，可治肺实热导致的咳喘兼大便秘结。".to_string(),
            indications: vec!["咳嗽".to_string(), "咽喉肿痛".to_string(), "便秘".to_string(), "气喘".to_string(), "手腕痛".to_string()],
            origin_classic: "《灵枢·经脉》原络主客配穴法".to_string(),
        },
        AcupointPairFormula {
            name: "心胆相济配穴方 (神门 + 风池)".to_string(),
            principle: PairPrinciple::JuBuYuanDuan,
            points: vec!["神门".to_string(), "风池".to_string()],
            efficacy: "清心降火，平熄肝风，安神定志。".to_string(),
            mechanism: "神门宁心安神，风池疏风清脑。上取风池以清头目，下取神门以安心神，神得安则志自宁。".to_string(),
            indications: vec!["失眠".to_string(), "头痛".to_string(), "心悸".to_string(), "眩晕".to_string(), "焦虑".to_string()],
            origin_classic: "《席弘赋》".to_string(),
        },
        AcupointPairFormula {
            name: "脾胃同调配穴方 (足三里 + 阴陵泉)".to_string(),
            principle: PairPrinciple::BiaoLi,
            points: vec!["足三里".to_string(), "阴陵泉".to_string()],
            efficacy: "健脾和胃，利湿消肿，培补后天。".to_string(),
            mechanism: "足三里升阳益胃，阴陵泉健脾利湿。一阴一阳，表里相合，健脾除湿，升清降浊。".to_string(),
            indications: vec!["水肿".to_string(), "泄泻".to_string(), "腹胀".to_string(), "食欲不振".to_string(), "身重困倦".to_string()],
            origin_classic: "《灵枢·本输》表里原合配穴法".to_string(),
        },
        AcupointPairFormula {
            name: "少阳枢机对配方 (支沟 + 阳陵泉)".to_string(),
            principle: PairPrinciple::TongMing,
            points: vec!["支沟".to_string(), "阳陵泉".to_string()],
            efficacy: "运转少阳，疏肝利胆，通达三焦。".to_string(),
            mechanism: "手足少阳经同气相求，支沟疏通三焦气化，阳陵泉调畅胆府气机。二穴合用是治疗胁肋痛、便秘的经典经验穴组。".to_string(),
            indications: vec!["胁肋痛".to_string(), "便秘".to_string(), "口苦".to_string(), "呕吐".to_string(), "胆绞痛".to_string()],
            origin_classic: "《玉龙歌》：“胁痛何由引得来，只因少阳气结开；支沟阳陵并下针，管教患者笑颜开。”".to_string(),
        },
    ]
}

/// 依据临床病症推荐配穴处方
pub fn recommend_pairs_for_symptom(symptom: &str) -> Vec<AcupointPairFormula> {
    let clean = symptom.trim();
    if clean.is_empty() {
        return Vec::new();
    }

    let pairs = get_canonical_acupoint_pairs();
    pairs
        .into_iter()
        .filter(|formula| {
            formula.name.contains(clean)
                || formula.efficacy.contains(clean)
                || formula.mechanism.contains(clean)
                || formula
                    .indications
                    .iter()
                    .any(|ind| ind.contains(clean) || clean.contains(ind))
        })
        .collect()
}

/// 依据临床病症症状快速智能推荐穴位
pub fn recommend_acupoints_for_symptom(symptom: &str) -> Vec<AcupointInfo> {
    let clean = symptom.trim();
    if clean.is_empty() {
        return Vec::new();
    }
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
        assert_eq!(base.len(), 14);
        assert!(base.iter().any(|m| m.name == "足阳明胃经"));
        assert!(base.iter().any(|m| m.name == "手厥阴心包经"));
        assert!(base.iter().any(|m| m.name == "足厥阴肝经"));
    }

    #[test]
    fn test_find_acupoint() {
        let zusanli = find_acupoint("足三里");
        assert!(zusanli.is_some());
        let pt = zusanli.unwrap();
        assert_eq!(pt.code, "ST36");
        assert_eq!(pt.meridian_name, "足阳明胃经");
        assert!(pt.specific_tags.iter().any(|t| t.contains("肚腹三里留")));

        let neiguan = find_acupoint("内关");
        assert!(neiguan.is_some());
        assert_eq!(neiguan.unwrap().code, "PC6");
    }

    #[test]
    fn test_recommend_acupoints() {
        let hits = recommend_acupoints_for_symptom("胃痛");
        assert!(!hits.is_empty());
        assert!(hits.iter().any(|p| p.name == "足三里"));
        assert!(hits.iter().any(|p| p.name == "中脘"));
        assert!(hits.iter().any(|p| p.name == "内关"));
    }

    #[test]
    fn test_canonical_acupoint_pairs() {
        let pairs = get_canonical_acupoint_pairs();
        assert!(!pairs.is_empty());
        let siguan = pairs.iter().find(|p| p.name.contains("四关"));
        assert!(siguan.is_some());
        assert_eq!(siguan.unwrap().points, vec!["合谷", "太冲"]);

        let stomach_pairs = recommend_pairs_for_symptom("胃痛");
        assert!(!stomach_pairs.is_empty());
        assert!(stomach_pairs
            .iter()
            .any(|p| p.name.contains("公孙内关") || p.name.contains("胃腑俞募")));
    }
}
