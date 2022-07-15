import './style.css'
import perlinrs, { PerlinNoise } from "perlinrs-web";

await perlinrs();

const noise = new PerlinNoise();
console.log(noise.raw_data())

const app = document.querySelector<HTMLDivElement>('#app')!

app.innerHTML = `
  <h1>Hello Vite!</h1>
  <a href="https://vitejs.dev/guide/features.html" target="_blank">Documentation</a>
`
