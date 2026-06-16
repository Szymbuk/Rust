use std::ops::Add;
use num_complex::Complex;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct JuliaConfig {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    iterations: usize,
    width: u32,
    height: u32,
    c_re: f64,
    c_im:f64
}
#[wasm_bindgen]
impl JuliaConfig {

    #[wasm_bindgen(constructor)]
    pub fn new(    x_min: f64, x_max: f64, y_min: f64, y_max: f64, iterations: usize, width: u32, height: u32,c_re:f64,c_im: f64) -> JuliaConfig {
        JuliaConfig {x_min, x_max, y_min, y_max, iterations, width, height, c_re, c_im }
    }
}



#[wasm_bindgen]
pub fn generate_julia(config: JuliaConfig) -> Vec<u8>{
    let size = (config.width * config.height * 4) as usize; // RGB and transparency
    let mut pixels = vec![0; size];
    let c = Complex{ re: config.c_re, im: config.c_im };
    for i in (0..size).step_by(4)
    {
        let number = crate::mandelbrot::convert_to_complex_number(i, config.width, config.height, config.x_min, config.x_max, config.y_min, config.y_max);
        let converges = check_convergence_julia(number,c, config.iterations);

        if !converges{
            pixels[i] = 255;
            pixels[i + 1] = 255;
            pixels[i + 2] = 255;
            pixels[i + 3] = 255;
        }
        else{
            pixels[i] = 0;
            pixels[i + 1] = 0;
            pixels[i + 2] = 0;
            pixels[i + 3] = 255;
        }
    }
    pixels
}


fn check_convergence_julia(pixel: Complex<f64>,c: Complex<f64>, iterations: usize) -> bool{
    let mut z = pixel;
    const UPPER_BOUND:f64 =2_i32.pow(2) as f64;

    let mut i =0;
    while  i < iterations && z.norm_sqr() < UPPER_BOUND {
        z = z.powi(2).add(c);
        i+=1;
    }
    z.norm_sqr() < UPPER_BOUND
}