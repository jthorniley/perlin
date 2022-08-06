import React from "react"
import * as THREE from "three";
import imtools, { ScalarImage, Perlin, GradientCMap, RgbaImage } from "imtools";

await imtools();


class PerlinImage {
    private _texture: THREE.Texture

    constructor(private _width: number, private _height: number) {
        this._texture = this.init();
    }

    private init() {
        const image = new ScalarImage(this._width, this._height);
        const output = new RgbaImage(this._width, this._height);
        const perlin = new Perlin(100, 1);
        const cmap = new GradientCMap();

        perlin.addToImage(image);
        cmap.cmap(image, output);

        const data = output.array();
        const texture = new THREE.DataTexture(data, this._width, this._height, THREE.RGBAFormat)
        texture.needsUpdate = true;

        return texture;
    }

    get texture(): THREE.Texture {
        return this._texture;
    }

    refresh(width: number, height: number): boolean {
        // Rerender image as necessary. Return true if updated
        if (width !== this._width || height !== this._height) {
            this._width = width;
            this._height = height;
            this._texture = this.init();
            return true;
        }
        return false;
    }
}

class Renderer {
    private _scene: THREE.Scene
    private _camera: THREE.OrthographicCamera
    private _renderer: THREE.WebGLRenderer
    private _mesh: THREE.Mesh
    private _geom: THREE.PlaneGeometry

    private _cancelled: boolean = false
    private _abortResize: () => void = () => { }

    private _perlinImage: PerlinImage

    constructor(private _container: HTMLElement) {
        let [width, height] = [100, 100]

        this._scene = new THREE.Scene()
        this._camera = new THREE.OrthographicCamera(-width / 2, width / 2, -height / 2, height / 2, -100, 100)
        this._renderer = new THREE.WebGLRenderer()
        this._container.appendChild(this._renderer.domElement)

        this._geom = new THREE.PlaneGeometry(width, height)
        this._perlinImage = new PerlinImage(width, height);
        this._mesh = new THREE.Mesh(this._geom)
        this._scene.add(this._mesh)

        this._autoResize();
    }

    private _autoResize() {
        const resize = () => {
            const { width, height } = this._container.getBoundingClientRect();
            this._camera.left = -width / 2
            this._camera.right = width / 2
            this._camera.top = -height / 2
            this._camera.bottom = height / 2
            this._renderer.setSize(width, height)
        }

        const controller = new AbortController();
        window.addEventListener("resize", () => {
            resize()
        }, { signal: controller.signal })
        resize();
        this._abortResize = () => controller.abort();
    }


    animate() {
        const size = new THREE.Vector2()
        this._renderer.getSize(size)
        if (this._perlinImage.refresh(size.x, size.y)) {
            console.log("resized")
            this._mesh.material = new THREE.MeshBasicMaterial(
                { side: THREE.DoubleSide, map: this._perlinImage.texture }
            )
        }

        this._renderer.render(this._scene, this._camera)
        if (!this._cancelled) {
            requestAnimationFrame(() => this.animate())
        }
        return () => this._cancel()
    }

    private _cancel() {
        this._abortResize();
        this._cancelled = true
    }
}

export function ThreeImageDisplay() {
    const containerRef = React.useRef<HTMLDivElement>(null);
    React.useEffect(() => new Renderer(containerRef.current!).animate(), [])

    return <>
        <div className="w-full h-full" ref={containerRef} />
    </>
}