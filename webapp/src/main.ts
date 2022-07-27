import imggen, { GradientCMap, Perlin, ScalarImage } from "imtools";

await imggen();

const WIDTH = 1000;
const HEIGHT = 500;
const imageGenerator = new ScalarImage(WIDTH, HEIGHT);

new Perlin(200, 0.5).addToImage(imageGenerator);
new Perlin(143, 0.3).addToImage(imageGenerator);
new Perlin(11, 0.04).addToImage(imageGenerator);

const rgbaImage = new GradientCMap().cmap(imageGenerator)

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.width = WIDTH;
canvas.height = HEIGHT;
const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

ctx.putImageData(rgbaImage.imageData(), 0, 0);