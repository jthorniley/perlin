import React from "react";
import { useController } from "../Controller";
import { Controls } from "./Controls";
import { ImageDisplay } from "./ImageDisplay";

export function Layout() {
    const { state, reducer } = useController();
    return (
        <div className="flex w-full h-full justify-between">
            <div className="absolute h-full w-auto bg-zinc-900 bg-opacity-50 right-0">
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

