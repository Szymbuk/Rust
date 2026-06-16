import {default_params, state} from './state.js'
import {default_resolution, draw} from "./render.js";
function handle_wheel(event){
    const canvas = document.getElementById('fractal');
    clearTimeout(state.scroll_timer);
    canvas.width = 480;
    canvas.height = 270;

    event.preventDefault();

    let x_middle = (state.x_max + state.x_min) / 2;
    let y_middle = (state.y_max + state.y_min) / 2;
    let new_width;
    let new_height;
    if (event.deltaY <0) {
        new_width = (state.x_max - state.x_min) * 0.9
        new_height = (state.y_max - state.y_min) * 0.9
    }
    else{
        new_width = (state.x_max - state.x_min) * 1.1
        new_height = (state.y_max - state.y_min) * 1.1
    }
    state.x_min = x_middle - new_width / 2
    state.x_max = x_middle + new_width / 2

    state.y_min = y_middle - new_height / 2
    state.y_max = y_middle + new_height / 2
    if (!state.frame_waiting){
        state.frame_waiting = true
        requestAnimationFrame( () => {draw();state.frame_waiting = false} )
    }

    state.scroll_timer = setTimeout(default_resolution, 600)
}


function handle_keys(event){
    const canvas = document.getElementById('fractal');
    clearTimeout(state.scroll_timer);
    canvas.width = 480;
    canvas.height = 270;

    if (event.key === 's' || event.key === 'ArrowDown') {
        let move = (state.y_max - state.y_min) * 0.1
        state.y_max -= move
        state.y_min -= move
    }

    if (event.key === 'w' || event.key === 'ArrowUp') {
        let move = (state.y_max-state.y_min)*0.1
        state.y_max += move
        state.y_min += move
    }

    if (event.key === 'd' || event.key === 'ArrowRight') {
        let move = (state.x_max-state.x_min)*0.1
        state.x_max += move
        state.x_min += move
    }

    if (event.key === 'a' || event.key === 'ArrowLeft') {
        let move = (state.x_max-state.x_min)*0.1
        state.x_max -= move
        state.x_min -= move
    }

    if (!state.frame_waiting){
        state.frame_waiting = true
        requestAnimationFrame( () => {draw();state.frame_waiting = false} )
    }

    state.scroll_timer = setTimeout(default_resolution, 600)
}


function handle_mousemove(event) {
    const canvas = document.getElementById('fractal');
    const coords_box = document.getElementById('cords');

// Pobranie pozycji piksela, nad którym jest myszka
    const rect = canvas.getBoundingClientRect();
    const mouse_x = event.clientX - rect.left;
    const mouse_y = event.clientY - rect.top;

// Pobranie aktualnych wymiarów z elementu HTML (nie z wewnętrznej rozdzielczości!)
    const html_width = rect.width;
    const html_height = rect.height;

// Przeliczenie pikseli na układ matematyczny
    const re = state.x_min + (mouse_x / html_width) * (state.x_max - state.x_min);
    const im = state.y_max - (mouse_y / html_height) * (state.y_max - state.y_min);

// Formatowanie tekstu (toExponential ładnie formatuje ekstremalnie małe liczby przy dużym zoomie)
    const re_text = (Math.abs(re) < 1e-6 && re !== 0) ? re.toExponential(6) : re.toFixed(7);
    const im_text = (Math.abs(im) < 1e-6 && im !== 0) ? im.toExponential(6) : im.toFixed(7);

// Aktualizacja HTML
    coords_box.innerHTML = `Re: ${re_text} <br>Im: ${im_text} `;
}

function fractal_selection(event){
    state.current_fractal = event.target.value;

    const julia_params = document.getElementById('julia_params');
    const barnsley_params = document.getElementById('barnsley_params');

    // Pokaż/ukryj parametry w zależności od wyboru
    if (state.current_fractal === "julia") {
        julia_params.style.display = "block";
        barnsley_params.style.display = "none";

    } else if (state.current_fractal === "barnsley") {
        julia_params.style.display = "none";
        barnsley_params.style.display = "block";
    }

    // Załaduj domyślne współrzędne dla wybranego fraktala
    const view = default_params[state.current_fractal];
    state.x_min = view.x_min;
    state.x_max = view.x_max;
    state.y_min = view.y_min;
    state.y_max = view.y_max;

    state.frame_waiting = false;
    clearTimeout(state.scroll_timer);
    default_resolution();

}

export {handle_wheel, handle_keys, handle_mousemove,fractal_selection}