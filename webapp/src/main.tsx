import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";
import { Slider2D } from "./Components/Slider2D";
import { App } from "./Components/App";

if (window.location.search == "?slidertest") {
    const root = createRoot(document.getElementById("app")!);
    root.render(<Slider2D />)
} else {
    const root = createRoot(document.getElementById("app")!);
    root.render(<App />)
}
