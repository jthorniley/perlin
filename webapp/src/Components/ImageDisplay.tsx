import React from "react";
import { State } from "../Controller";

import imtools, { Perlin, ScalarImage, GradientCMap, RgbaImage } from "imtools";

await imtools()

function useSetCanvasSizeToContainer(container: React.RefObject<HTMLElement>, canvas: React.RefObject<HTMLCanvasElement>) {
    const [shape, setShape] = React.useState<[number, number]>([100, 100]);

    const setContainerShape = React.useCallback((containerShape: [number, number]) => {
        const ratio = containerShape[0] / containerShape[1];
        const width = Math.min(800, containerShape[0]);
        const height = width / ratio;
        if (canvas.current) {
            canvas.current.width = width;
            canvas.current.height = height;
        }
        setShape([width, height])
    }, [canvas])

    React.useEffect(() => {
        if (!container.current) {
            return;
        }
        const el = container.current;
        const { width, height } = el.getBoundingClientRect();
        setContainerShape([Math.ceil(width), Math.ceil(height)]);

        const { signal, abort } = new AbortController();
        window.addEventListener("resize", () => {
            const { width, height } = el.getBoundingClientRect();
            setContainerShape([Math.ceil(width), Math.ceil(height)]);
        }, { signal })

        return () => {
            abort();
        }
    }, [setContainerShape, container]);

    return shape;
}

function usePerlinNoise(width: number, height: number, state: State) {
    const [scalarImage, rgbaImage] = React.useMemo(() => {
        return [new ScalarImage(width, height), new RgbaImage(width, height)]
    }, [width, height])

    React.useEffect(() => {
        scalarImage.clear();
        for (const layerId in state.layers) {
            const layer = state.layers[layerId];
            new Perlin(layer.scale, layer.amp).addToImage(scalarImage);
        }
        new GradientCMap().cmap(scalarImage, rgbaImage)
    }, [scalarImage, state])

    return rgbaImage;
}

function putImageToCanvas(rgbaImage: RgbaImage, canvas: React.RefObject<HTMLCanvasElement>) {
    if (canvas.current) {
        const ctx = canvas.current.getContext("2d") as CanvasRenderingContext2D;
        ctx.putImageData(rgbaImage.imageData(), 0, 0);
    }
}

export type ImageDisplayProps = {
    state: State,
}

export function ImageDisplay(props: ImageDisplayProps) {
    const { state } = props;

    const container = React.useRef<HTMLDivElement>(null);
    const canvas = React.useRef<HTMLCanvasElement>(null);

    const [width, height] = useSetCanvasSizeToContainer(container, canvas);

    const rgbaImage = usePerlinNoise(width, height, state);

    React.useEffect(() => {
        putImageToCanvas(rgbaImage, canvas);
    }, [rgbaImage, canvas, state])

    return (
        <div ref={container} className="w-full h-full">
            <canvas ref={canvas} className="h-full"></canvas>
        </div>
    )
}
