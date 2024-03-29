import React from "react";
import { State, Reducer } from "../Controller";
import { Slider2D } from "./Slider2D";

const PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139,
    149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
    233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
    331, 337, 347, 349, 353, 359]

function closestPrime(x: number): number {
    let distance = 1000000;
    let chosenPrime = 2
    for (const prime of PRIMES) {
        const currentDistance = Math.abs(prime - x)
        if (currentDistance < distance) {
            distance = currentDistance
            chosenPrime = prime
        } else {
            break
        }
    }
    return chosenPrime
}


export type ControlsProps = {
    state: State,
    reducer: Reducer
    layerId: number,
}

export function Controls(props: ControlsProps) {
    const { state, layerId, reducer } = props;

    const snapValue = React.useCallback((value: [number, number]) => {
        const [rawScale, amp] = value;
        const scale = closestPrime(rawScale);
        reducer({ setLayer: { layerId, scale, amp } })
    }, [reducer, layerId])

    const { scale, amp } = state.layers[layerId];

    return (
        <div className="m-3 bg-zinc-800 rounded-2xl drop-shadow-xl border-4 border-fuchsia-400">
            <div style={{ width: 250, height: 60 }}>
                <Slider2D
                    dataLimits={{ xMin: 1, xMax: 360, yMin: 0, yMax: 1 }}
                    screenLimits={{ xMin: 10, xMax: 240, yMin: 55, yMax: 5 }}
                    xGridLines={PRIMES}
                    setValue={snapValue}
                    value={[scale, amp]} />
            </div>
        </div>
    )
}
