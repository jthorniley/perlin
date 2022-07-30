import React from "react";
import { Slider2D } from "./Slider2D";

export function Slider2DDemo() {
    const [scale, setScale] = React.useState(17)

    return (
        <div className="flex flex-col">
            <Slider2D scale={scale} setScale={setScale} />
            <div className="text-fuchsia-100">Scale: {scale}</div>
        </div>
    )
}