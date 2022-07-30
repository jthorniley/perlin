import React from "react";
import { State, Reducer } from "../Controller";
import { Slider } from "./Slider";

export type ControlsProps = {
    state: State,
    reducer: Reducer
    layerId: number,
}

export function Controls(props: ControlsProps) {
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
