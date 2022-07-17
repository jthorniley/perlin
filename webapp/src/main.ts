import imggen, { ScalarImage, Perlin } from "imtools";

await imggen();

const WIDTH = 1000;
const HEIGHT = 500;
const imageGenerator = new ScalarImage(WIDTH, HEIGHT);

new Perlin(200, 1.1).addToImage(imageGenerator);
new Perlin(140, 0.1).addToImage(imageGenerator);
new Perlin(90, 0.1).addToImage(imageGenerator);
new Perlin(70, 0.1).addToImage(imageGenerator);
new Perlin(40, 0.04).addToImage(imageGenerator);
new Perlin(10, 0.02).addToImage(imageGenerator);


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