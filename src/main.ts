import "./app.css";
import App from "./App.svelte";
import "nprogress/nprogress.css";

const app = new App({
  target: document.getElementById("app")!,
});

export default app;
