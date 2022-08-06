import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";
import { App } from "./Components/App";
import { Slider2DDemo } from "./Components/Slider2DDemo";
import { ThreeImageDisplay } from "./Components/ThreeImageDisplay";

if (window.location.search === "?slidertest") {
    const root = createRoot(document.getElementById("app")!);
    root.render(<Slider2DDemo />)
} else if (window.location.search === "?3dtest") {
    const root = createRoot(document.getElementById("app")!);
    root.render(<ThreeImageDisplay />)
} else {
    const root = createRoot(document.getElementById("app")!);
    root.render(<App />)
}
