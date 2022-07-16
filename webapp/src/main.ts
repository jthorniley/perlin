import './style.css'
import perlinrs, { ImageGenerator } from "perlinrs-web";

await perlinrs();

const noise = new ImageGenerator(20, 30);
console.log({ data: noise.as_array() });
