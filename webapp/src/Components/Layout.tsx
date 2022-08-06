import React from "react";
import { useController } from "../Controller";
import { Controls } from "./Controls";
import { ThreeImageDisplay } from "./ThreeImageDisplay";

export function Layout() {
    const { state, reducer } = useController();
    return (
        <div className="flex w-full h-full justify-between">
            <div className="absolute overflow-auto h-1/3 sm:h-full w-full sm:w-auto bg-zinc-900 bg-opacity-50 bottom-0 sm:right-0">
                <div className="flex flex-col items-center">
                    <Controls state={state} reducer={reducer} layerId={0} />
                    <Controls state={state} reducer={reducer} layerId={1} />
                    <Controls state={state} reducer={reducer} layerId={2} />
                    <Controls state={state} reducer={reducer} layerId={3} />
                    <Controls state={state} reducer={reducer} layerId={4} />
                    <Controls state={state} reducer={reducer} layerId={5} />
                </div>
            </div>
            <div className="flex-grow overflow-hidden">
                <ThreeImageDisplay state={state} />
            </div>
        </div>
    )
}

