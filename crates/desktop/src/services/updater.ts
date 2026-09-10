import { invoke } from "@tauri-apps/api/core";
import { check, type Update } from "@tauri-apps/plugin-updater";

export interface UpdateCheckResult {
  available: boolean;
  currentVersion: string;
  newVersion?: string;
  date?: string;
  body?: string;
  rawUpdate?: Update | null;
  error?: string;
}

/**
 * 获取当前应用构建版本号
 */
export async function getAppVersion(): Promise<string> {
  try {
    return await invoke<string>("get_app_version");
  } catch {
    return "0.2.0";
  }
}

/**
 * 重启应用
 */
export async function appRestart(): Promise<void> {
  try {
    await invoke("app_restart");
  } catch (err) {
    console.error("Failed to restart app via IPC:", err);
  }
}

/**
 * 检查 GitHub Releases 是否有新版本
 */
export async function checkForAppUpdate(): Promise<UpdateCheckResult> {
  const currentVersion = await getAppVersion();

  try {
    const update = await check();
    if (update) {
      return {
        available: true,
        currentVersion,
        newVersion: update.version,
        date: update.date,
        body: update.body || "本次更新包含性能提升与功能修复。",
        rawUpdate: update,
      };
    } else {
      return {
        available: false,
        currentVersion,
      };
    }
  } catch (err: any) {
    console.warn("Failed to check for updates:", err);
    return {
      available: false,
      currentVersion,
      error: err?.message || String(err) || "检查更新失败，请确认网络连接。",
    };
  }
}

/**
 * 执行下载并安装更新
 */
export async function downloadAndInstallUpdate(
  update: Update,
  onProgress?: (downloaded: number, total: number, percent: number) => void
): Promise<void> {
  let totalLength = 0;
  let downloadedLength = 0;

  await update.downloadAndInstall((event) => {
    switch (event.event) {
      case "Started":
        totalLength = event.data.contentLength ?? 0;
        if (onProgress) {
          onProgress(0, totalLength, 0);
        }
        break;
      case "Progress":
        downloadedLength += event.data.chunkLength;
        if (onProgress && totalLength > 0) {
          const pct = Math.min(100, Math.round((downloadedLength / totalLength) * 100));
          onProgress(downloadedLength, totalLength, pct);
        }
        break;
      case "Finished":
        if (onProgress) {
          onProgress(totalLength, totalLength, 100);
        }
        break;
    }
  });
}
