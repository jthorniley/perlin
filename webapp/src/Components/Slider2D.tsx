import React from "react"

const PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139,
    149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
    233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
    331, 337, 347, 349, 353, 359]

export type Slider2DProps = {
    scale: number,
    setScale: (_: number) => void
    amp: number,
    setAmp: (_: number) => void
    shape?: [number, number]
    chartOffset?: [number, number]
}

export function Slider2D(props: Slider2DProps) {
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