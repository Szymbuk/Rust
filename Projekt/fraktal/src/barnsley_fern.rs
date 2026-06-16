use wasm_bindgen::prelude::wasm_bindgen;
use rand;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[wasm_bindgen]
pub struct BarnsleyConfig {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    iterations: usize,
    width: u32,
    height: u32,
    seed: u64
}
#[wasm_bindgen]
impl BarnsleyConfig {

    #[wasm_bindgen(constructor)]
    pub fn new(    x_min: f64, x_max: f64, y_min: f64, y_max: f64, iterations: usize, width: u32, height: u32, seed:u64) -> BarnsleyConfig {
        BarnsleyConfig {x_min, x_max, y_min, y_max, iterations, width, height,seed}
    }
}



#[wasm_bindgen]
pub fn generate_barnsley(config: BarnsleyConfig) -> Vec<u8>{
    let size = (config.width * config.height * 4) as usize;
    let mut pixels = vec![255; size];
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);



    let mut x = 0.0;
    let mut y = 0.0;

    let mut xn = 0.0;
    let mut yn = 0.0;

    for i in 0..config.iterations {
        let r: f64 = rng.random();
        if r < 0.01 {
            xn = 0.0;
            yn = 0.16 * y;
        }

        else if r < 0.86 {

            xn = 0.85 * x + 0.04 * y;
            yn = -0.04 * x + 0.85 * y + 1.6;
        }
        else if r < 0.93
        {
            xn = 0.2 * x - 0.26 * y;
            yn = 0.23 * x + 0.22 * y + 1.6;
        }
        else {
            xn = -0.15 * x + 0.28 * y;
            yn = 0.26 * x + 0.24 * y + 0.44;
        }

        if config.x_min<= xn && xn<= config.x_max && config.y_min<= yn && yn<= config.y_max{
            let pixel = convert_to_pixel(xn,yn,&config);
            pixels[pixel] = 50;
            pixels[pixel+1] = 205;
            pixels[pixel+2] = 50;
            pixels[pixel+3] = 255;



        }
        x = xn;
        y = yn;
    }
    pixels
}



fn convert_to_pixel(xn: f64, yn: f64, config: &BarnsleyConfig) -> usize {
    // Odległość punktu od lewej krawędzi (x_min) podzielona przez rozpiętość
    let x_fraction = (xn - config.x_min) / (config.x_max - config.x_min);
    let x_pos = (x_fraction * config.width as f64) as usize;

    // Odległość punktu od górnej krawędzi (y_max) podzielona przez rozpiętość
    let y_fraction = (config.y_max - yn) / (config.y_max - config.y_min);
    let y_pos = (y_fraction * config.height as f64) as usize;

    // Przeliczenie na 1-wymiarową tablicę RGBA
    (y_pos * config.width as usize + x_pos) * 4
}