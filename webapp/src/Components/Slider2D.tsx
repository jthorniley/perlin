import React, { MouseEventHandler } from "react"

const PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139,
    149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
    233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
    331, 337, 347, 349, 353, 359]

const CHART_BACKGROUND = "#86198f";
const HIGHTLIGHT = "#fae8ff";

export type Limits = { xMin: number, xMax: number, yMin: number, yMax: number }

class Transform {
    private _xDataRange: number
    private _yDataRange: number
    private _xScreenRange: number
    private _yScreenRange: number

    constructor(private dataLimits: Limits, private screenLimits: Limits) {
        this._xDataRange = dataLimits.xMax - dataLimits.xMin
        this._yDataRange = dataLimits.yMax - dataLimits.yMin
        this._xScreenRange = screenLimits.xMax - screenLimits.xMin
        this._yScreenRange = screenLimits.yMax - screenLimits.yMin
    }

    get xScreenRange(): number {
        return this._xScreenRange
    }

    get yScreenRange(): number {
        return this._yScreenRange
    }

    toDataX(value: number): number {
        const x = this.dataLimits.xMin + this._xDataRange * (value - this.screenLimits.xMin) / this._xScreenRange
        return Math.min(this.dataLimits.xMax, Math.max(this.dataLimits.xMin, x))
    }

    toDataY(value: number): number {
        const y = this.dataLimits.yMin + this._yDataRange * (value - this.screenLimits.yMin) / this._yScreenRange
        return Math.min(this.dataLimits.yMax, Math.max(this.dataLimits.yMin, y))
    }

    toScreenX(value: number): number {
        const x = this.screenLimits.xMin + this._xScreenRange * (value - this.dataLimits.xMin) / this._xDataRange
        return Math.min(this.screenLimits.xMax, Math.max(this.screenLimits.xMin, x))
    }

    toScreenY(value: number): number {
        const y = this.screenLimits.yMin + this._yScreenRange * (value - this.dataLimits.yMin) / this._yDataRange
        return Math.max(this.screenLimits.yMax, Math.min(this.screenLimits.yMin, y))
    }
}

export type Slider2DProps = {
    value: [number, number]
    setValue: (value: [number, number]) => void
    dataLimits?: Limits
    screenLimits?: Limits
}

export function Slider2D(props: Slider2DProps) {
    const { value, setValue, dataLimits, screenLimits } = props;

    const transform = React.useMemo(() => {
        return new Transform(
            dataLimits ?? { xMin: 2, xMax: 200, yMin: 0, yMax: 1 },
            screenLimits ?? { xMin: 10, xMax: 190, yMin: 190, yMax: 10 }
        )
    }, [dataLimits, screenLimits])

    const screenPosition = [transform.toScreenX(value[0]), transform.toScreenY(value[1])];

    const onMouse = React.useCallback<MouseEventHandler>((ev) => {
        const { left, top } = ev.currentTarget.getBoundingClientRect();
        const [screenX, screenY] = [
            ev.clientX - left,
            ev.clientY - top
        ]

        setValue([
            transform.toDataX(screenX),
            transform.toDataY(screenY)
        ])
    }, [transform, setValue])

    return (<div className="h-full w-full">
        <svg style={{ width: "100%", height: "100%" }}
            onMouseMove={onMouse}
        >
            <rect
                fill={CHART_BACKGROUND}
                x={transform.toScreenX(0)} y={transform.toScreenY(1)}
                width={transform.xScreenRange} height={-transform.yScreenRange}
            />

            <circle cx={screenPosition[0]} cy={screenPosition[1]} r="5" stroke="white" fill="transparent" />
            <circle cx={screenPosition[0]} cy={screenPosition[1]} r="1" fill="white" />
        </svg>
    </div>)
}

export type Slider2DXProps = {
    scale: number,
    setScale: (_: number) => void
    amp: number,
    setAmp: (_: number) => void
    shape?: [number, number]
    chartOffset?: [number, number]
}

export function Slider2DX(props: Slider2DXProps) {
    const { scale, setScale, amp, setAmp, shape, chartOffset } = props;
    const [width, height] = shape ?? [200, 200];
    const [chartOffsetX, chartOffsetY] = chartOffset ?? [10, 10];

    const positionX = React.useMemo(() => {
        return chartOffsetX + scale / 2 - 1
    }, [scale])
    const positionY = React.useMemo(() => {
        return height - (chartOffsetY + amp * (height - chartOffsetY * 2))
    }, [amp])

    const chartBackground = "#86198f";
    const primeHighlight = "#fae8ff";

    const svgRef = React.useRef<SVGSVGElement>(null);
    const setX = React.useCallback((clientX: number) => {
        if (!svgRef.current) {
            return;
        }
        const { left } = svgRef.current.getBoundingClientRect();
        const x = clientX - left;

        let dist = 100000;
        let val = 0;
        for (const prime of PRIMES) {
            const primeLoc = chartOffsetX + prime / 2 - 1;
            const nextDist = Math.abs(primeLoc - x);
            if (nextDist < dist) {
                dist = nextDist
                val = prime
            } else {
                break;
            }
        }
        setScale(val);
    }, [svgRef, setScale])

    const setY = React.useCallback((clientY: number) => {
        if (!svgRef.current) {
            return;
        }
        const { bottom } = svgRef.current.getBoundingClientRect();
        const y = bottom - clientY;
        setAmp(Math.min(1, Math.max(0, (y - chartOffsetY) / (height - 2 * chartOffsetY))))
    }, [svgRef, setAmp])

    const onMouseMove = React.useCallback<React.MouseEventHandler>((ev) => {
        if (ev.buttons) {
            setX(ev.clientX)
            setY(ev.clientY)
        }
    }, [setX, setY])

    return (
        <div className="flex bg-zinc-900 overflow-hidden justify-start" style={{ width, height }}>
            <svg
                ref={svgRef}
                width={width} height={height}
                onMouseMove={onMouseMove}
                onMouseDown={onMouseMove}
            >
                <rect
                    fill={chartBackground}
                    x={chartOffsetX} y={chartOffsetY}
                    width={width - 2 * chartOffsetX} height={height - 2 * chartOffsetY}
                />
                {
                    PRIMES.map(prime => <line
                        key={`line-${prime}`}
                        x1={chartOffsetX + prime / 2 - 1} x2={chartOffsetX + prime / 2 - 1}
                        y1={chartOffsetY} y2={height - chartOffsetY}
                        style={{ stroke: primeHighlight, strokeWidth: 0.5 }}
                    />)
                }
                <circle cx={positionX} cy={positionY} r="5" stroke="white" fill="transparent" />
                <circle cx={positionX} cy={positionY} r="1" fill="white" />
            </svg>
        </div >
    )
}