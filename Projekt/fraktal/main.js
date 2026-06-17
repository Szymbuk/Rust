
import {draw} from "./render.js";
import {handle_mousemove,handle_keys,handle_wheel,fractal_selection,handle_input} from "./event.js";
import init, {initThreadPool} from "./pkg/fraktal.js"

const canvas = document.getElementById('fractal');

async function uruchom() {
// Inicjalizacja modułu WebAssembly
    await init();

    const logicalCores = navigator.hardwareConcurrency || 4;

    await initThreadPool(logicalCores);
    console.log(`Uruchomiono wielowątkowość na ${logicalCores} rdzeniach!`);



    const select_fraktal = document.getElementById('choose_fractal');
    const param_inputs = [
        document.getElementById('input_c_re'),
        document.getElementById('input_c_im'),
        document.getElementById('input_julia_iterations'),
        document.getElementById('input_seed'),
        document.getElementById('input_barnsley_iterations'),
        document.getElementById('input_mandelbrot_iterations'),
    ];

// Przypinamy do każdego z nich ten sam mechanizm
    param_inputs.forEach(handle_input);


    select_fraktal.addEventListener('change', fractal_selection);
    canvas.addEventListener('wheel', handle_wheel);
    window.addEventListener('keydown',handle_keys)
    canvas.addEventListener('mousemove', handle_mousemove);

    let mandelbrot_params = document.getElementById("mandelbrot_params");
    mandelbrot_params.style.display = "block"
    draw();
}


uruchom();
