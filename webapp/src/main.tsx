import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";

type SliderProps = {
    minValue: number
    maxValue: number
    defaultValue: number
    onChange?: (_: number) => void
}

function Slider(props: SliderProps) {
    const { minValue, maxValue, defaultValue, onChange } = props;
    const [position, setPosition] = React.useState(defaultValue);

    const percent = Math.round((position - minValue) / (maxValue - minValue) * 100)
    const offset = {
        width: `calc(${percent}% - 0.5rem)`
    }

    React.useEffect(() => {
        if (onChange) {
            onChange(position)
        }
    }, [onChange, position])

    const onMouseMove = React.useCallback<React.MouseEventHandler>((ev) => {
        if (ev.buttons) {
            const { left } = ev.currentTarget.getBoundingClientRect();
            const x = ev.clientX - left;
            const relativePosition = Math.min(1, Math.max(0, x / ev.currentTarget.clientWidth));

            setPosition(minValue + relativePosition * (maxValue - minValue))
        }
    }, [setPosition, position])
    const onTouchMove = React.useCallback<React.TouchEventHandler>((ev) => {
        if (ev) {
            const { left } = ev.currentTarget.getBoundingClientRect();
            const x = ev.touches[0].clientX - left;
            const relativePosition = Math.min(1, Math.max(0, x / ev.currentTarget.clientWidth));
            setPosition(minValue + relativePosition * (maxValue - minValue))
        }
    }, [setPosition, position])

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

function ImageDisplay() {
    return <></>;
}

function Controls() {
    const [scale, setScale] = React.useState(10);

    return (
        <div className="flex flex-col m-2">
            <div className="flex flex-col justify-between p-4 bg-zinc-800 rounded-2xl drop-shadow-xl border-4 border-fuchsia-400">
                <div className="text-fuchsia-400 mb-2">Scale: {scale}</div>
                <Slider
                    minValue={5}
                    maxValue={500}
                    defaultValue={10}
                    onChange={val => setScale(Math.round(val))}
                />
            </div>
        </div>
    )
}

function Layout() {
    return (
        <div className="flex w-full h-full justify-between">
            <div className="flex-grow">
                <ImageDisplay />
            </div>
            <div className="w-5 bg-gradient-to-r from-transparent to-fuchsia-900 opacity-10"></div>
            <div className="h-full w-96 bg-fuchsia-900 bg-opacity-10 shadow-lg">
                <Controls />
            </div>
        </div>
    )
}

function App() {
    return (
        <div className="flex w-full h-full overflow-auto absolute">
            <Layout />
        </div >
    )
}

const root = createRoot(document.getElementById("app")!);
root.render(<App />)