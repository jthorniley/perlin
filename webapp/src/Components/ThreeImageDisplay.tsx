import React from "react"
import * as THREE from "three";

class Renderer {
    scene: THREE.Scene
    camera: THREE.OrthographicCamera
    renderer: THREE.WebGLRenderer
    mesh: THREE.Mesh
    private _cancelled: boolean = false

    constructor(container: HTMLElement) {
        let { width, height } = container.getBoundingClientRect();

        this.scene = new THREE.Scene()
        this.camera = new THREE.OrthographicCamera(-width / 2, width / 2, -height / 2, height / 2, -100, 100)
        this.renderer = new THREE.WebGLRenderer()
        this.renderer.setSize(width, height)
        container.appendChild(this.renderer.domElement)

        const geom = new THREE.PlaneGeometry(width, height)
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
    const containerRef = React.useRef<HTMLDivElement>(null);
    React.useEffect(() => new Renderer(containerRef.current!).animate())

    return <>
        <div className="flex flex-col justify-between h-full w-full absolute">
            <div className="w-full h-full" ref={containerRef} />

        </div>
    </>
}