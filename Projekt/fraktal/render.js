import {state} from "./state.js";
import  { generate_mandelbrot, MandelbrotConfig,
    generate_julia, JuliaConfig,
    generate_barnsley, BarnsleyConfig} from './pkg/fraktal.js';

function draw(){
    const canvas = document.getElementById('fractal');

    const ctx = canvas.getContext('2d');
    const width = canvas.width;
    const height = canvas.height;
    let surowePiksele;


    if (state.current_fractal === "mandelbrot") {
        state.iterations = parseInt(document.getElementById("input_mandelbrot_iterations").value)
        const start_time = performance.now();
        const config = new MandelbrotConfig(state.x_min, state.x_max, state.y_min, state.y_max, state.iterations, width, height);
        surowePiksele = generate_mandelbrot(config);
        const end_time = performance.now();
        const time_taken = (end_time - start_time).toFixed(2);
        console.log(`[Rust] Wygenerowanie fraktala zajęło: ${time_taken} ms`);
    }
    else if (state.current_fractal === "julia"){
        let c_re = parseFloat(document.getElementById('input_c_re').value);
        let c_im = parseFloat(document.getElementById('input_c_im').value);
        state.iterations = parseInt(document.getElementById("input_julia_iterations").value)

        const config = new JuliaConfig(state.x_min, state.x_max, state.y_min, state.y_max, state.iterations, width, height, c_re, c_im);
        const start_time = performance.now();
        surowePiksele = generate_julia(config);
        const end_time = performance.now();
        const time_taken = (end_time - start_time).toFixed(2);
        console.log(`[Rust] Wygenerowanie fraktala zajęło: ${time_taken} ms`);
    }
    else if (state.current_fractal === "barnsley"){
        let seed = BigInt(document.getElementById('input_seed').value)
        state.iterations = parseInt(document.getElementById("input_barnsley_iterations").value)
        const config = new BarnsleyConfig(state.x_min, state.x_max, state.y_min, state.y_max, state.iterations, width, height, seed);
        const start_time = performance.now();
        surowePiksele = generate_barnsley(config);
        const end_time = performance.now();
        const time_taken = (end_time - start_time).toFixed(2);
        console.log(`[Rust] Wygenerowanie fraktala zajęło: ${time_taken} ms`);
    }

// Przekształcamy tablicę z Rusta na format obrazu dla przeglądarki
    const obraz = new ImageData(
        new Uint8ClampedArray(surowePiksele),
        width,
        height
    );

// Rysujemy na płótnie
    ctx.putImageData(obraz, 0, 0);
    draw_axis();
}


function draw_axis() {
    const axis_canvas = document.getElementById('axis');
    const ctx_axis = axis_canvas.getContext('2d');
    const width = axis_canvas.width;
    const height = axis_canvas.height;

// Czyszczenie poprzedniej klatki
    ctx_axis.clearRect(0, 0, width, height);

// Pozycje głównego krzyża
    let x_dist = -state.x_min / (state.x_max - state.x_min) * width;
    let y_dist = state.y_max / (state.y_max - state.y_min) * height;

// --- OBLICZANIE DYNAMICZNEJ SKALI ---
    const math_width = state.x_max - state.x_min;
// Chcemy docelowo około 10-12 znaczników na całą szerokość ekranu
    const target_step = math_width / 10;

// Logika do zaokrąglenia kroku do "ładnych" liczb (np. 0.1, 0.5, 1, 2, 50)
    const exponent = Math.floor(Math.log10(target_step));
    const fraction = target_step / Math.pow(10, exponent);

    let step;
    if (fraction < 1.5) step = Math.pow(10, exponent);
    else if (fraction < 3) step = 2 * Math.pow(10, exponent);
    else if (fraction < 7) step = 5 * Math.pow(10, exponent);
    else step = 10 * Math.pow(10, exponent);

// --- RYSOWANIE GŁÓWNYCH OSI ---
    ctx_axis.beginPath();
    ctx_axis.strokeStyle = "rgba(200, 200, 200, 0.5)";
    ctx_axis.lineWidth = 2;

    ctx_axis.moveTo(x_dist, 0);
    ctx_axis.lineTo(x_dist, height);
    ctx_axis.moveTo(0, y_dist);
    ctx_axis.lineTo(width, y_dist);

// Konfiguracja czcionki dla liczb
    ctx_axis.fillStyle = "rgba(100, 100, 100, 0.8)";
    ctx_axis.font = "14px Arial";

// --- ZNACZNIKI OSI X (Poziomej) ---
    ctx_axis.textAlign = "center";
    ctx_axis.textBaseline = "top";

    let start_x = Math.ceil(state.x_min / step) * step;
    for (let v = start_x; v <= state.x_max; v += step) {
// Pomiń zero, żeby nie rysować liczby na samym środku skrzyżowania
        if (Math.abs(v) < step * 0.1) continue;

        let x_pos = ((v - state.x_min) / (state.x_max - state.x_min)) * width;

// Pionowa kreska znacznika
        ctx_axis.moveTo(x_pos, y_dist - 6);
        ctx_axis.lineTo(x_pos, y_dist + 6);

// Etykieta (używamy toPrecision, by JavaScript sam użył notacji naukowej np. 1e-4)
        let label = parseFloat(v.toPrecision(9)).toString();
        ctx_axis.fillText(label, x_pos, y_dist + 12);
    }

// --- ZNACZNIKI OSI Y (Pionowej) ---
    ctx_axis.textAlign = "left";
    ctx_axis.textBaseline = "middle";

    let start_y = Math.ceil(state.y_min / step) * step;
    for (let v = start_y; v <= state.y_max; v += step) {
        if (Math.abs(v) < step * 0.1) continue;

        let y_pos = height - ((v - state.y_min) / (state.y_max - state.y_min)) * height;

// Pozioma kreska znacznika
        ctx_axis.moveTo(x_dist - 6, y_pos);
        ctx_axis.lineTo(x_dist + 6, y_pos);

        let label = parseFloat(v.toPrecision(9)).toString();
        ctx_axis.fillText(label, x_dist + 12, y_pos);
    }

// Nanieś wszystko na płótno
    ctx_axis.stroke();
}


function default_resolution(){
    const canvas = document.getElementById('fractal');

    canvas.width = 1920;
    canvas.height = 1080;
    draw()
}


export {draw,default_resolution}