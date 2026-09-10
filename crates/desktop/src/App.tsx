import { createSignal } from "solid-js";
import TitleBar from "./components/TitleBar";
import ReaderView from "./views/ReaderView";
import styles from "./App.module.css";

export default function App() {
  const [totalCount, setTotalCount] = createSignal<number>(0);

  return (
    <div class={styles.app}>
      <TitleBar totalCount={totalCount()} />
      <div class={styles.content}>
        <ReaderView onTotalCountChange={setTotalCount} />
      </div>
    </div>
  );
}
