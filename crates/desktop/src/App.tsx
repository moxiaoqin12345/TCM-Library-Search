import { createSignal, onMount, Show } from "solid-js";
import TitleBar from "./components/TitleBar";
import ReaderView from "./views/ReaderView";
import { UpdateModal } from "./components/UpdateModal";
import {
  checkForAppUpdate,
  getAppVersion,
  type UpdateCheckResult,
} from "./services/updater";
import styles from "./App.module.css";

export default function App() {
  const [totalCount, setTotalCount] = createSignal<number>(0);
  const [appVersion, setAppVersion] = createSignal<string>("0.2.0");
  const [isCheckingUpdate, setIsCheckingUpdate] = createSignal<boolean>(false);
  const [activeUpdateInfo, setActiveUpdateInfo] = createSignal<UpdateCheckResult | null>(null);
  const [lastCheckTime, setLastCheckTime] = createSignal<string>("");

  onMount(async () => {
    try {
      const ver = await getAppVersion();
      setAppVersion(ver);
    } catch {}

    // 应用启动 3 秒后执行一次后台静默检测
    setTimeout(async () => {
      try {
        const res = await checkForAppUpdate();
        setLastCheckTime(new Date().toLocaleTimeString());
        if (res.available) {
          setActiveUpdateInfo(res);
        }
      } catch {
        // 静默忽略
      }
    }, 3000);
  });

  const handleCheckUpdate = async (manual: boolean = true) => {
    setIsCheckingUpdate(true);
    try {
      const res = await checkForAppUpdate();
      setLastCheckTime(new Date().toLocaleTimeString());
      if (res.available) {
        setActiveUpdateInfo(res);
      } else if (manual) {
        if (res.error) {
          alert(`检查更新提示：${res.error}`);
        } else {
          alert(`当前已是最新版本 (v${res.currentVersion})，无需更新。`);
        }
      }
    } finally {
      setIsCheckingUpdate(false);
    }
  };

  return (
    <div class={styles.app}>
      <TitleBar
        totalCount={totalCount()}
        onCheckUpdate={() => handleCheckUpdate(true)}
        isCheckingUpdate={isCheckingUpdate()}
      />
      <div class={styles.content}>
        <ReaderView
          onTotalCountChange={setTotalCount}
          appVersion={appVersion()}
          onCheckUpdate={() => handleCheckUpdate(true)}
          isCheckingUpdate={isCheckingUpdate()}
          lastUpdateCheckTime={lastCheckTime()}
        />
      </div>

      {/* 发现新版本时的全屏弹窗 */}
      <Show when={activeUpdateInfo()}>
        <UpdateModal
          updateInfo={activeUpdateInfo()!}
          onClose={() => setActiveUpdateInfo(null)}
        />
      </Show>
    </div>
  );
}

