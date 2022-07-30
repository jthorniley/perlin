import React from "react";
import { State } from "../Controller";

import imtools, { Perlin, ScalarImage, GradientCMap } from "imtools";

await imtools()

export type ImageDisplayProps = {
    state: State,
}

export function ImageDisplay(props: ImageDisplayProps) {
    const { state } = props;
    const [shape, setShape] = React.useState<[number, number]>([100, 100]);
    React.useEffect(() => {
        const el = document.getElementById("canvasContainer")!;
        const { width, height } = el.getBoundingClientRect();
        setShape([Math.ceil(width), Math.ceil(height)]);

        const { signal, abort } = new AbortController();
        window.addEventListener("resize", () => {
            const { width, height } = el.getBoundingClientRect();
            setShape([Math.ceil(width), Math.ceil(height)]);
        }, { signal })

        return () => {
            abort();
        }
    }, [setShape]);

    React.useEffect(() => {
        const ratio = shape[0] / shape[1];
        const width = 500;
        const height = width / ratio;
        const imageGenerator = new ScalarImage(width, height);

        for (const layerId in state.layers) {
            const layer = state.layers[layerId];
            new Perlin(layer.scale, layer.amp).addToImage(imageGenerator);
        }

        const rgbaImage = new GradientCMap().cmap(imageGenerator)

        const canvas = document.getElementById("canvas") as HTMLCanvasElement;
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

        ctx.putImageData(rgbaImage.imageData(), 0, 0);

    }, [state, shape])

    return (
        <div id="canvasContainer" className="w-full h-full">
            <canvas id="canvas" className="h-full"></canvas>
        </div>
    )
}
