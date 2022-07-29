import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";
import { Reducer, State, useController } from "./Controller";
import imtools, { Perlin, ScalarImage, GradientCMap } from "imtools";

await imtools();

type SliderProps = {
    minValue: number
    maxValue: number
    value: number
    onChange?: (_: number) => void
}

function Slider(props: SliderProps) {
    const { minValue, maxValue, value, onChange } = props;

    const percent = Math.round((value - minValue) / (maxValue - minValue) * 100)
    const offset = {
        width: `calc(${percent}% - 0.5rem)`
    }

    const onMove = React.useCallback((clientX: number, rect: DOMRect) => {
        if (onChange) {
            const { left, width } = rect;
            const relativePosition = Math.min(1, Math.max(0, (clientX - left) / width));

            onChange(minValue + relativePosition * (maxValue - minValue))
        }
    }, [onChange, minValue, maxValue])
    const onMouseMove = React.useCallback<React.MouseEventHandler>((ev) => {
        if (ev.buttons && onChange) {
            onMove(ev.clientX, ev.currentTarget.getBoundingClientRect())
        }
    }, [onMove])
    const onTouchMove = React.useCallback<React.TouchEventHandler>((ev) => {
        if (onChange) {
            onMove(ev.touches[0].clientX, ev.currentTarget.getBoundingClientRect())
        }
    }, [onMove])

    return (
        <div className="w-full rounded-md h-2 pt-2 pb-2 bg-fuchsia-100 flex items-center touch-none"
            onMouseMove={onMouseMove}
            onMouseDown={onMouseMove}
            onTouchMove={onTouchMove}
            onTouchStart={onTouchMove}
        >
            <div className="flex w-full">
                <div style={offset}></div>
                <div className="w-4 h-6 bg-fuchsia-400 rounded-lg">
                </div>
            </div>
        </div>
    )
}

type ImageDisplayProps = {
    state: State,
}

function ImageDisplay(props: ImageDisplayProps) {
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
            new Perlin(layer.scale, Math.pow(0.4, parseInt(layerId) * 2)).addToImage(imageGenerator);
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

type ControlsProps = {
    state: State,
    reducer: Reducer
    layerId: number,
}
function Controls(props: ControlsProps) {
    const { state, layerId, reducer } = props;

    const { scale } = state.layers[layerId];

    return (
        <div className="flex flex-col m-2">
            <div className="flex justify-between items-center p-4 bg-zinc-800 rounded-2xl drop-shadow-xl border-4 border-fuchsia-400">
                <div className="text-fuchsia-400 mr-2">Scale: </div>
                <Slider
                    minValue={5}
                    maxValue={300}
                    value={scale}
                    onChange={val => reducer({ setScale: { layerId: layerId, scale: val } })}
                />
            </div>
        </div>
    )
}

function Layout() {
    const { state, reducer } = useController();
    return (
        <div className="flex w-full h-full justify-between">
            <div className="absolute h-full w-96 bg-zinc-900 opacity-70 right-0">
            </div>
            <div className="absolute h-full w-96 right-0">
                <Controls state={state} reducer={reducer} layerId={0} />
                <Controls state={state} reducer={reducer} layerId={1} />
                <Controls state={state} reducer={reducer} layerId={2} />
            </div>
            <div className="flex-grow overflow-hidden">
                <ImageDisplay state={state} />
            </div>
        </div>
    )
}

function App() {
    return (
        <div className="flex w-full h-full overflow-hidden absolute">
            <Layout />
        </div >
    )
}

const root = createRoot(document.getElementById("app")!);
root.render(<App />)