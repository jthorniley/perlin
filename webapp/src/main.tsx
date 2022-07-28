import "./styles.css";
import React from "react";
import { createRoot } from "react-dom/client";

function App() {
    console.log("app")
    return <div className="text-3xl font-bold">Hello world</div>
}

const root = createRoot(document.getElementById("app")!);
root.render(<App />)