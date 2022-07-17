import imggen, { ImageGenerator } from "imtools";

await imggen();

const WIDTH = 1000;
const HEIGHT = 500;
const imageGenerator = new ImageGenerator(WIDTH, HEIGHT);
imageGenerator.addPerlinNoise(200, 0.4);
imageGenerator.addPerlinNoise(140, 0.1);
imageGenerator.addPerlinNoise(90, 0.1);
imageGenerator.addPerlinNoise(70, 0.1);
imageGenerator.addPerlinNoise(40, 0.04);
imageGenerator.addPerlinNoise(10, 0.02);


const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.width = WIDTH;
canvas.height = HEIGHT;
const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

const imageData = ctx.createImageData(WIDTH, HEIGHT);

let i = 0;
imageGenerator.imageData().forEach(element => {
    const level = Math.max(Math.min(element + 0.5, 1.0), 0.0) * 255.0;
    imageData.data[i] = level;
    imageData.data[i + 1] = level;
    imageData.data[i + 2] = level;
    imageData.data[i + 3] = 255;
    i = i + 4;
});

ctx.putImageData(imageData, 0, 0);