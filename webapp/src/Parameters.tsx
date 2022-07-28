import React from "react"

type PerlinParameters = {
    scale: number
}

type State = {
    perlin: Array<PerlinParameters>
}

type Action = { setScale: number }

export interface Parameters {
    scale: number
}

class ParametersImpl implements Parameters {
    constructor(private _state: State, private _reducer: (_action: Action) => void) { }

    get scale() {
        return this._state.perlin[0].scale;
    }

    set scale(value: number) {
        this._reducer({ setScale: value })
    }
}

export function useParameters(): Parameters {
    const [state, reducer] = React.useReducer((state: State, action: Action) => {
        state.perlin = [{ scale: action.setScale }]
        return { ...state }
    }, { perlin: [{ scale: 10 }] })

    return React.useMemo(() => {
        return new ParametersImpl(state, reducer)
    }, [state, reducer])
}