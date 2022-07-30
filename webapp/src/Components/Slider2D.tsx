import React, { MouseEventHandler, TouchEventHandler } from "react"

const CHART_BACKGROUND = "#86198f";
const HIGHTLIGHT = "#fae8ff";

export type Limits = { xMin: number, xMax: number, yMin: number, yMax: number }

class Transform {
    private _xDataRange: number
    private _yDataRange: number
    private _xScreenRange: number
    private _yScreenRange: number

    constructor(private _dataLimits: Limits, private _screenLimits: Limits) {
        this._xDataRange = _dataLimits.xMax - _dataLimits.xMin
        this._yDataRange = _dataLimits.yMax - _dataLimits.yMin
        this._xScreenRange = _screenLimits.xMax - _screenLimits.xMin
        this._yScreenRange = _screenLimits.yMax - _screenLimits.yMin
    }

    get xScreenRange(): number {
        return this._xScreenRange
    }

    get yScreenRange(): number {
        return this._yScreenRange
    }

    get screenLimits(): Limits {
        return this._screenLimits
    }

    get dataLimits(): Limits {
        return this._dataLimits
    }

    toDataX(value: number): number {
        const x = this._dataLimits.xMin + this._xDataRange * (value - this._screenLimits.xMin) / this._xScreenRange
        return Math.min(this._dataLimits.xMax, Math.max(this._dataLimits.xMin, x))
    }

    toDataY(value: number): number {
        const y = this._dataLimits.yMin + this._yDataRange * (value - this._screenLimits.yMin) / this._yScreenRange
        return Math.min(this._dataLimits.yMax, Math.max(this._dataLimits.yMin, y))
    }

    toData([x, y]: [number, number]): [number, number] {
        return [
            this.toDataX(x),
            this.toDataY(y)
        ]
    }

    toScreenX(value: number): number {
        const x = this._screenLimits.xMin + this._xScreenRange * (value - this._dataLimits.xMin) / this._xDataRange
        return Math.min(this._screenLimits.xMax, Math.max(this._screenLimits.xMin, x))
    }

    toScreenY(value: number): number {
        const y = this._screenLimits.yMin + this._yScreenRange * (value - this._dataLimits.yMin) / this._yDataRange
        return Math.max(this._screenLimits.yMax, Math.min(this._screenLimits.yMin, y))
    }

    toScreen([x, y]: [number, number]): [number, number] {
        return [
            this.toScreenX(x),
            this.toScreenY(y)
        ]
    }
}

export type Slider2DProps = {
    value: [number, number]
    setValue: (value: [number, number]) => void
    dataLimits?: Limits
    screenLimits?: Limits
    xGridLines?: number[]
}

export function Slider2D(props: Slider2DProps) {
    const { value, setValue, dataLimits, screenLimits, xGridLines } = props;

    const transform = React.useMemo(() => {
        return new Transform(
            dataLimits ?? { xMin: 0, xMax: 1, yMin: 0, yMax: 1 },
            screenLimits ?? { xMin: 10, xMax: 190, yMin: 190, yMax: 10 }
        )
    }, [dataLimits, screenLimits])

    const screenPosition = React.useMemo(() => {
        return transform.toScreen(value)
    }, [transform, value]);

    const onMouse = React.useCallback<MouseEventHandler>((ev) => {
        if (!ev.buttons) {
            return;
        }
        const { left, top } = ev.currentTarget.getBoundingClientRect();

        setValue(transform.toData([
            ev.clientX - left,
            ev.clientY - top
        ]))
    }, [transform, setValue])

    const onTouch = React.useCallback<TouchEventHandler>((ev) => {
        const { left, top } = ev.currentTarget.getBoundingClientRect();

        setValue(transform.toData([
            ev.touches[0].clientX - left,
            ev.touches[0].clientY - top
        ]))
    }, [transform, setValue])

    return (<div className="h-full w-full">
        <svg style={{ width: "100%", height: "100%", touchAction: "none" }}
            onMouseDown={onMouse}
            onMouseMove={onMouse}
            onTouchStart={onTouch}
            onTouchMove={onTouch}
        >
            <rect
                fill={CHART_BACKGROUND}
                x={transform.screenLimits.xMin} y={transform.screenLimits.yMax}
                width={transform.xScreenRange} height={-transform.yScreenRange}
            />
            {
                xGridLines && xGridLines.map((x) => <line
                    key={`line-${x}`}
                    x1={transform.toScreenX(x)} x2={transform.toScreenX(x)}
                    y1={transform.screenLimits.yMin} y2={transform.screenLimits.yMax}
                    style={{ stroke: HIGHTLIGHT, strokeWidth: 0.5 }}
                />)
            }

            <circle cx={screenPosition[0]} cy={screenPosition[1]} r="8" stroke="white" strokeWidth={2} fill="transparent" />
            <circle cx={screenPosition[0]} cy={screenPosition[1]} r="2" fill="white" />
        </svg>
    </div>)
}

