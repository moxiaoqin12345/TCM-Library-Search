import { Component, For, Show, createSignal, onMount } from "solid-js";
import {
  listMeridians,
  recommendAcupoints,
  listAcupointPairs,
  recommendAcupointPairs,
  AcupointInfo,
  MeridianInfo,
  AcupointPairFormula,
  SpecificAcupointType,
} from "../services/api";
import styles from "./MeridianPanel.module.css";

interface MeridianPanelProps {
  onSearchGlobal?: (keyword: string) => void;
  onSelectAcupoint?: (pointName: string) => void;
}

type MainTab = "meridians" | "pairs";
type MeridianCategoryFilter = "all" | "shou_san_yin" | "shou_san_yang" | "zu_san_yang" | "zu_san_yin" | "qi_jing_ba_mai";
type SpecificFilter = "all" | SpecificAcupointType;

export const MeridianPanel: Component<MeridianPanelProps> = (props) => {
  const [meridians, setMeridians] = createSignal<MeridianInfo[]>([]);
  const [allPairs, setAllPairs] = createSignal<AcupointPairFormula[]>([]);
  const [selectedMeridian, setSelectedMeridian] = createSignal<MeridianInfo | null>(null);
  const [selectedPoint, setSelectedPoint] = createSignal<AcupointInfo | null>(null);
  const [selectedPair, setSelectedPair] = createSignal<AcupointPairFormula | null>(null);
  const [isLoading, setIsLoading] = createSignal<boolean>(true);

  // Tab 模式：经络穴位图谱 vs 经典对偶配穴
  const [activeTab, setActiveTab] = createSignal<MainTab>("meridians");

  // 筛选过滤
  const [categoryFilter, setCategoryFilter] = createSignal<MeridianCategoryFilter>("all");
  const [specificFilter, setSpecificFilter] = createSignal<SpecificFilter>("all");

  // 症状智能对穴推荐
  const [symptomInput, setSymptomInput] = createSignal<string>("");
  const [recommendedPoints, setRecommendedPoints] = createSignal<AcupointInfo[]>([]);
  const [recommendedPairs, setRecommendedPairs] = createSignal<AcupointPairFormula[]>([]);

  onMount(async () => {
    setIsLoading(true);
    try {
      const [meridianData, pairData] = await Promise.all([
        listMeridians(),
        listAcupointPairs(),
      ]);
      setMeridians(meridianData);
      setAllPairs(pairData);

      if (meridianData.length > 0) {
        setSelectedMeridian(meridianData[0]);
        if (meridianData[0].acupoints.length > 0) {
          setSelectedPoint(meridianData[0].acupoints[0]);
        }
      }
      if (pairData.length > 0) {
        setSelectedPair(pairData[0]);
      }
    } catch (e) {
      console.error("Failed to load meridian data:", e);
    } finally {
      setIsLoading(false);
    }
  });

  const filteredMeridians = () => {
    const list = meridians();
    const cat = categoryFilter();
    if (cat === "all") return list;
    return list.filter((m) => m.category === cat);
  };

  const filteredAcupoints = () => {
    const m = selectedMeridian();
    if (!m) return [];
    const filter = specificFilter();
    if (filter === "all") return m.acupoints;
    return m.acupoints.filter((pt) => pt.specific_types.includes(filter as SpecificAcupointType));
  };

  const handleSelectMeridian = (m: MeridianInfo) => {
    setSelectedMeridian(m);
    if (m.acupoints.length > 0) {
      setSelectedPoint(m.acupoints[0]);
    } else {
      setSelectedPoint(null);
    }
  };

  const handleSearchSymptom = async () => {
    const sym = symptomInput().trim();
    if (!sym) {
      setRecommendedPoints([]);
      setRecommendedPairs([]);
      return;
    }
    try {
      const [pts, pairs] = await Promise.all([
        recommendAcupoints(sym),
        recommendAcupointPairs(sym),
      ]);
      setRecommendedPoints(pts);
      setRecommendedPairs(pairs);
      if (pairs.length > 0 && activeTab() === "pairs") {
        setSelectedPair(pairs[0]);
      }
    } catch (e) {
      console.error("Failed to recommend acupoints & pairs:", e);
    }
  };

  const selectAcupointByName = (point: AcupointInfo) => {
    setActiveTab("meridians");
    const matchedMeridian = meridians().find((m) => m.name === point.meridian_name);
    if (matchedMeridian) {
      setSelectedMeridian(matchedMeridian);
    }
    setSelectedPoint(point);
  };

  const selectPointByNameString = (name: string) => {
    for (const m of meridians()) {
      for (const pt of m.acupoints) {
        if (pt.name === name) {
          selectAcupointByName(pt);
          return;
        }
      }
    }
    props.onSearchGlobal?.(name);
  };

  const getElementColor = (element: string) => {
    if (element.includes("木")) return "#3d7a5a";
    if (element.includes("火")) return "#c0392b";
    if (element.includes("土")) return "#b57224";
    if (element.includes("金")) return "#997300";
    if (element.includes("水")) return "#2980b9";
    if (element.includes("阳")) return "#d35400";
    return "#5c544d";
  };

  return (
    <div class={styles.container}>
      {/* 1. 左侧导航与列表 */}
      <div class={styles.meridianListPane}>
        {/* 顶部主切换 Tab */}
        <div class={styles.mainTabs}>
          <button
            type="button"
            class={`${styles.mainTabBtn} ${activeTab() === "meridians" ? styles.mainTabBtnActive : ""}`}
            onClick={() => setActiveTab("meridians")}
          >
            十四经脉
          </button>
          <button
            type="button"
            class={`${styles.mainTabBtn} ${activeTab() === "pairs" ? styles.mainTabBtnActive : ""}`}
            onClick={() => setActiveTab("pairs")}
          >
            经典配穴对方
          </button>
        </div>

        {/* 临床辨证配穴速查搜索框 */}
        <div class={styles.symptomSearchBox}>
          <input
            type="text"
            class={styles.symptomInput}
            placeholder={activeTab() === "meridians" ? "输病症 (如: 胃痛/失眠/头痛)..." : "输病症搜对穴配方..."}
            value={symptomInput()}
            onInput={(e) => setSymptomInput(e.currentTarget.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleSearchSymptom();
            }}
          />
          <button
            type="button"
            class={styles.symptomSearchBtn}
            onClick={handleSearchSymptom}
            title="智能推荐配穴"
          >
            配穴
          </button>
        </div>

        {/* 智能对偶配穴推荐展示 */}
        <Show when={recommendedPoints().length > 0 || recommendedPairs().length > 0}>
          <div class={styles.recommendSection}>
            <div class={styles.recommendHeader}>
              <span>
                🎯 推荐结果 ({activeTab() === "meridians" ? recommendedPoints().length : recommendedPairs().length})
              </span>
              <button
                type="button"
                class={styles.clearBtn}
                onClick={() => {
                  setRecommendedPoints([]);
                  setRecommendedPairs([]);
                }}
              >
                ✕
              </button>
            </div>

            <Show when={activeTab() === "meridians" && recommendedPoints().length > 0}>
              <div class={styles.recommendChips}>
                <For each={recommendedPoints()}>
                  {(pt) => (
                    <button
                      type="button"
                      class={styles.recommendChip}
                      onClick={() => selectAcupointByName(pt)}
                      title={`所属：${pt.meridian_name} | 主治：${pt.indications.join("、")}`}
                    >
                      <span class={styles.recName}>{pt.name}</span>
                      <span class={styles.recCode}>({pt.code})</span>
                    </button>
                  )}
                </For>
              </div>
            </Show>

            <Show when={activeTab() === "pairs" && recommendedPairs().length > 0}>
              <div class={styles.recommendChips}>
                <For each={recommendedPairs()}>
                  {(p) => (
                    <button
                      type="button"
                      class={styles.recommendChip}
                      onClick={() => setSelectedPair(p)}
                      title={`功效：${p.efficacy}`}
                    >
                      <span class={styles.recName}>{p.name}</span>
                      <span class={styles.recCode}>[{p.points.join("+")}]</span>
                    </button>
                  )}
                </For>
              </div>
            </Show>
          </div>
        </Show>

        {/* 经脉模式下的类别切换 */}
        <Show when={activeTab() === "meridians"}>
          <div class={styles.categoryFilterBar}>
            <button
              type="button"
              class={`${styles.catFilterPill} ${categoryFilter() === "all" ? styles.catFilterActive : ""}`}
              onClick={() => setCategoryFilter("all")}
            >
              全部
            </button>
            <button
              type="button"
              class={`${styles.catFilterPill} ${categoryFilter() === "shou_san_yin" ? styles.catFilterActive : ""}`}
              onClick={() => setCategoryFilter("shou_san_yin")}
            >
              手三阴
            </button>
            <button
              type="button"
              class={`${styles.catFilterPill} ${categoryFilter() === "shou_san_yang" ? styles.catFilterActive : ""}`}
              onClick={() => setCategoryFilter("shou_san_yang")}
            >
              手三阳
            </button>
            <button
              type="button"
              class={`${styles.catFilterPill} ${categoryFilter() === "zu_san_yang" ? styles.catFilterActive : ""}`}
              onClick={() => setCategoryFilter("zu_san_yang")}
            >
              足三阳
            </button>
            <button
              type="button"
              class={`${styles.catFilterPill} ${categoryFilter() === "zu_san_yin" ? styles.catFilterActive : ""}`}
              onClick={() => setCategoryFilter("zu_san_yin")}
            >
              足三阴
            </button>
            <button
              type="button"
              class={`${styles.catFilterPill} ${categoryFilter() === "qi_jing_ba_mai" ? styles.catFilterActive : ""}`}
              onClick={() => setCategoryFilter("qi_jing_ba_mai")}
            >
              奇经
            </button>
          </div>
        </Show>

        {/* 列表渲染：经络模式 vs 对穴模式 */}
        <div class={styles.meridianScroll}>
          <Show when={isLoading()}>
            <div class={styles.loadingTip}>正在载入经络图谱与对穴数据库...</div>
          </Show>

          {/* 经脉列表 */}
          <Show when={activeTab() === "meridians"}>
            <For each={filteredMeridians()}>
              {(m) => {
                const isSelected = selectedMeridian()?.code === m.code;
                return (
                  <div
                    class={`${styles.meridianCard} ${isSelected ? styles.meridianCardActive : ""}`}
                    onClick={() => handleSelectMeridian(m)}
                  >
                    <div class={styles.cardMain}>
                      <span class={styles.meridianName}>{m.name}</span>
                      <span class={styles.meridianCode}>{m.code}</span>
                    </div>
                    <div class={styles.cardSub}>
                      <span
                        class={styles.elementTag}
                        style={{ "border-left": `3px solid ${getElementColor(m.element)}` }}
                      >
                        {m.element}
                      </span>
                      <span class={styles.peakTimeTag}>{m.peak_time.split(" ")[0]}</span>
                      <span class={styles.pointCountTag}>{m.acupoints.length} 穴</span>
                    </div>
                  </div>
                );
              }}
            </For>
          </Show>

          {/* 经典配穴对方列表 */}
          <Show when={activeTab() === "pairs"}>
            <For each={allPairs()}>
              {(pair) => {
                const isSelected = selectedPair()?.name === pair.name;
                return (
                  <div
                    class={`${styles.pairListCard} ${isSelected ? styles.pairListCardActive : ""}`}
                    onClick={() => setSelectedPair(pair)}
                  >
                    <div class={styles.pairListTop}>
                      <span class={styles.pairListName}>{pair.name}</span>
                      <span class={styles.pairPrincipleBadge}>
                        {pair.principle === "yuan_luo" && "原络配穴"}
                        {pair.principle === "shu_mu" && "俞募配穴"}
                        {pair.principle === "ba_mai_jiao_hui" && "八脉交会"}
                        {pair.principle === "biao_li" && "表里经配穴"}
                        {pair.principle === "tong_ming" && "同名经配穴"}
                        {pair.principle === "ju_bu_yuan_duan" && "局部远端配穴"}
                      </span>
                    </div>
                    <div class={styles.pairPointsTagRow}>
                      <For each={pair.points}>
                        {(pt) => <span class={styles.pairPointTag}>{pt}</span>}
                      </For>
                    </div>
                    <div class={styles.pairEfficacySnippet}>{pair.efficacy}</div>
                  </div>
                );
              }}
            </For>
          </Show>
        </div>
      </div>

      {/* 2. 中间展示区：经脉循行拓扑图解 OR 对穴深研 */}
      <div class={styles.flowMapPane}>
        {/* 经络详情模式 */}
        <Show when={activeTab() === "meridians" && selectedMeridian()}>
          <div class={styles.mapHeader}>
            <div class={styles.mapTitleGroup}>
              <div class={styles.titleRow}>
                <h3 class={styles.mapTitle}>
                  {selectedMeridian()!.name}（{selectedMeridian()!.code}）
                </h3>
                <span
                  class={styles.elementBadge}
                  style={{ background: getElementColor(selectedMeridian()!.element) }}
                >
                  五行属{selectedMeridian()!.element}
                </span>
              </div>
              <div class={styles.mapMeta}>
                <span>表里经络：<strong>{selectedMeridian()!.paired_meridian}</strong></span>
                <span>子午流注时辰：<strong>{selectedMeridian()!.peak_time}</strong></span>
              </div>
            </div>
          </div>

          <div class={styles.courseBox}>
            <span class={styles.courseTag}>【经脉循行走向与病候】</span>
            <p class={styles.courseText}>
              {selectedMeridian()!.course_description}
            </p>
          </div>

          {/* 交互式经脉循行拓扑图 (SVG dynamic vector) */}
          <div class={styles.flowRailSection}>
            <div class={styles.flowRailHeader}>
              <span class={styles.flowRailTitle}>经气流注矢量拓扑图（点击节点查看定位）</span>
              <span class={styles.flowRailSub}>
                流注方向：始于 {selectedMeridian()!.acupoints[0]?.name || ""} ➔ 止于{" "}
                {selectedMeridian()!.acupoints[selectedMeridian()!.acupoints.length - 1]?.name || ""}
              </span>
            </div>

            <div class={styles.svgFlowContainer}>
              <svg class={styles.svgCanvas} viewBox="0 0 760 120" preserveAspectRatio="none">
                <defs>
                  <linearGradient id="meridianGrad" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color={getElementColor(selectedMeridian()!.element)} stop-opacity="0.3" />
                    <stop offset="50%" stop-color={getElementColor(selectedMeridian()!.element)} stop-opacity="0.9" />
                    <stop offset="100%" stop-color={getElementColor(selectedMeridian()!.element)} stop-opacity="0.4" />
                  </linearGradient>
                  <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
                    <feGaussianBlur stdDeviation="3" result="blur" />
                    <feComposite in="SourceGraphic" in2="blur" operator="over" />
                  </filter>
                </defs>

                {/* 背景底轨 */}
                <path
                  d="M 30 60 Q 190 20, 380 60 T 730 60"
                  fill="none"
                  stroke="var(--border-default)"
                  stroke-width="6"
                  stroke-linecap="round"
                />

                {/* 经气脉冲流注主动态光轨 */}
                <path
                  d="M 30 60 Q 190 20, 380 60 T 730 60"
                  fill="none"
                  stroke="url(#meridianGrad)"
                  stroke-width="4"
                  stroke-linecap="round"
                  class={styles.svgPulseFlow}
                />

                {/* 交互穴位节点 */}
                <For each={selectedMeridian()!.acupoints}>
                  {(point) => {
                    const isSelected = selectedPoint()?.code === point.code;
                    // 根据 flow_position 计算贝塞尔曲线上近似坐标
                    const pos = point.flow_position; // 0.05 ~ 0.95
                    const cx = 30 + pos * 700;
                    // y 轴波浪波动计算: y = 60 - 40 * sin(pos * PI)
                    const cy = 60 - Math.sin(pos * Math.PI) * 28;
                    const elemColor = getElementColor(selectedMeridian()!.element);

                    return (
                      <g
                        class={`${styles.svgNodeGroup} ${isSelected ? styles.svgNodeActive : ""}`}
                        onClick={() => setSelectedPoint(point)}
                      >
                        {/* 扩散光晕圆圈 (选定时) */}
                        <Show when={isSelected}>
                          <circle
                            cx={cx}
                            cy={cy}
                            r="16"
                            fill={elemColor}
                            fill-opacity="0.25"
                            class={styles.svgHalo}
                          />
                        </Show>
                        <circle
                          cx={cx}
                          cy={cy}
                          r={isSelected ? "10" : "7"}
                          fill={isSelected ? elemColor : "var(--bg-card)"}
                          stroke={elemColor}
                          stroke-width="2.5"
                          filter={isSelected ? "url(#glow)" : undefined}
                          class={styles.svgCircle}
                        />
                        <text
                          x={cx}
                          y={cy - 14}
                          text-anchor="middle"
                          class={styles.svgNodeText}
                          fill={isSelected ? elemColor : "var(--text-primary)"}
                          font-weight={isSelected ? "bold" : "normal"}
                        >
                          {point.name}
                        </text>
                        <text
                          x={cx}
                          y={cy + 18}
                          text-anchor="middle"
                          class={styles.svgCodeText}
                          fill="var(--text-muted)"
                        >
                          {point.code}
                        </text>
                      </g>
                    );
                  }}
                </For>
              </svg>
            </div>
          </div>

          {/* 特定穴过滤标签组 */}
          <div class={styles.specificFilterSection}>
            <div class={styles.specificFilterTitle}>
              <span>特定穴位过滤矩阵：</span>
            </div>
            <div class={styles.filterPills}>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "all" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("all")}
              >
                全部 ({selectedMeridian()!.acupoints.length})
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "wu_shu_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("wu_shu_xue")}
              >
                五输穴
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "yuan_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("yuan_xue")}
              >
                原穴
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "luo_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("luo_xue")}
              >
                络穴
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "xi_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("xi_xue")}
              >
                郄穴
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "bei_shu_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("bei_shu_xue")}
              >
                背俞穴
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "mu_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("mu_xue")}
              >
                募穴
              </button>
              <button
                type="button"
                class={`${styles.filterPill} ${specificFilter() === "ba_mai_jiao_hui_xue" ? styles.filterPillActive : ""}`}
                onClick={() => setSpecificFilter("ba_mai_jiao_hui_xue")}
              >
                八脉交会穴
              </button>
            </div>
          </div>

          {/* 穴位列表矩阵 */}
          <div class={styles.acupointGridSection}>
            <div class={styles.acupointGrid}>
              <For each={filteredAcupoints()}>
                {(pt) => {
                  const isSelected = selectedPoint()?.code === pt.code;
                  return (
                    <div
                      class={`${styles.gridCard} ${isSelected ? styles.gridCardActive : ""}`}
                      onClick={() => setSelectedPoint(pt)}
                    >
                      <div class={styles.gridCardTop}>
                        <span class={styles.ptName}>{pt.name}</span>
                        <span class={styles.ptCode}>{pt.code}</span>
                      </div>
                      <div class={styles.tagChips}>
                        <For each={pt.specific_tags}>
                          {(tag) => <span class={styles.tagChip}>{tag}</span>}
                        </For>
                      </div>
                    </div>
                  );
                }}
              </For>
            </div>
          </div>
        </Show>

        {/* 经典对穴展示模式 */}
        <Show when={activeTab() === "pairs" && selectedPair()}>
          <div class={styles.pairDetailContainer}>
            <div class={styles.pairDetailHeader}>
              <div class={styles.pairNameRow}>
                <h3 class={styles.pairMainTitle}>{selectedPair()!.name}</h3>
                <span class={styles.pairPrincipleFullTag}>
                  {selectedPair()!.principle === "yuan_luo" && "【原络配穴法】主病与表里同治"}
                  {selectedPair()!.principle === "shu_mu" && "【俞募配穴法】阴阳互济·脏腑通调"}
                  {selectedPair()!.principle === "ba_mai_jiao_hui" && "【八脉交会穴】通奇经八脉与十二正经"}
                  {selectedPair()!.principle === "biao_li" && "【表里经配穴法】阴阳升降相因"}
                  {selectedPair()!.principle === "tong_ming" && "【同名经配穴法】手足同气相求"}
                  {selectedPair()!.principle === "ju_bu_yuan_duan" && "【远近配穴法】通经引气循经求源"}
                </span>
              </div>
              <p class={styles.pairEfficacyHeader}>{selectedPair()!.efficacy}</p>
            </div>

            {/* 对穴组成穴位交互卡片 */}
            <div class={styles.pairPointsCardsRow}>
              <For each={selectedPair()!.points}>
                {(pointName) => (
                  <div
                    class={styles.pairAcupointActionCard}
                    onClick={() => selectPointByNameString(pointName)}
                    title="点击跳转并定位此穴位循行解剖与详细辨析"
                  >
                    <div class={styles.pairPtCardHeader}>
                      <span class={styles.pairPtIcon}>📍</span>
                      <span class={styles.pairPtName}>{pointName}</span>
                    </div>
                    <span class={styles.pairPtActionTip}>查看穴位归经与解剖 ➔</span>
                  </div>
                )}
              </For>
            </div>

            {/* 临床配伍机理深度剖析 */}
            <div class={styles.pairMechanismBox}>
              <div class={styles.mechanismTitle}>
                <span>💡 临床配伍机理辨析</span>
              </div>
              <div class={styles.mechanismContent}>
                {selectedPair()!.mechanism}
              </div>
            </div>

            {/* 历代典籍出处考据 */}
            <div class={styles.pairClassicBox}>
              <div class={styles.classicBoxHeader}>
                <span>📜 经典古籍文献溯源</span>
              </div>
              <div class={styles.classicBoxQuote}>
                {selectedPair()!.origin_classic}
              </div>
            </div>

            {/* 临床主治病证 */}
            <div class={styles.pairIndicationsBox}>
              <div class={styles.indicationsBoxHeader}>
                <span>🩺 主治证候（点击在典籍库中检索）</span>
              </div>
              <div class={styles.pairIndicationsList}>
                <For each={selectedPair()!.indications}>
                  {(ind) => (
                    <button
                      type="button"
                      class={styles.pairIndicationBtn}
                      onClick={() => props.onSearchGlobal?.(ind)}
                    >
                      {ind} 🔍
                    </button>
                  )}
                </For>
              </div>
            </div>
          </div>
        </Show>
      </div>

      {/* 3. 右侧：选定穴位临床定位与名家辨析详解卡 */}
      <div class={styles.acupointDetailPane}>
        <Show
          when={selectedPoint()}
          fallback={
            <div class={styles.emptyDetail}>
              <span>👈 请在左侧或循行脉络中选择穴位</span>
            </div>
          }
        >
          <div class={styles.detailCard}>
            <div class={styles.detailHeader}>
              <div class={styles.detailNameGroup}>
                <h2 class={styles.detailName}>{selectedPoint()!.name}</h2>
                <span class={styles.detailCodeBadge}>{selectedPoint()!.code}</span>
              </div>
              <span class={styles.detailMeridianName}>
                {selectedPoint()!.meridian_name}
              </span>
            </div>

            {/* 特定穴标签群 */}
            <div class={styles.detailTags}>
              <For each={selectedPoint()!.specific_tags}>
                {(t) => <span class={styles.specificBadge}>{t}</span>}
              </For>
            </div>

            <div class={styles.detailBody}>
              {/* 解剖分寸定位 */}
              <div class={styles.infoSection}>
                <div class={styles.sectionHeader}>
                  <span class={styles.sectionIcon}>📍</span>
                  <span class={styles.sectionTitle}>解剖定位与骨度分寸</span>
                </div>
                <div class={styles.sectionContent}>
                  {selectedPoint()!.location}
                </div>
              </div>

              {/* 经典文献出处 */}
              <div class={styles.infoSection}>
                <div class={styles.sectionHeader}>
                  <span class={styles.sectionIcon}>📜</span>
                  <span class={styles.sectionTitle}>经典出处溯源</span>
                </div>
                <div class={styles.classicQuote}>
                  {selectedPoint()!.origin_classic}
                </div>
              </div>

              {/* 临床主治 */}
              <div class={styles.infoSection}>
                <div class={styles.sectionHeader}>
                  <span class={styles.sectionIcon}>🩺</span>
                  <span class={styles.sectionTitle}>临床主治病证</span>
                </div>
                <div class={styles.indicationsList}>
                  <For each={selectedPoint()!.indications}>
                    {(ind) => (
                      <span
                        class={styles.indicationChip}
                        onClick={() => props.onSearchGlobal?.(ind)}
                        title="点击在典籍文库中检索相关病机论述"
                      >
                        {ind}
                      </span>
                    )}
                  </For>
                </div>
              </div>

              {/* 针灸刺灸法操作与禁忌 */}
              <div class={styles.infoSection}>
                <div class={styles.sectionHeader}>
                  <span class={styles.sectionIcon}>💉</span>
                  <span class={styles.sectionTitle}>针灸刺法与配穴按语</span>
                </div>
                <div class={styles.manipulationText}>
                  {selectedPoint()!.manipulation}
                </div>
              </div>

              {/* 全局检索联动按钮 */}
              <button
                type="button"
                class={styles.linkSearchBtn}
                onClick={() => props.onSearchGlobal?.(selectedPoint()!.name)}
              >
                🔍 在典籍库中研读《{selectedPoint()!.name}》相关医案与经典
              </button>
            </div>
          </div>
        </Show>
      </div>
    </div>
  );
};
