import { Component, For, Show, createSignal, onMount, createMemo } from "solid-js";
import {
  KnowledgeGraph,
  GraphNode,
  GraphEdge,
  listFeaturedKnowledgeGraphs,
  getKnowledgeGraph,
} from "../services/api";
import styles from "./KnowledgeGraphPanel.module.css";

interface KnowledgeGraphPanelProps {
  onSearchGlobal?: (keyword: string) => void;
}

interface LayoutNode extends GraphNode {
  x: number;
  y: number;
  color: string;
}

export const KnowledgeGraphPanel: Component<KnowledgeGraphPanelProps> = (props) => {
  const [featuredGraphs, setFeaturedGraphs] = createSignal<KnowledgeGraph[]>([]);
  const [selectedGraph, setSelectedGraph] = createSignal<KnowledgeGraph | null>(null);
  const [selectedNode, setSelectedNode] = createSignal<GraphNode | null>(null);
  const [searchKeyword, setSearchKeyword] = createSignal<string>("");
  const [errorMessage, setErrorMessage] = createSignal<string>("");

  onMount(async () => {
    try {
      const graphs = await listFeaturedKnowledgeGraphs();
      setFeaturedGraphs(graphs);
      if (graphs.length > 0) {
        setSelectedGraph(graphs[0]);
        const centerNode = graphs[0].nodes.find((n) => n.node_type === "formula") || graphs[0].nodes[0];
        setSelectedNode(centerNode || null);
      }
    } catch (e) {
      console.error("Failed to load featured knowledge graphs:", e);
    }
  });

  const handleSearchGraph = async () => {
    const kw = searchKeyword().trim();
    if (!kw) return;
    setErrorMessage("");
    try {
      const graph = await getKnowledgeGraph(kw);
      if (graph) {
        setSelectedGraph(graph);
        const centerNode = graph.nodes.find((n) => n.node_type === "formula") || graph.nodes[0];
        setSelectedNode(centerNode || null);
      } else {
        setErrorMessage("未检索到相关配伍图谱，可尝试输入：桂枝汤、麻黄汤等");
      }
    } catch (e: any) {
      console.error("Failed to search knowledge graph:", e);
      setErrorMessage(typeof e === "string" ? e : "检索图谱失败");
    }
  };

  const handleSelectGraph = (graph: KnowledgeGraph) => {
    setSelectedGraph(graph);
    const centerNode = graph.nodes.find((n) => n.node_type === "formula") || graph.nodes[0];
    setSelectedNode(centerNode || null);
    setErrorMessage("");
  };

  // 节点颜色映射函数
  const getNodeColor = (node: GraphNode): string => {
    switch (node.node_type) {
      case "formula":
        return "#b83b26"; // 朱砂红
      case "herb": {
        const desc = node.description || "";
        if (desc.includes("君药")) return "#c0392b";
        if (desc.includes("臣药")) return "#d35400";
        if (desc.includes("佐药")) return "#2980b9";
        if (desc.includes("使药")) return "#8e44ad";
        return "#3d7a5a"; // 草木绿
      }
      case "meridian":
        return "#1f4e5b"; // 墨青
      case "nature_flavor":
        return "#b57224"; // 姜黄土色
      case "treatment":
        return "#16a085"; // 翡翠绿
      case "indication":
        return "#7d3c98"; // 紫藤
      default:
        return "#5c544d";
    }
  };

  // 连线颜色映射
  const getEdgeColor = (edge: GraphEdge): string => {
    if (edge.is_warning || edge.edge_type === "incompatible") {
      return "#e74c3c";
    }
    switch (edge.edge_type) {
      case "monarch":
        return "#c0392b";
      case "minister":
        return "#d35400";
      case "assistant":
        return "#2980b9";
      case "envoy":
        return "#8e44ad";
      case "channels":
        return "#1f4e5b";
      case "has_property":
        return "#b57224";
      case "treats":
        return "#16a085";
      default:
        return "#888888";
    }
  };

  // 计算节点二维拓扑布局 (中心放射 + 双层圆环)
  const layoutData = createMemo(() => {
    const graph = selectedGraph();
    if (!graph) return { nodes: [] as LayoutNode[], edges: [] as (GraphEdge & { x1: number; y1: number; x2: number; y2: number; color: string })[] };

    const width = 800;
    const height = 540;
    const centerX = width / 2;
    const centerY = height / 2;

    const formulaNodes = graph.nodes.filter((n) => n.node_type === "formula");
    const herbNodes = graph.nodes.filter((n) => n.node_type === "herb");
    const otherNodes = graph.nodes.filter(
      (n) => n.node_type !== "formula" && n.node_type !== "herb"
    );

    const layoutNodesMap = new Map<string, LayoutNode>();

    // 1. 中心节点 (formula)
    formulaNodes.forEach((node, idx) => {
      const offsetX = (idx - (formulaNodes.length - 1) / 2) * 80;
      layoutNodesMap.set(node.id, {
        ...node,
        x: centerX + offsetX,
        y: centerY,
        color: getNodeColor(node),
      });
    });

    // 2. 内圈本草药味节点 (herb) 半径 ~150px
    const herbRadius = 145;
    const herbCount = herbNodes.length;
    herbNodes.forEach((node, idx) => {
      const angle = (2 * Math.PI * idx) / (herbCount || 1) - Math.PI / 2;
      const x = centerX + herbRadius * Math.cos(angle);
      const y = centerY + herbRadius * Math.sin(angle);
      layoutNodesMap.set(node.id, {
        ...node,
        x,
        y,
        color: getNodeColor(node),
      });
    });

    // 3. 外圈属性节点 (meridian, nature_flavor, treatment, indication) 半径 ~240px
    const otherRadius = 235;
    const otherCount = otherNodes.length;
    otherNodes.forEach((node, idx) => {
      const angle = (2 * Math.PI * idx) / (otherCount || 1) - Math.PI / 4;
      const x = centerX + otherRadius * Math.cos(angle);
      const y = centerY + otherRadius * Math.sin(angle);
      layoutNodesMap.set(node.id, {
        ...node,
        x,
        y,
        color: getNodeColor(node),
      });
    });

    // 连线坐标映射
    const layoutEdges = graph.edges
      .map((edge) => {
        const src = layoutNodesMap.get(edge.source);
        const tgt = layoutNodesMap.get(edge.target);
        if (!src || !tgt) return null;
        return {
          ...edge,
          x1: src.x,
          y1: src.y,
          x2: tgt.x,
          y2: tgt.y,
          color: getEdgeColor(edge),
        };
      })
      .filter(Boolean) as (GraphEdge & { x1: number; y1: number; x2: number; y2: number; color: string })[];

    return {
      nodes: Array.from(layoutNodesMap.values()),
      edges: layoutEdges,
    };
  });

  // 与选定节点相关的连接
  const nodeConnections = createMemo(() => {
    const node = selectedNode();
    const graph = selectedGraph();
    if (!node || !graph) return [];
    return graph.edges.filter((e) => e.source === node.id || e.target === node.id);
  });

  return (
    <div class={styles.container}>
      {/* 1. 左侧经典方剂图谱列表与搜索 */}
      <div class={styles.leftSidebar}>
        <div class={styles.paneHeader}>
          <div class={styles.paneTitle}>方剂配伍 · 知识图谱</div>
        </div>

        <div class={styles.searchBox}>
          <input
            type="text"
            class={styles.searchInput}
            placeholder="搜方剂/中药 (如: 桂枝汤)..."
            value={searchKeyword()}
            onInput={(e) => setSearchKeyword(e.currentTarget.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleSearchGraph();
            }}
          />
          <button type="button" class={styles.searchBtn} onClick={handleSearchGraph}>
            构建
          </button>
        </div>

        <Show when={errorMessage()}>
          <div style={{ padding: "0.5rem 0.8rem", color: "var(--accent-cinnabar)", "font-size": "0.75rem" }}>
            {errorMessage()}
          </div>
        </Show>

        <div class={styles.graphScrollList}>
          <For each={featuredGraphs()}>
            {(graph) => {
              const isActive = selectedGraph()?.focus_id === graph.focus_id;
              return (
                <div
                  class={`${styles.graphItemCard} ${isActive ? styles.graphItemActive : ""}`}
                  onClick={() => handleSelectGraph(graph)}
                >
                  <div class={styles.itemTitleRow}>
                    <span class={styles.itemTitle}>{graph.title}</span>
                    <span class={styles.badgeCategory}>经方图谱</span>
                  </div>
                  <div class={styles.itemSummarySnippet}>{graph.clinical_summary}</div>
                </div>
              );
            }}
          </For>
        </div>
      </div>

      {/* 2. 中间知识图谱画布 */}
      <div class={styles.centerCanvasPane}>
        <div class={styles.canvasTopBar}>
          <div class={styles.canvasTitleGroup}>
            <h3 class={styles.canvasMainTitle}>
              {selectedGraph() ? `${selectedGraph()!.title} · 配伍知识图谱` : "配伍图谱"}
            </h3>
          </div>

          <div class={styles.legendRow}>
            <span>
              <span class={styles.legendDot} style={{ background: "#b83b26" }} />
              方剂
            </span>
            <span>
              <span class={styles.legendDot} style={{ background: "#c0392b" }} />
              君药
            </span>
            <span>
              <span class={styles.legendDot} style={{ background: "#d35400" }} />
              臣药
            </span>
            <span>
              <span class={styles.legendDot} style={{ background: "#2980b9" }} />
              佐药
            </span>
            <span>
              <span class={styles.legendDot} style={{ background: "#8e44ad" }} />
              使药
            </span>
            <span>
              <span class={styles.legendDot} style={{ background: "#1f4e5b" }} />
              归经
            </span>
            <span>
              <span class={styles.legendDot} style={{ background: "#16a085" }} />
              治法/功用
            </span>
          </div>
        </div>

        <svg class={styles.svgViewport} viewBox="0 0 800 540">
          <defs>
            <filter id="nodeGlow" x="-20%" y="-20%" width="140%" height="140%">
              <feGaussianBlur stdDeviation="3" result="blur" />
              <feComposite in="SourceGraphic" in2="blur" operator="over" />
            </filter>
            <marker
              id="arrow"
              viewBox="0 0 10 10"
              refX="16"
              refY="5"
              markerWidth="6"
              markerHeight="6"
              orient="auto-start-reverse"
            >
              <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--border-default)" />
            </marker>
          </defs>

          {/* 渲染边 (连线) */}
          <For each={layoutData().edges}>
            {(edge) => {
              const isWarning = edge.is_warning || edge.edge_type === "incompatible";
              const midX = (edge.x1 + edge.x2) / 2;
              const midY = (edge.y1 + edge.y2) / 2;
              return (
                <g>
                  <line
                    x1={edge.x1}
                    y1={edge.y1}
                    x2={edge.x2}
                    y2={edge.y2}
                    stroke={edge.color}
                    stroke-width={isWarning ? "2.5" : "1.8"}
                    stroke-opacity={isWarning ? "0.9" : "0.55"}
                    class={`${styles.edgeLine} ${isWarning ? styles.warningLine : ""}`}
                  />
                  {/* 关系文字标注 */}
                  <text
                    x={midX}
                    y={midY - 4}
                    text-anchor="middle"
                    class={styles.edgeLabelText}
                    fill={edge.color}
                  >
                    {edge.label}
                  </text>
                </g>
              );
            }}
          </For>

          {/* 渲染节点 (圆圈及标签) */}
          <For each={layoutData().nodes}>
            {(node) => {
              const isSelected = selectedNode()?.id === node.id;
              const isFormula = node.node_type === "formula";
              const radius = isFormula ? 24 : node.node_type === "herb" ? 18 : 14;

              return (
                <g
                  class={styles.graphNodeGroup}
                  transform={`translate(${node.x}, ${node.y})`}
                  onClick={() => setSelectedNode(node)}
                >
                  {/* 选定扩散光环 */}
                  <Show when={isSelected}>
                    <circle
                      r={radius + 8}
                      fill={node.color}
                      fill-opacity="0.25"
                      filter="url(#nodeGlow)"
                    />
                  </Show>
                  <circle
                    r={radius}
                    fill={node.color}
                    stroke="var(--bg-card)"
                    stroke-width="2"
                    filter={isSelected ? "url(#nodeGlow)" : undefined}
                  />
                  <text
                    y={radius + 14}
                    text-anchor="middle"
                    class={styles.nodeLabelText}
                    font-size={isFormula ? "0.85rem" : "0.75rem"}
                  >
                    {node.label}
                  </text>
                  <Show when={node.node_type === "herb" && node.category}>
                    <text y={radius + 25} text-anchor="middle" class={styles.nodeSubText}>
                      [{node.category}]
                    </text>
                  </Show>
                </g>
              );
            }}
          </For>
        </svg>
      </div>

      {/* 3. 右侧节点详情与临床按语研读 */}
      <div class={styles.rightDetailPane}>
        <Show
          when={selectedNode()}
          fallback={
            <div style={{ color: "var(--text-muted)", "text-align": "center", "margin-top": "2rem" }}>
              请在图谱中选择节点查看配伍机理
            </div>
          }
        >
          <div class={styles.cardSection}>
            <div class={styles.sectionTitle}>
              <span>📌 节点详情</span>
            </div>
            <div style={{ display: "flex", "align-items": "center", gap: "0.6rem" }}>
              <h2 style={{ margin: 0, "font-size": "1.3rem", color: "var(--text-primary)" }}>
                {selectedNode()!.label}
              </h2>
              <span
                class={styles.badgeCategory}
                style={{
                  background: getNodeColor(selectedNode()!),
                  color: "#fff",
                  padding: "0.2rem 0.5rem",
                }}
              >
                {selectedNode()!.category || selectedNode()!.node_type}
              </span>
            </div>
          </div>

          {/* 节点描述信息 */}
          <Show when={selectedNode()!.description}>
            <div class={styles.cardSection}>
              <div class={styles.sectionTitle}>
                <span>🏷️ 药性与配伍释义</span>
              </div>
              <div class={styles.sectionBody}>
                {selectedNode()!.description}
              </div>
            </div>
          </Show>

          {/* 配伍关系列表 */}
          <div class={styles.cardSection}>
            <div class={styles.sectionTitle}>
              <span>🔗 关联网路与配伍机理 ({nodeConnections().length})</span>
            </div>
            <div style={{ display: "flex", "flex-direction": "column", gap: "0.4rem" }}>
              <For each={nodeConnections()}>
                {(edge) => {
                  const isOut = edge.source === selectedNode()!.id;
                  const otherId = isOut ? edge.target : edge.source;
                  const otherNode = selectedGraph()?.nodes.find((n) => n.id === otherId);
                  const isWarn = edge.is_warning || edge.edge_type === "incompatible";
                  return (
                    <div
                      style={{
                        padding: "0.4rem 0.6rem",
                        background: "var(--bg-subtle)",
                        "border-radius": "var(--radius-sm)",
                        "font-size": "0.78rem",
                        display: "flex",
                        "flex-direction": "column",
                        gap: "0.2rem",
                        border: "1px solid var(--border-subtle)",
                      }}
                    >
                      <div style={{ display: "flex", "justify-content": "space-between", "align-items": "center" }}>
                        <span style={{ "font-weight": "600", color: isWarn ? "var(--accent-cinnabar)" : "var(--text-primary)" }}>
                          {edge.label} ➔ {otherNode?.label || otherId}
                        </span>
                        <span style={{ "font-size": "0.68rem", color: "var(--text-muted)" }}>
                          {edge.edge_type}
                        </span>
                      </div>
                    </div>
                  );
                }}
              </For>
            </div>
          </div>

          {/* 临床配伍按语 */}
          <Show when={selectedGraph()?.clinical_summary}>
            <div class={styles.cardSection}>
              <div class={styles.sectionTitle}>
                <span>📜 临床方解按语</span>
              </div>
              <div class={styles.clinicalSummaryBox}>
                {selectedGraph()!.clinical_summary}
              </div>
            </div>
          </Show>

          {/* 全局研读跳转按钮 */}
          <button
            type="button"
            class={styles.nodeActionBtn}
            onClick={() => props.onSearchGlobal?.(selectedNode()!.label)}
          >
            🔍 在典籍库中研读《{selectedNode()!.label}》名家阐微与医案
          </button>
        </Show>
      </div>
    </div>
  );
};
