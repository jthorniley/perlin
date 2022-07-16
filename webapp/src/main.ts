import perlinrs, { ImageGenerator } from "perlinrs-web";

await perlinrs();

const WIDTH = 500;
const HEIGHT = 600;
const noise = new ImageGenerator(WIDTH, HEIGHT);
noise.add_perlin_noise(200, 0.4);
noise.add_perlin_noise(140, 0.1);
noise.add_perlin_noise(90, 0.1);
noise.add_perlin_noise(70, 0.1);
noise.add_perlin_noise(40, 0.04);
noise.add_perlin_noise(10, 0.02);


const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.width = WIDTH;
canvas.height = HEIGHT;
const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

const imageData = ctx.createImageData(WIDTH, HEIGHT);

let i = 0;
(noise.as_array() as Float32Array).forEach(element => {
    const level = Math.max(Math.min(element + 0.5, 1.0), 0.0) * 255.0;
    imageData.data[i] = level;
    imageData.data[i + 1] = level;
    imageData.data[i + 2] = level;
    imageData.data[i + 3] = 255;
    i = i + 4;
});
ctx.putImageData(imageData, 0, 0);