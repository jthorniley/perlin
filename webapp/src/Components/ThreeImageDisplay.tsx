import React from "react"
import * as THREE from "three";

class Renderer {
    scene: THREE.Scene
    camera: THREE.OrthographicCamera
    renderer: THREE.WebGLRenderer
    mesh: THREE.Mesh
    private _cancelled: boolean = false

    constructor(canvas: HTMLCanvasElement) {
        this.scene = new THREE.Scene()
        this.camera = new THREE.OrthographicCamera(0, 200, 0, 300, -1, 1)
        this.renderer = new THREE.WebGLRenderer({ canvas })

        const geom = new THREE.PlaneGeometry(200, 300)
        const mat = new THREE.MeshBasicMaterial(
            { color: 0x00ff00, side: THREE.DoubleSide }
        )
        this.mesh = new THREE.Mesh(geom, mat)
        this.scene.add(this.mesh)
    }

    animate() {
        this.renderer.render(this.scene, this.camera)
        if (!this._cancelled) {
            requestAnimationFrame(() => this.animate())
        }
        return () => this.cancel()
    }

    cancel() {
        this._cancelled = true
    }
}

export function ThreeImageDisplay() {
    const canvasRef = React.useRef<HTMLCanvasElement>(null);
    React.useEffect(() => new Renderer(canvasRef.current!).animate())

    return <><canvas ref={canvasRef} /></>
}