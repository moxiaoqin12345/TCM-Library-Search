import { Component, For, Show, createSignal } from "solid-js";
import { CompatibilityAlert } from "../services/api";
import styles from "./CompatibilityRadar.module.css";

interface CompatibilityRadarProps {
  alerts: CompatibilityAlert[];
  onHerbClick?: (herb: string) => void;
}

export const CompatibilityRadar: Component<CompatibilityRadarProps> = (props) => {
  const [isExpanded, setIsExpanded] = createSignal<boolean>(true);

  const severeCount = () =>
    props.alerts.filter((a) => a.severity === "severe").length;
  const warningCount = () =>
    props.alerts.filter((a) => a.severity === "warning").length;

  const getTypeBadge = (type: string) => {
    switch (type) {
      case "shiba_fan":
        return "十八反";
      case "shijiu_wei":
        return "十九畏";
      case "renshen_jinji":
        return "妊娠用药禁忌";
      case "xiang_e":
        return "相恶相杀";
      default:
        return "配伍提示";
    }
  };

  return (
    <Show when={props.alerts && props.alerts.length > 0}>
      <div
        class={`${styles.radarContainer} ${
          severeCount() > 0 ? styles.radarSevere : styles.radarWarning
        }`}
      >
        {/* 顶部警示条 */}
        <div class={styles.radarHeader} onClick={() => setIsExpanded(!isExpanded())}>
          <div class={styles.radarTitleGroup}>
            <span class={styles.radarPulseIcon}>⚡</span>
            <span class={styles.radarTitle}>
              中药配伍禁忌预警雷达
            </span>
            <span class={styles.alertCounter}>
              检出 {props.alerts.length} 项冲突
              <Show when={severeCount() > 0}>
                <span class={styles.severeTag}>（{severeCount()} 项相反剧毒）</span>
              </Show>
              <Show when={warningCount() > 0}>
                <span class={styles.warningTag}>（{warningCount()} 项相畏减效）</span>
              </Show>
            </span>
          </div>

          <div class={styles.headerRight}>
            <span class={styles.expandToggleText}>
              {isExpanded() ? "收起明细 ▲" : "查看辨析 ▼"}
            </span>
          </div>
        </div>

        {/* 展开的预警卡片网格 */}
        <Show when={isExpanded()}>
          <div class={styles.radarBody}>
            <div class={styles.alertGrid}>
              <For each={props.alerts}>
                {(alert) => {
                  const isSevere = alert.severity === "severe";
                  return (
                    <div
                      class={`${styles.alertCard} ${
                        isSevere ? styles.cardSevere : styles.cardWarning
                      }`}
                    >
                      <div class={styles.cardTop}>
                        <span class={styles.typeBadge}>
                          {getTypeBadge(alert.incompatibility_type)}
                        </span>
                        <span class={styles.severityBadge}>
                          {isSevere ? "严禁合用" : "临床慎用"}
                        </span>
                      </div>

                      <div class={styles.herbPair}>
                        <span
                          class={styles.herbChip}
                          onClick={(e) => {
                            e.stopPropagation();
                            props.onHerbClick?.(alert.herb_a);
                          }}
                          title="点击聚焦检索该药材"
                        >
                          {alert.herb_a}
                        </span>
                        <Show
                          when={alert.herb_b}
                          fallback={<span class={styles.pairDivider}>〔单药禁忌〕</span>}
                        >
                          <span class={styles.pairDivider}>⚡ 冲突 ⚡</span>
                          <span
                            class={styles.herbChip}
                            onClick={(e) => {
                              e.stopPropagation();
                              if (alert.herb_b) props.onHerbClick?.(alert.herb_b);
                            }}
                            title="点击聚焦检索该药材"
                          >
                            {alert.herb_b}
                          </span>
                        </Show>
                      </div>

                      <div class={styles.rhymeQuote}>
                        『 {alert.source_rhyme} 』
                      </div>

                      <div class={styles.explanationText}>
                        {alert.explanation}
                      </div>
                    </div>
                  );
                }}
              </For>
            </div>
          </div>
        </Show>
      </div>
    </Show>
  );
};
