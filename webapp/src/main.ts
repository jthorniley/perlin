import imggen, { Perlin, RgbaImage } from "imtools";

await imggen();

const WIDTH = 1000;
const HEIGHT = 500;
const imageGenerator = RgbaImage.fill(WIDTH, HEIGHT, 200, 100, 0);

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.width = WIDTH;
canvas.height = HEIGHT;
const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

ctx.putImageData(imageGenerator.imageData(), 0, 0);