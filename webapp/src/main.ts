import './style.css'
import perlinrs, { test_fn } from "perlinrs-web";

await perlinrs();
console.log({ test: test_fn() });
const app = document.querySelector<HTMLDivElement>('#app')!

app.innerHTML = `
  <h1>Hello Vite!</h1>
  <a href="https://vitejs.dev/guide/features.html" target="_blank">Documentation</a>
`
