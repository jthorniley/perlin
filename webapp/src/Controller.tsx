import React from "react"

export type Layer = {
    scale: number
}

export type State = {
    layers: Record<number, Layer>
}

export type Action = { setScale: { layerId: number, scale: number } }

export type Reducer = (_: Action) => void;

export function useController(): { state: State, reducer: Reducer } {
    const [state, reducer] = React.useReducer((state: State, action: Action) => {
        if ("setScale" in action) {
            const { layerId, scale } = action.setScale;
            if (layerId in state.layers) {
                state.layers[layerId] = { scale }
            } else {
                throw new Error("No such layer")
            }
        }

        return { ...state }
    }, { layers: [{ scale: 100 }, { scale: 50 }, { scale: 10 }] })
    return { state, reducer }
}