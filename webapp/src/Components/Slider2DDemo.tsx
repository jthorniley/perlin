import React from "react";
import { Slider2D } from "./Slider2D";

export function Slider2DDemo() {
    const [scale, setScale] = React.useState(17)
    const [amp, setAmp] = React.useState(0.5);

    return (
        <div className="flex flex-col">
            <Slider2D scale={scale} setScale={setScale} amp={amp} setAmp={setAmp} />
            <div className="text-fuchsia-100">Scale: {scale}</div>
            <div className="text-fuchsia-100">Amp: {amp.toFixed(3)}</div>
        </div>
    )
}