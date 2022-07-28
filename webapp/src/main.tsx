import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";


function Slider() {
    let [position, setPosition] = React.useState(0);
    const [isMouseDown, setIsMouseDown] = React.useState(false);

    let offset = {
        width: `${position}%`
    }

    const onMouseMove = React.useCallback((ev: React.MouseEvent) => {
        console.debug("onMouseMove", ev)
        if (ev.buttons) {
            setPosition(position + ev.movementX)
        }
    }, [isMouseDown, setPosition, position])

    return (
        <div className="w-full rounded-md h-2 mt-2 mb-2 bg-slate-300 flex items-center"
            onMouseMove={onMouseMove}
        >
            <div className="flex w-full">
                <div style={offset}></div>
                <div className="w-4 h-4 bg-violet-700 rounded-lg z-10"
                >

                </div>
            </div>
        </div>
    )
}

function ImageDisplay() {
    return <></>;
}

function Controls() {
    return (
        <div className="flex flex-col">
            <div className="flex flex-col justify-between p-3 bg-slate-800 rounded-2xl drop-shadow-xl border-4 border-slate-400">
                <Slider />
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
            <div className="w-5 bg-gradient-to-r from-transparent to-white opacity-10"></div>
            <div className="h-full w-96 bg-white bg-opacity-10 shadow-lg">
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