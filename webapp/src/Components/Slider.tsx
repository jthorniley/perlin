import React from "react"

export type SliderProps = {
    minValue: number
    maxValue: number
    value: number
    onChange?: (_: number) => void
}

export function Slider(props: SliderProps) {
    const { minValue, maxValue, value, onChange } = props;

    const percent = Math.round((value - minValue) / (maxValue - minValue) * 100)
    const offset = {
        width: `calc(${percent}% - 0.5rem)`
    }

    const onMove = React.useCallback((clientX: number, rect: DOMRect) => {
        if (onChange) {
            const { left, width } = rect;
            const relativePosition = Math.min(1, Math.max(0, (clientX - left) / width));

            onChange(minValue + relativePosition * (maxValue - minValue))
        }
    }, [onChange, minValue, maxValue])
    const onMouseMove = React.useCallback<React.MouseEventHandler>((ev) => {
        if (ev.buttons && onChange) {
            onMove(ev.clientX, ev.currentTarget.getBoundingClientRect())
        }
    }, [onMove])
    const onTouchMove = React.useCallback<React.TouchEventHandler>((ev) => {
        if (onChange) {
            onMove(ev.touches[0].clientX, ev.currentTarget.getBoundingClientRect())
        }
    }, [onMove])

    return (
        <div className="w-full rounded-md h-2 pt-2 pb-2 bg-fuchsia-100 flex items-center touch-none"
            onMouseMove={onMouseMove}
            onMouseDown={onMouseMove}
            onTouchMove={onTouchMove}
            onTouchStart={onTouchMove}
        >
            <div className="flex w-full">
                <div style={offset}></div>
                <div className="w-4 h-6 bg-fuchsia-400 rounded-lg">
                </div>
            </div>
        </div>
    )
}
