
import {draw} from "./render.js";
import {handle_mousemove,handle_keys,handle_wheel,fractal_selection} from "./event.js";
import init from "./pkg/fraktal.js"

const canvas = document.getElementById('fractal');

async function uruchom() {
// Inicjalizacja modułu WebAssembly
    await init();
    const select_fraktal = document.getElementById('choose_fractal');

    select_fraktal.addEventListener('change', fractal_selection);
    canvas.addEventListener('wheel', handle_wheel);
    window.addEventListener('keydown',handle_keys)
    canvas.addEventListener('mousemove', handle_mousemove);
    draw();
}


uruchom();
