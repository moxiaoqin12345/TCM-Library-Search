import { Component, createSignal, Show } from "solid-js";
import { appRestart, downloadAndInstallUpdate, type UpdateCheckResult } from "../services/updater";
import styles from "./UpdateModal.module.css";

export interface UpdateModalProps {
  updateInfo: UpdateCheckResult;
  onClose: () => void;
}

export const UpdateModal: Component<UpdateModalProps> = (props) => {
  const [isDownloading, setIsDownloading] = createSignal(false);
  const [downloadPercent, setDownloadPercent] = createSignal(0);
  const [downloadedText, setDownloadedText] = createSignal("");
  const [isReadyToRestart, setIsReadyToRestart] = createSignal(false);
  const [errorMessage, setErrorMessage] = createSignal<string | null>(null);

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  };

  const handleStartUpdate = async () => {
    if (!props.updateInfo.rawUpdate) {
      setErrorMessage("更新对象不可用，请前往 GitHub Releases 页面手动下载。");
      return;
    }

    setIsDownloading(true);
    setErrorMessage(null);

    try {
      await downloadAndInstallUpdate(
        props.updateInfo.rawUpdate,
        (downloaded, total, percent) => {
          setDownloadPercent(percent);
          setDownloadedText(`${formatBytes(downloaded)} / ${formatBytes(total)}`);
        }
      );
      setIsReadyToRestart(true);
    } catch (err: any) {
      setIsDownloading(false);
      setErrorMessage(err?.message || String(err) || "下载安装更新失败，请稍后重试。");
    }
  };

  const handleRestart = async () => {
    await appRestart();
  };

  return (
    <div class={styles.modalOverlay} onClick={() => !isDownloading() && props.onClose()}>
      <div class={styles.modalContent} onClick={(e) => e.stopPropagation()}>
        {/* 标题栏 */}
        <div class={styles.modalHeader}>
          <div class={styles.titleRow}>
            <span class={styles.headerIcon}>🚀</span>
            <h3 class={styles.headerTitle}>发现软件新版本</h3>
            <span class={styles.newVersionBadge}>
              v{props.updateInfo.newVersion || "最新"}
            </span>
          </div>
          <button
            type="button"
            class={styles.closeBtn}
            disabled={isDownloading() && !isReadyToRestart()}
            onClick={props.onClose}
          >
            ✕
          </button>
        </div>

        {/* 主体信息 */}
        <div class={styles.modalBody}>
          <div class={styles.versionCompare}>
            <div class={styles.versionBox}>
              <span class={styles.versionLabel}>当前版本</span>
              <span class={styles.versionVal}>v{props.updateInfo.currentVersion}</span>
            </div>
            <span class={styles.arrowIcon}>➔</span>
            <div class={`${styles.versionBox} ${styles.versionBoxNew}`}>
              <span class={styles.versionLabel}>目标版本</span>
              <span class={styles.versionVal}>v{props.updateInfo.newVersion}</span>
            </div>
          </div>

          {/* 更新日志说明 */}
          <div class={styles.notesSection}>
            <div class={styles.notesTitle}>更新说明与变更日志：</div>
            <div class={styles.notesContent}>
              <pre class={styles.notesPre}>{props.updateInfo.body}</pre>
            </div>
          </div>

          {/* 错误提示 */}
          <Show when={errorMessage()}>
            <div class={styles.errorAlert}>
              <span>⚠️</span>
              <span>{errorMessage()}</span>
            </div>
          </Show>

          {/* 下载进度条 */}
          <Show when={isDownloading() && !isReadyToRestart()}>
            <div class={styles.progressSection}>
              <div class={styles.progressInfo}>
                <span class={styles.progressLabel}>正在下载更新包并校验签名...</span>
                <span class={styles.progressPct}>{downloadPercent()}%</span>
              </div>
              <div class={styles.progressBarWrapper}>
                <div
                  class={styles.progressBarFill}
                  style={{ width: `${downloadPercent()}%` }}
                />
              </div>
              <Show when={downloadedText()}>
                <div class={styles.progressBytes}>{downloadedText()}</div>
              </Show>
            </div>
          </Show>

          {/* 安装就绪提示 */}
          <Show when={isReadyToRestart()}>
            <div class={styles.readyAlert}>
              <span class={styles.readyIcon}>🎉</span>
              <div>
                <strong>更新已下载并安装就绪！</strong>
                <p>点击下方按钮重启应用，即可完成最新版本升级。</p>
              </div>
            </div>
          </Show>
        </div>

        {/* 底部按钮栏 */}
        <div class={styles.modalFooter}>
          <Show
            when={isReadyToRestart()}
            fallback={
              <>
                <button
                  type="button"
                  class={styles.cancelBtn}
                  disabled={isDownloading()}
                  onClick={props.onClose}
                >
                  稍后提醒
                </button>
                <button
                  type="button"
                  class={styles.confirmBtn}
                  disabled={isDownloading()}
                  onClick={handleStartUpdate}
                >
                  {isDownloading() ? "正在下载更新..." : "立即更新"}
                </button>
              </>
            }
          >
            <button
              type="button"
              class={styles.confirmBtn}
              onClick={handleRestart}
            >
              🔄 立即重启应用生效
            </button>
          </Show>
        </div>
      </div>
    </div>
  );
};
