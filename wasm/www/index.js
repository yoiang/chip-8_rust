import { Index } from "../pkg/chip8_wasm";

const pre = document.getElementById("chip8_render-canvas");
const index = Index.new();

const renderLoop = () => {
    pre.textContent = index.render_text();
    index.update();
  
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);