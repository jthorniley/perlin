import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";
import { useParameters, Parameters } from "./Parameters";
import imtools, { Perlin, ScalarImage, RgbaImage, GradientCMap } from "imtools";

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

    const onMouseMove = React.useCallback<React.MouseEventHandler>((ev) => {
        if (ev.buttons && onChange) {
            const { left } = ev.currentTarget.getBoundingClientRect();
            const x = ev.clientX - left;
            const relativePosition = Math.min(1, Math.max(0, x / ev.currentTarget.clientWidth));

            onChange(minValue + relativePosition * (maxValue - minValue))
        }
    }, [onChange, minValue, maxValue])
    const onTouchMove = React.useCallback<React.TouchEventHandler>((ev) => {
        if (onChange) {
            const { left } = ev.currentTarget.getBoundingClientRect();
            const x = ev.touches[0].clientX - left;
            const relativePosition = Math.min(1, Math.max(0, x / ev.currentTarget.clientWidth));
            onChange(minValue + relativePosition * (maxValue - minValue))
        }
    }, [onChange, minValue, maxValue])

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
    parameters: Parameters
}

function ImageDisplay(props: ImageDisplayProps) {
    const { parameters } = props;

    React.useEffect(() => {
        console.warn("rendering")
        const WIDTH = 2000;
        const HEIGHT = 2000;
        const imageGenerator = new ScalarImage(WIDTH, HEIGHT);

        new Perlin(parameters.scale, 0.5).addToImage(imageGenerator);

        const rgbaImage = new GradientCMap().cmap(imageGenerator)

        const canvas = document.getElementById("canvas") as HTMLCanvasElement;
        canvas.width = WIDTH;
        canvas.height = HEIGHT;
        const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

        ctx.putImageData(rgbaImage.imageData(), 0, 0);

    }, [parameters.scale])

    return <canvas id="canvas"></canvas>;
}

type ControlsProps = {
    parameters: Parameters
}
function Controls(props: ControlsProps) {
    const { parameters } = props;

    return (
        <div className="flex flex-col m-2">
            <div className="flex flex-col justify-between p-4 bg-zinc-800 rounded-2xl drop-shadow-xl border-4 border-fuchsia-400">
                <div className="text-fuchsia-400 mb-2">Scale: {parameters.scale}</div>
                <Slider
                    minValue={5}
                    maxValue={500}
                    value={parameters.scale}
                    onChange={val => parameters.scale = Math.round(val)}
                />
            </div>
        </div>
    )
}

function Layout() {
    const parameters = useParameters();
    return (
        <div className="flex w-full h-full justify-between">
            <div className="absolute h-full w-96 bg-zinc-900 opacity-70 right-0">
            </div>
            <div className="absolute h-full w-96 right-0">
                <Controls parameters={parameters} />
            </div>
            <div className="flex-grow">
                <ImageDisplay parameters={parameters} />
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