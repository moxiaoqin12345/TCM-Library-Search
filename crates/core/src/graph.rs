use serde::{Deserialize, Serialize};

/// 知识图谱节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeType {
    /// 方剂节点 (Formula)
    Formula,
    /// 本草中药节点 (Herb)
    Herb,
    /// 归经经络节点 (Meridian)
    Meridian,
    /// 四气五味药性节点 (NatureFlavor)
    NatureFlavor,
    /// 治法功用节点 (Treatment)
    Treatment,
    /// 主治证候病症节点 (Indication)
    Indication,
}

/// 知识图谱边关系类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphEdgeType {
    /// 方剂包含组方本草 (Contains)
    Contains,
    /// 君臣佐使角色 (Monarch, Minister, Assistant, Envoy)
    Monarch,
    Minister,
    Assistant,
    Envoy,
    /// 本草归属经络 (Channels / Guijing)
    Channels,
    /// 具有性味属性 (HasProperty)
    HasProperty,
    /// 具有治法/主治功效 (Treats)
    Treats,
    /// 配伍相克与禁忌 (Incompatible / 相反相畏预警)
    Incompatible,
}

/// 知识图谱节点实体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphNode {
    /// 节点唯一 ID (如 "formula_guizhitang", "herb_guizhi")
    pub id: String,
    /// 显示标签名称 (如 "桂枝汤", "桂枝")
    pub label: String,
    /// 节点大类
    pub node_type: GraphNodeType,
    /// 节点权重/重要性 (0.1 ~ 1.0)
    pub weight: f64,
    /// 关联附加信息/描述
    pub description: String,
    /// 分组分类标识 (用于可视化色彩区分)
    pub category: String,
}

/// 知识图谱关系边实体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphEdge {
    /// 源节点 ID
    pub source: String,
    /// 目标节点 ID
    pub target: String,
    /// 关系类型
    pub edge_type: GraphEdgeType,
    /// 关系标签 (如 "君药", "辛温解表", "归心经", "十九畏禁忌")
    pub label: String,
    /// 边权重/连接强度
    pub weight: f64,
    /// 是否为禁忌/冲突警告边
    pub is_warning: bool,
}

/// 知识图谱完整模型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// 图谱主题名称 (如 "桂枝汤配伍协同图谱")
    pub title: String,
    /// 核心中心实体 ID
    pub focus_id: String,
    /// 节点列表
    pub nodes: Vec<GraphNode>,
    /// 关系边列表
    pub edges: Vec<GraphEdge>,
    /// 核心治法机理解读
    pub clinical_summary: String,
}

/// 经典配伍知识图谱库构建
pub fn get_canonical_formula_graphs() -> Vec<KnowledgeGraph> {
    vec![
        // 1. 桂枝汤配伍图谱 (群方之首，调和营卫)
        KnowledgeGraph {
            title: "桂枝汤 · 调和营卫经典配伍图谱".to_string(),
            focus_id: "formula_guizhitang".to_string(),
            nodes: vec![
                GraphNode {
                    id: "formula_guizhitang".to_string(),
                    label: "桂枝汤".to_string(),
                    node_type: GraphNodeType::Formula,
                    weight: 1.0,
                    description: "《伤寒论》群方之冠，解肌发表，调和营卫。".to_string(),
                    category: "经方".to_string(),
                },
                GraphNode {
                    id: "herb_guizhi".to_string(),
                    label: "桂枝".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.9,
                    description: "辛温解肌发表，温通经脉，助卫阳。".to_string(),
                    category: "君药".to_string(),
                },
                GraphNode {
                    id: "herb_shaoyao".to_string(),
                    label: "芍药".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.85,
                    description: "酸苦微寒，敛阴和营，益阴敛汗。".to_string(),
                    category: "臣药".to_string(),
                },
                GraphNode {
                    id: "herb_shengjiang".to_string(),
                    label: "生姜".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.7,
                    description: "辛温散寒，助桂枝解肌，止呕降逆。".to_string(),
                    category: "佐药".to_string(),
                },
                GraphNode {
                    id: "herb_dazao".to_string(),
                    label: "大枣".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.7,
                    description: "甘温补脾和胃，益气养血，助芍药和营。".to_string(),
                    category: "佐药".to_string(),
                },
                GraphNode {
                    id: "herb_gancao".to_string(),
                    label: "炙甘草".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.75,
                    description: "甘温益气和中，调和诸药。".to_string(),
                    category: "使药".to_string(),
                },
                GraphNode {
                    id: "meridian_taiyang".to_string(),
                    label: "太阳膀胱经".to_string(),
                    node_type: GraphNodeType::Meridian,
                    weight: 0.65,
                    description: "主一身之表，主皮毛。".to_string(),
                    category: "归经".to_string(),
                },
                GraphNode {
                    id: "meridian_piwei".to_string(),
                    label: "脾胃经".to_string(),
                    node_type: GraphNodeType::Meridian,
                    weight: 0.65,
                    description: "中焦后天之本，气血营卫所生。".to_string(),
                    category: "归经".to_string(),
                },
                GraphNode {
                    id: "nature_xinwen".to_string(),
                    label: "辛甘化阳".to_string(),
                    node_type: GraphNodeType::NatureFlavor,
                    weight: 0.6,
                    description: "桂枝配甘草，辛甘发散为阳，充养卫气。".to_string(),
                    category: "配伍法度".to_string(),
                },
                GraphNode {
                    id: "nature_suangan".to_string(),
                    label: "酸甘化阴".to_string(),
                    node_type: GraphNodeType::NatureFlavor,
                    weight: 0.6,
                    description: "白芍配甘草，酸甘化阴，和营敛阴。".to_string(),
                    category: "配伍法度".to_string(),
                },
                GraphNode {
                    id: "treat_tiaoheyingwei".to_string(),
                    label: "解肌调和营卫".to_string(),
                    node_type: GraphNodeType::Treatment,
                    weight: 0.8,
                    description: "桂枝芍药等量相配，散收兼备，营卫同调。".to_string(),
                    category: "治法".to_string(),
                },
                GraphNode {
                    id: "ind_zhongfeng".to_string(),
                    label: "太阳中风表虚证".to_string(),
                    node_type: GraphNodeType::Indication,
                    weight: 0.75,
                    description: "发热、头痛、汗出、恶风、脉浮缓。".to_string(),
                    category: "主治".to_string(),
                },
            ],
            edges: vec![
                GraphEdge {
                    source: "formula_guizhitang".to_string(),
                    target: "herb_guizhi".to_string(),
                    edge_type: GraphEdgeType::Monarch,
                    label: "君药 (解肌通阳)".to_string(),
                    weight: 1.0,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_guizhitang".to_string(),
                    target: "herb_shaoyao".to_string(),
                    edge_type: GraphEdgeType::Minister,
                    label: "臣药 (和营敛阴)".to_string(),
                    weight: 0.95,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_guizhitang".to_string(),
                    target: "herb_shengjiang".to_string(),
                    edge_type: GraphEdgeType::Assistant,
                    label: "佐药 (助桂发表)".to_string(),
                    weight: 0.8,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_guizhitang".to_string(),
                    target: "herb_dazao".to_string(),
                    edge_type: GraphEdgeType::Assistant,
                    label: "佐药 (补中滋营)".to_string(),
                    weight: 0.8,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_guizhitang".to_string(),
                    target: "herb_gancao".to_string(),
                    edge_type: GraphEdgeType::Envoy,
                    label: "使药 (调和诸药)".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_guizhi".to_string(),
                    target: "herb_shaoyao".to_string(),
                    edge_type: GraphEdgeType::Contains,
                    label: "散收相对·阴阳相因".to_string(),
                    weight: 0.95,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_guizhi".to_string(),
                    target: "nature_xinwen".to_string(),
                    edge_type: GraphEdgeType::HasProperty,
                    label: "与甘草相合".to_string(),
                    weight: 0.7,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_shaoyao".to_string(),
                    target: "nature_suangan".to_string(),
                    edge_type: GraphEdgeType::HasProperty,
                    label: "与甘草相合".to_string(),
                    weight: 0.7,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_guizhi".to_string(),
                    target: "meridian_taiyang".to_string(),
                    edge_type: GraphEdgeType::Channels,
                    label: "走太阳经解肌".to_string(),
                    weight: 0.8,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_shengjiang".to_string(),
                    target: "herb_dazao".to_string(),
                    edge_type: GraphEdgeType::Contains,
                    label: "姜枣配对·调和脾胃营卫".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_guizhitang".to_string(),
                    target: "treat_tiaoheyingwei".to_string(),
                    edge_type: GraphEdgeType::Treats,
                    label: "核心治法".to_string(),
                    weight: 0.9,
                    is_warning: false,
                },
                GraphEdge {
                    source: "treat_tiaoheyingwei".to_string(),
                    target: "ind_zhongfeng".to_string(),
                    edge_type: GraphEdgeType::Treats,
                    label: "主治证候".to_string(),
                    weight: 0.9,
                    is_warning: false,
                },
            ],
            clinical_summary: "桂枝汤方中桂枝解肌散邪，温通卫阳；芍药敛阴和营，益阴敛汗。桂枝与芍药等量合用，一散一收，既能散太阳之表邪，又能和在里之营阴。生姜助桂枝辛散，大枣助芍药滋阴，姜枣合用升发脾胃之气以资汗源；炙甘草益气和中，调和诸药。全方体现辛甘化阳、酸甘化阴之精妙法度。".to_string(),
        },

        // 2. 麻黄汤配伍图谱 (宣肺平喘，发汗解表)
        KnowledgeGraph {
            title: "麻黄汤 · 开腠发汗宣肺平喘图谱".to_string(),
            focus_id: "formula_mahuangtang".to_string(),
            nodes: vec![
                GraphNode {
                    id: "formula_mahuangtang".to_string(),
                    label: "麻黄汤".to_string(),
                    node_type: GraphNodeType::Formula,
                    weight: 1.0,
                    description: "《伤寒论》发汗解表之峻剂，宣肺平喘。".to_string(),
                    category: "经方".to_string(),
                },
                GraphNode {
                    id: "herb_mahuang".to_string(),
                    label: "麻黄".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.95,
                    description: "辛温发汗，开腠理，宣肺气以平喘。".to_string(),
                    category: "君药".to_string(),
                },
                GraphNode {
                    id: "herb_guizhi".to_string(),
                    label: "桂枝".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.85,
                    description: "辛温温通经脉，通阳化气，助麻黄发汗。".to_string(),
                    category: "臣药".to_string(),
                },
                GraphNode {
                    id: "herb_xingren".to_string(),
                    label: "苦杏仁".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.8,
                    description: "苦温降气，宣降肺气，与麻黄一宣一降。".to_string(),
                    category: "佐药".to_string(),
                },
                GraphNode {
                    id: "herb_gancao".to_string(),
                    label: "炙甘草".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.7,
                    description: "甘温调和，缓麻桂之峻烈，防发汗伤正。".to_string(),
                    category: "使药".to_string(),
                },
                GraphNode {
                    id: "meridian_fei".to_string(),
                    label: "手太阴肺经".to_string(),
                    node_type: GraphNodeType::Meridian,
                    weight: 0.7,
                    description: "主气，司呼吸，外合皮毛。".to_string(),
                    category: "归经".to_string(),
                },
                GraphNode {
                    id: "treat_xuanfei".to_string(),
                    label: "发汗解表·宣肺平喘".to_string(),
                    node_type: GraphNodeType::Treatment,
                    weight: 0.85,
                    description: "麻黄与苦杏仁相伍，宣降肺气以定喘逆。".to_string(),
                    category: "治法".to_string(),
                },
                GraphNode {
                    id: "ind_shanghan".to_string(),
                    label: "太阳伤寒表实证".to_string(),
                    node_type: GraphNodeType::Indication,
                    weight: 0.8,
                    description: "恶寒发热、头身痛、无汗而喘、脉浮紧。".to_string(),
                    category: "主治".to_string(),
                },
            ],
            edges: vec![
                GraphEdge {
                    source: "formula_mahuangtang".to_string(),
                    target: "herb_mahuang".to_string(),
                    edge_type: GraphEdgeType::Monarch,
                    label: "君药 (发汗开腠)".to_string(),
                    weight: 1.0,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_mahuangtang".to_string(),
                    target: "herb_guizhi".to_string(),
                    edge_type: GraphEdgeType::Minister,
                    label: "臣药 (通阳解肌)".to_string(),
                    weight: 0.9,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_mahuangtang".to_string(),
                    target: "herb_xingren".to_string(),
                    edge_type: GraphEdgeType::Assistant,
                    label: "佐药 (苦降下气)".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_mahuangtang".to_string(),
                    target: "herb_gancao".to_string(),
                    edge_type: GraphEdgeType::Envoy,
                    label: "使药 (缓急和中)".to_string(),
                    weight: 0.75,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_mahuang".to_string(),
                    target: "herb_xingren".to_string(),
                    edge_type: GraphEdgeType::Contains,
                    label: "宣降相因 (麻黄宣肺+杏仁降气)".to_string(),
                    weight: 0.95,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_mahuang".to_string(),
                    target: "herb_guizhi".to_string(),
                    edge_type: GraphEdgeType::Contains,
                    label: "麻桂相须 (开皮毛+通营卫)".to_string(),
                    weight: 0.95,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_mahuang".to_string(),
                    target: "meridian_fei".to_string(),
                    edge_type: GraphEdgeType::Channels,
                    label: "入肺经宣闭".to_string(),
                    weight: 0.8,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_mahuangtang".to_string(),
                    target: "treat_xuanfei".to_string(),
                    edge_type: GraphEdgeType::Treats,
                    label: "治法功效".to_string(),
                    weight: 0.9,
                    is_warning: false,
                },
                GraphEdge {
                    source: "treat_xuanfei".to_string(),
                    target: "ind_shanghan".to_string(),
                    edge_type: GraphEdgeType::Treats,
                    label: "临床主治".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
            ],
            clinical_summary: "麻黄汤中麻黄为君，专入肺经与膀胱经，开腠理而散风寒，宣肺气以平喘逆；桂枝为臣，通阳达表，助麻黄开发毛窍。二者相合，麻黄得桂枝则发汗之力尤著。杏仁苦降，佐麻黄一开一降，畅肺气而定喘逆；炙甘草使药调和诸药，既助杏仁缓急，又防麻桂发汗太过伤正。".to_string(),
        },

        // 3. 天王补心丹配伍图谱 (滋阴养血，补心安神)
        KnowledgeGraph {
            title: "天王补心丹 · 滋阴养血清热安神图谱".to_string(),
            focus_id: "formula_tianwangbuxin".to_string(),
            nodes: vec![
                GraphNode {
                    id: "formula_tianwangbuxin".to_string(),
                    label: "天王补心丹".to_string(),
                    node_type: GraphNodeType::Formula,
                    weight: 1.0,
                    description: "《摄生秘剖》滋阴清热、养血安神名方。".to_string(),
                    category: "成方".to_string(),
                },
                GraphNode {
                    id: "herb_shengdihuang".to_string(),
                    label: "生地黄".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.95,
                    description: "重用滋阴凉血，补肾阴以制虚火，充血脉。".to_string(),
                    category: "君药".to_string(),
                },
                GraphNode {
                    id: "herb_renshen".to_string(),
                    label: "人参".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.85,
                    description: "大补元气，补脾益心。".to_string(),
                    category: "臣药".to_string(),
                },
                GraphNode {
                    id: "herb_tianmendong".to_string(),
                    label: "天冬".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.8,
                    description: "滋阴清热，润肺滋肾。".to_string(),
                    category: "臣药".to_string(),
                },
                GraphNode {
                    id: "herb_maidong".to_string(),
                    label: "麦冬".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.8,
                    description: "清心润燥，养阴生津。".to_string(),
                    category: "臣药".to_string(),
                },
                GraphNode {
                    id: "herb_baiziren".to_string(),
                    label: "柏子仁".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.8,
                    description: "养心安神，润肠通便。".to_string(),
                    category: "佐药".to_string(),
                },
                GraphNode {
                    id: "herb_suanzaoren".to_string(),
                    label: "酸枣仁".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.8,
                    description: "养心益肝，安神敛汗。".to_string(),
                    category: "佐药".to_string(),
                },
                GraphNode {
                    id: "herb_dangguishen".to_string(),
                    label: "当归身".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.75,
                    description: "补血活血，养心生血。".to_string(),
                    category: "佐药".to_string(),
                },
                GraphNode {
                    id: "herb_zhusha".to_string(),
                    label: "朱砂".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.75,
                    description: "重镇安神，清心镇惊（为衣，防蓄积毒性）。".to_string(),
                    category: "佐使".to_string(),
                },
                GraphNode {
                    id: "herb_jiegang".to_string(),
                    label: "桔梗".to_string(),
                    node_type: GraphNodeType::Herb,
                    weight: 0.7,
                    description: "载药上行，入心经胸中。".to_string(),
                    category: "使药".to_string(),
                },
                GraphNode {
                    id: "meridian_xin".to_string(),
                    label: "手少阴心经".to_string(),
                    node_type: GraphNodeType::Meridian,
                    weight: 0.8,
                    description: "心主血脉，主神明。".to_string(),
                    category: "归经".to_string(),
                },
                GraphNode {
                    id: "meridian_shen".to_string(),
                    label: "足少阴肾经".to_string(),
                    node_type: GraphNodeType::Meridian,
                    weight: 0.75,
                    description: "肾藏精，心肾相交水火既济。".to_string(),
                    category: "归经".to_string(),
                },
                GraphNode {
                    id: "treat_anshen".to_string(),
                    label: "滋阴清热·养血安神".to_string(),
                    node_type: GraphNodeType::Treatment,
                    weight: 0.85,
                    description: "壮水以制虚火，心肾同调，神志得宁。".to_string(),
                    category: "治法".to_string(),
                },
                GraphNode {
                    id: "ind_shimian".to_string(),
                    label: "心肾阴虚血少证".to_string(),
                    node_type: GraphNodeType::Indication,
                    weight: 0.8,
                    description: "虚烦不眠、惊悸怔忡、咽干口燥、舌红少苔。".to_string(),
                    category: "主治".to_string(),
                },
            ],
            edges: vec![
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "herb_shengdihuang".to_string(),
                    edge_type: GraphEdgeType::Monarch,
                    label: "君药 (重用滋阴清热)".to_string(),
                    weight: 1.0,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "herb_tianmendong".to_string(),
                    edge_type: GraphEdgeType::Minister,
                    label: "臣药 (二冬滋阴清虚热)".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "herb_maidong".to_string(),
                    edge_type: GraphEdgeType::Minister,
                    label: "臣药 (润肺养阴)".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "herb_baiziren".to_string(),
                    edge_type: GraphEdgeType::Assistant,
                    label: "佐药 (柏子仁养心安神)".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "herb_suanzaoren".to_string(),
                    edge_type: GraphEdgeType::Assistant,
                    label: "佐药 (酸枣仁敛阴宁神)".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "herb_jiegang".to_string(),
                    edge_type: GraphEdgeType::Envoy,
                    label: "使药 (载药上行)".to_string(),
                    weight: 0.75,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_baiziren".to_string(),
                    target: "herb_suanzaoren".to_string(),
                    edge_type: GraphEdgeType::Contains,
                    label: "二仁相配 (心肝同治·安神定志)".to_string(),
                    weight: 0.9,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_shengdihuang".to_string(),
                    target: "meridian_shen".to_string(),
                    edge_type: GraphEdgeType::Channels,
                    label: "补肾阴助水升".to_string(),
                    weight: 0.8,
                    is_warning: false,
                },
                GraphEdge {
                    source: "herb_shengdihuang".to_string(),
                    target: "meridian_xin".to_string(),
                    edge_type: GraphEdgeType::Channels,
                    label: "清心火济心阴".to_string(),
                    weight: 0.8,
                    is_warning: false,
                },
                GraphEdge {
                    source: "formula_tianwangbuxin".to_string(),
                    target: "treat_anshen".to_string(),
                    edge_type: GraphEdgeType::Treats,
                    label: "治法功效".to_string(),
                    weight: 0.9,
                    is_warning: false,
                },
                GraphEdge {
                    source: "treat_anshen".to_string(),
                    target: "ind_shimian".to_string(),
                    edge_type: GraphEdgeType::Treats,
                    label: "临床主治".to_string(),
                    weight: 0.85,
                    is_warning: false,
                },
            ],
            clinical_summary: "天王补心丹重用生地黄滋阴补肾、凉血清虚火为君药；天冬、麦冬助生地滋阴清火；人参益气生津；柏子仁、酸枣仁养心血、安神志；当归滋阴补血；丹参清心活血；朱砂重镇安神，朱砂为衣，镇浮越之火；桔梗载药上行，使诸药之力留恋于心胸。全方标本兼顾，心肾并调。".to_string(),
        },
    ]
}

/// 根据方剂或中药名称获取匹配的知识图谱
pub fn get_knowledge_graph_for_term(term: &str) -> Option<KnowledgeGraph> {
    let term_clean = term.trim();
    let graphs = get_canonical_formula_graphs();

    // 1. 精确或模糊匹配方剂名称
    for g in &graphs {
        if g.title.contains(term_clean) || g.focus_id.contains(term_clean) {
            return Some(g.clone());
        }
        for n in &g.nodes {
            if n.label == term_clean {
                return Some(g.clone());
            }
        }
    }

    // 2. 若无完全匹配，返回默认首个经典图谱
    graphs.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_graphs() {
        let graphs = get_canonical_formula_graphs();
        assert!(!graphs.is_empty());
        assert!(graphs.iter().any(|g| g.title.contains("桂枝汤")));
        assert!(graphs.iter().any(|g| g.title.contains("麻黄汤")));
        assert!(graphs.iter().any(|g| g.title.contains("天王补心丹")));
    }

    #[test]
    fn test_search_knowledge_graph() {
        let g = get_knowledge_graph_for_term("桂枝");
        assert!(g.is_some());
        let graph = g.unwrap();
        assert!(graph.nodes.iter().any(|n| n.label == "桂枝"));
        assert!(graph.edges.iter().any(|e| e.label.contains("散收相对")));
    }
}
