import './style.css'
import perlinrs, { PerlinNoise } from "perlinrs-web";

await perlinrs();

const noise = new PerlinNoise();
console.log({ data: noise.get_data() });
