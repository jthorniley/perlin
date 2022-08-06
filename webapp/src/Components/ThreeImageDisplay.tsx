import React from "react"
import * as THREE from "three";
import imtools, { ScalarImage, Perlin, GradientCMap, RgbaImage } from "imtools";

await imtools();

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

        const image = new ScalarImage(width, height);
        const output = new RgbaImage(width, height);
        const perlin = new Perlin(100, 1);
        const cmap = new GradientCMap();

        perlin.addToImage(image);
        cmap.cmap(image, output);

        const data = output.array();
        const texture = new THREE.DataTexture(data, width, height, THREE.RGBAFormat)
        texture.needsUpdate = true;

        const mat = new THREE.MeshBasicMaterial(
            { side: THREE.DoubleSide, map: texture }
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