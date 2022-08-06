import React from "react"
import * as THREE from "three";

export function ThreeImageDisplay() {
    const canvasRef = React.useRef<HTMLCanvasElement>(null);

    React.useEffect(() => {
        if (canvasRef.current === null) {
            return;
        }
        const scene = new THREE.Scene()
        const camera = new THREE.OrthographicCamera(0, 200, 0, 300, -10, 10)
        const renderer = new THREE.WebGLRenderer({ canvas: canvasRef.current })
        const geometry = new THREE.PlaneGeometry(200, 300)
        const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, side: THREE.DoubleSide });
        const cube = new THREE.Mesh(geometry, material)
        cube.translateY(150)
        cube.translateX(100)
        scene.add(cube);

        let cancel = false;
        function animate() {
            if (cancel) {
                return;
            }
            requestAnimationFrame(animate);
            renderer.render(scene, camera);
        }

        animate();
        return () => {
            cancel = true;
        }
    }, [canvasRef])

    return <><canvas ref={canvasRef} /></>
}