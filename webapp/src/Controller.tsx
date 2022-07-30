import React from "react"

export type Layer = {
    scale: number
    amp: number
}

export type State = {
    layers: Record<number, Layer>
}

export type Action = { setLayer: { layerId: number, scale: number, amp: number } }

export type Reducer = (_: Action) => void;

export function useController(): { state: State, reducer: Reducer } {
    const [state, reducer] = React.useReducer((state: State, action: Action) => {
        if ("setLayer" in action) {
            const { layerId, scale, amp } = action.setLayer;
            if (layerId in state.layers) {
                state.layers[layerId] = { scale, amp }
            } else {
                throw new Error("No such layer")
            }
        }

        return { ...state }
    }, {
        layers: [
            { scale: 349, amp: 0.8 },
            { scale: 251, amp: 0.8 },
            { scale: 163, amp: 0.5 },
            { scale: 83, amp: 0.5 },
            { scale: 13, amp: 0.1 },
            { scale: 7, amp: 0.1 }
        ]
    })
    return { state, reducer }
}