


const default_params = {
    mandelbrot: { x_min: -2.0, x_max: 1.0, y_min: -1.0, y_max: 1.0 },
    julia: { x_min: -1.5, x_max: 1.5, y_min: -1.0, y_max: 1.0 },
    barnsley: { x_min: -3, x_max: 3, y_min: -0.5, y_max: 10.5 }
}

let state =
{
    x_min: -2.0,
    x_max: 1.0,
    y_min: -1.0,
    y_max: 1.0,
    iterations: 200,
    current_fractal: "mandelbrot",
    frame_waiting: false,
    scroll_timer: null
}



export {default_params,state}