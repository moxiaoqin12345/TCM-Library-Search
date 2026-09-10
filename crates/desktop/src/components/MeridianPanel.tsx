import { Component, For, Show, createSignal, onMount } from "solid-js";
import {
  listMeridians,
  recommendAcupoints,
  AcupointInfo,
  MeridianInfo,
} from "../services/api";
import styles from "./MeridianPanel.module.css";

interface MeridianPanelProps {
  onSearchGlobal?: (keyword: string) => void;
  onSelectAcupoint?: (pointName: string) => void;
}

export const MeridianPanel: Component<MeridianPanelProps> = (props) => {
  const [meridians, setMeridians] = createSignal<MeridianInfo[]>([]);
  const [selectedMeridian, setSelectedMeridian] = createSignal<MeridianInfo | null>(null);
  const [selectedPoint, setSelectedPoint] = createSignal<AcupointInfo | null>(null);
  const [isLoading, setIsLoading] = createSignal<boolean>(true);

  // 症状智能对穴推荐
  const [symptomInput, setSymptomInput] = createSignal<string>("");
  const [recommendedPoints, setRecommendedPoints] = createSignal<AcupointInfo[]>([]);

  onMount(async () => {
    setIsLoading(true);
    try {
      const data = await listMeridians();
      setMeridians(data);
      if (data.length > 0) {
        setSelectedMeridian(data[0]);
        if (data[0].acupoints.length > 0) {
          setSelectedPoint(data[0].acupoints[0]);
        }
      }
    } catch (e) {
      console.error("Failed to load meridians:", e);
    } finally {
      setIsLoading(false);
    }
  });

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
      return;
    }
    try {
      const recs = await recommendAcupoints(sym);
      setRecommendedPoints(recs);
    } catch (e) {
      console.error("Failed to recommend acupoints:", e);
    }
  };

  const selectAcupointByName = (point: AcupointInfo) => {
    // 切换到其归经
    const matchedMeridian = meridians().find((m) => m.name === point.meridian_name);
    if (matchedMeridian) {
      setSelectedMeridian(matchedMeridian);
    }
    setSelectedPoint(point);
  };

  return (
    <div class={styles.container}>
      {/* 1. 左侧经络导航栏 */}
      <div class={styles.meridianListPane}>
        <div class={styles.paneTitle}>
          <span>十四经脉 · 气血流注</span>
        </div>

        {/* 临床辨证配穴速查搜索框 */}
        <div class={styles.symptomSearchBox}>
          <input
            type="text"
            class={styles.symptomInput}
            placeholder="输病症 (如: 胃痛/失眠/头痛)..."
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
        <Show when={recommendedPoints().length > 0}>
          <div class={styles.recommendSection}>
            <div class={styles.recommendHeader}>
              <span>🎯 临床推荐主穴 ({recommendedPoints().length})</span>
              <button
                type="button"
                class={styles.clearBtn}
                onClick={() => setRecommendedPoints([])}
              >
                ✕
              </button>
            </div>
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
          </div>
        </Show>

        <div class={styles.meridianScroll}>
          <Show when={isLoading()}>
            <div class={styles.loadingTip}>正在载入经络图谱数据...</div>
          </Show>
          <For each={meridians()}>
            {(m) => {
              const isSelected = selectedMeridian()?.code === m.code;
              return (
                <div
                  class={`${styles.meridianCard} ${
                    isSelected ? styles.meridianCardActive : ""
                  }`}
                  onClick={() => handleSelectMeridian(m)}
                >
                  <div class={styles.cardMain}>
                    <span class={styles.meridianName}>{m.name}</span>
                    <span class={styles.meridianCode}>{m.code}</span>
                  </div>
                  <div class={styles.cardSub}>
                    <span class={styles.elementTag}>五行属{m.element}</span>
                    <span class={styles.peakTimeTag}>{m.peak_time.split(" ")[0]}</span>
                    <span class={styles.pointCountTag}>{m.acupoints.length} 要穴</span>
                  </div>
                </div>
              );
            }}
          </For>
        </div>
      </div>

      {/* 2. 中间：经脉循行拓扑图解与交互穴位链 */}
      <div class={styles.flowMapPane}>
        <Show when={selectedMeridian()}>
          <div class={styles.mapHeader}>
            <div class={styles.mapTitleGroup}>
              <h3 class={styles.mapTitle}>
                {selectedMeridian()!.name}（{selectedMeridian()!.code}）
              </h3>
              <div class={styles.mapMeta}>
                <span>表里经：{selectedMeridian()!.paired_meridian}</span>
                <span>子午流注：{selectedMeridian()!.peak_time}</span>
              </div>
            </div>
          </div>

          <div class={styles.courseBox}>
            <span class={styles.courseTag}>【经脉循行】</span>
            <p class={styles.courseText}>
              {selectedMeridian()!.course_description}
            </p>
          </div>

          {/* 经络流注拓扑线管与交互穴位节点 */}
          <div class={styles.flowRailSection}>
            <div class={styles.flowRailTitle}>
              <span>经气循行流注脉络（点击穴位节点定位精解）</span>
            </div>

            <div class={styles.flowCanvas}>
              {/* 经络主干线 */}
              <div class={styles.railLine} />

              {/* 循行节点 */}
              <div class={styles.nodesTrack}>
                <For each={selectedMeridian()!.acupoints}>
                  {(point, idx) => {
                    const isSelected = selectedPoint()?.code === point.code;
                    return (
                      <div
                        class={`${styles.flowNodeBox} ${
                          isSelected ? styles.flowNodeActive : ""
                        }`}
                        style={{
                          left: `${point.flow_position * 88 + 6}%`,
                        }}
                        onClick={() => setSelectedPoint(point)}
                        title={`${point.name} (${point.code}) - ${point.specific_tags.join("、")}`}
                      >
                        <div class={styles.nodePulse} />
                        <div class={styles.nodeDot}>{idx() + 1}</div>
                        <div class={styles.nodeLabel}>
                          <span class={styles.nodeName}>{point.name}</span>
                          <span class={styles.nodeCode}>{point.code}</span>
                        </div>
                      </div>
                    );
                  }}
                </For>
              </div>
            </div>
          </div>

          {/* 穴位列表矩阵 */}
          <div class={styles.acupointGridSection}>
            <div class={styles.gridTitle}>重点穴位明细</div>
            <div class={styles.acupointGrid}>
              <For each={selectedMeridian()!.acupoints}>
                {(pt) => {
                  const isSelected = selectedPoint()?.code === pt.code;
                  return (
                    <div
                      class={`${styles.gridCard} ${
                        isSelected ? styles.gridCardActive : ""
                      }`}
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
