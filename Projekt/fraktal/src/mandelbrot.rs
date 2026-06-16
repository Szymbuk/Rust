

use std::ops::Add;
use wasm_bindgen::prelude::*;
use num_complex::{Complex};

// Makro #[wasm_bindgen] sprawia, że funkcja będzie widoczna w JavaScript
#[wasm_bindgen]
pub struct MandelbrotConfig {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    iterations: usize,
    width: u32,
    height: u32
}
#[wasm_bindgen]
impl MandelbrotConfig {

    #[wasm_bindgen(constructor)]
    pub fn new(    x_min: f64, x_max: f64, y_min: f64, y_max: f64, iterations: usize, width: u32, height: u32) -> MandelbrotConfig {
        MandelbrotConfig {x_min, x_max, y_min, y_max, iterations, width, height}
    }
}



#[wasm_bindgen]
pub fn generate_mandelbrot(config: MandelbrotConfig) -> Vec<u8>{
    let size = (config.width * config.height * 4) as usize; // RGB and transparency
    let mut piksels = vec![0; size];
    for i in (0..size).step_by(4)
    {
        let number = convert_to_complex_number(i, config.width, config.height, config.x_min, config.x_max, config.y_min, config.y_max);
        let converges = check_convergence(number,config.iterations);

        if !converges{
            piksels[i] = 255;
            piksels[i + 1] = 255;
            piksels[i + 2] = 255;
            piksels[i + 3] = 255;
        }
        else{
            piksels[i] = 0;
            piksels[i + 1] = 0;
            piksels[i + 2] = 0;
            piksels[i + 3] = 255;
        }
    }
    piksels
}

fn convert_to_complex_number(x: usize, width: u32, height: u32, x_min:f64, x_max:f64, y_min:f64, y_max:f64) -> Complex<f64>{
    let x = x/4;
    let horizontal_step = (x_max-x_min)/(width as f64 - 1.0);
    let vertical_step = (y_max-y_min)/(height as f64 - 1.0);

    let horizontal_pixel = x%(width as usize);
    let vertical_pixel = x/(width as usize);

    let x_chord = x_min+horizontal_pixel as f64 * horizontal_step;
    let y_chord = y_max - (vertical_pixel as f64 * vertical_step);
    Complex{re:x_chord,im:y_chord}
}

fn check_convergence(number: Complex<f64>,iterations: usize) -> bool{
    let p = number; //creating p so it matches the math formula
    let mut z = Complex{re:0.0,im:0.0};
    const UPPER_BOUND:f64 =2_i32.pow(2) as f64;

    let mut i =0;
    while  i < iterations && z.norm_sqr() < UPPER_BOUND {
        z = z.powi(2).add(p);
        i+=1;
    }
    z.norm_sqr() < UPPER_BOUND
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_convergence_false_test(){
        let number = Complex{re:3.0,im:2.0};
        let iterations = 100;
        assert_eq!(false,check_convergence(number,iterations))
    }

    #[test]
    fn check_convergence_true_test(){
        let number = Complex{re:-0.25,im:-0.25};
        let iterations = 100;
        assert_eq!(true,check_convergence(number,iterations))
    }

    #[test]
    fn convert_to_complex_number_test(){

        let x_min = -2.0;
        let x_max = 1.0;
        let y_min = -1.0;
        let y_max = 1.0;
        let width = 11;
        let height = 11;
        let pixel = 60*4;
        let number = convert_to_complex_number(pixel, width, height, x_min, x_max, y_min, y_max);
        assert_eq!(Complex{re:-0.5,im:0.0},number);


        let pixel = 55*4;
        let number = convert_to_complex_number(pixel, width, height, x_min, x_max, y_min, y_max);
        assert_eq!(Complex{re:-2.0,im:0.0},number);



        let pixel = 65*4;
        let number = convert_to_complex_number(pixel, width, height, x_min, x_max, y_min, y_max);
        assert_eq!(Complex{re:1.0,im:0.0},number);


        let pixel = 0*4;
        let number = convert_to_complex_number(pixel, width, height, x_min, x_max, y_min, y_max);
        assert_eq!(Complex{re:-2.0,im:-1.0},number);

        let pixel = 120*4;
        let number = convert_to_complex_number(pixel, width, height, x_min, x_max, y_min, y_max);
        assert_eq!(Complex{re:1.0,im:1.0},number);
    }
}