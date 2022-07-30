import React from "react";
import { Slider2D, Slider2DX } from "./Slider2D";

export function Slider2DDemo() {
    const [value, setValue] = React.useState<[number, number]>([200, 1]);
    return (
        <div className=" border-lime-400 border-2 m-6 p-6" style={{ width: 250, height: 250 }}>
            <Slider2D screenLimits={{ xMin: 40, xMax: 190, yMin: 190, yMax: 30 }} {...{ value, setValue }} />
        </div>
    )
    /*    return (
            <div className="flex flex-col">
                <Slider2DX scale={scale} setScale={setScale} amp={amp} setAmp={setAmp} />
                <div className="text-fuchsia-100">Scale: {scale}</div>
                <div className="text-fuchsia-100">Amp: {amp.toFixed(3)}</div>
            </div>
        )*/
}