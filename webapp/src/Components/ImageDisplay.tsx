import React from "react";
import { State } from "../Controller";

import imtools, { Perlin, ScalarImage, GradientCMap, RgbaImage } from "imtools";

await imtools()

export type ImageDisplayProps = {
    state: State,
}

export function ImageDisplay(props: ImageDisplayProps) {
    const { state } = props;

    const canvasContainerEl = React.useRef<HTMLDivElement>(null);
    const canvasEl = React.useRef<HTMLCanvasElement>(null);

    const [shape, setShape] = React.useState<[number, number]>([100, 100]);

    React.useEffect(() => {
        if (!canvasContainerEl.current) {
            return;
        }
        const el = canvasContainerEl.current;
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
    }, [setShape, canvasContainerEl]);

    const [scalarImage, rgbaImage] = React.useMemo(() => {
        const ratio = shape[0] / shape[1];
        const width = 500;
        const height = width / ratio;
        if (canvasEl.current) {
            canvasEl.current.width = width;
            canvasEl.current.height = height;
        }

        return [new ScalarImage(width, height), new RgbaImage(width, height)]
    }, [shape, canvasEl])

    React.useEffect(() => {
        scalarImage.clear();
        for (const layerId in state.layers) {
            const layer = state.layers[layerId];
            new Perlin(layer.scale, layer.amp).addToImage(scalarImage);
        }
        new GradientCMap().cmap(scalarImage, rgbaImage)
    }, [scalarImage, state])

    React.useEffect(() => {
        if (canvasEl.current) {
            const ctx = canvasEl.current.getContext("2d") as CanvasRenderingContext2D;
            ctx.putImageData(rgbaImage.imageData(), 0, 0);
        }
    }, [rgbaImage, canvasEl])

    return (
        <div ref={canvasContainerEl} className="w-full h-full">
            <canvas ref={canvasEl} className="h-full"></canvas>
        </div>
    )
}
