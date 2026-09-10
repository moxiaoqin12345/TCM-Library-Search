import { createSignal, onCleanup, onMount } from "solid-js";
import styles from "./Lightbox.module.css";

interface LightboxProps {
  src: string;
  title: string;
  caption?: string;
  onClose: () => void;
}

export default function Lightbox(props: LightboxProps) {
  const [scale, setScale] = createSignal(1);
  const [translateX, setTranslateX] = createSignal(0);
  const [translateY, setTranslateY] = createSignal(0);
  const [isDragging, setIsDragging] = createSignal(false);
  const [startX, setStartX] = createSignal(0);
  const [startY, setStartY] = createSignal(0);

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      props.onClose();
    }
  };

  onMount(() => {
    window.addEventListener("keydown", handleKeyDown);
  });

  onCleanup(() => {
    window.removeEventListener("keydown", handleKeyDown);
  });

  const zoomIn = () => setScale((s) => Math.min(s + 0.3, 4.0));
  const zoomOut = () => setScale((s) => Math.max(s - 0.3, 0.5));
  const resetZoom = () => {
    setScale(1);
    setTranslateX(0);
    setTranslateY(0);
  };

  const handleWheel = (e: WheelEvent) => {
    e.preventDefault();
    if (e.deltaY < 0) {
      zoomIn();
    } else {
      zoomOut();
    }
  };

  const handleMouseDown = (e: MouseEvent) => {
    if (e.button === 0) {
      setIsDragging(true);
      setStartX(e.clientX - translateX());
      setStartY(e.clientY - translateY());
    }
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (isDragging()) {
      setTranslateX(e.clientX - startX());
      setTranslateY(e.clientY - startY());
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  return (
    <div
      class={styles.overlay}
      onClick={(e) => {
        if (e.target === e.currentTarget) props.onClose();
      }}
    >
      <div class={styles.topBar}>
        <div class={styles.titleBox}>
          <span class={styles.title}>{props.title}</span>
          {props.caption && <span class={styles.caption}>{props.caption}</span>}
        </div>

        <div class={styles.controls}>
          <button type="button" class={styles.btn} onClick={zoomIn} title="放大">
            ➕ 放大
          </button>
          <button type="button" class={styles.btn} onClick={zoomOut} title="缩小">
            ➖ 缩小
          </button>
          <button type="button" class={styles.btn} onClick={resetZoom} title="重置">
            ↺ 重置 ({(scale() * 100).toFixed(0)}%)
          </button>
          <button
            type="button"
            class={`${styles.btn} ${styles.btnClose}`}
            onClick={props.onClose}
            title="关闭 (Esc)"
          >
            ✕ 关闭
          </button>
        </div>
      </div>

      <div
        class={styles.viewport}
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      >
        <img
          src={props.src}
          alt={props.title}
          class={styles.image}
          style={{
            transform: `translate(${translateX()}px, ${translateY()}px) scale(${scale()})`,
          }}
          draggable={false}
        />
      </div>
    </div>
  );
}
