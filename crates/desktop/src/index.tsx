import { render } from "solid-js/web";
import App from "./App";
import "./theme/theme.css";
import "./theme/theme"; // 触发主题与字号初始化

const root = document.getElementById("root");

if (root) {
  render(() => <App />, root);
}
