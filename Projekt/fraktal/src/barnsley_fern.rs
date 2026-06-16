use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct BarnsleyConfig {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    iterations: usize,
    width: u32,
    height: u32
}
#[wasm_bindgen]
impl BarnsleyConfig {

    #[wasm_bindgen(constructor)]
    pub fn new(    x_min: f64, x_max: f64, y_min: f64, y_max: f64, iterations: usize, width: u32, height: u32, seed:f64) -> BarnsleyConfig {
        BarnsleyConfig {x_min, x_max, y_min, y_max, iterations, width, height}
    }
}



#[wasm_bindgen]
pub fn generate_barnsley(config: BarnsleyConfig) -> Vec<u8>{
    vec![3]
}



fn x_to_pixel(x: f64,width:u32) -> u32{
    ((x + 2.1820)/4.8378 * width as f64) as u32
}


fn y_to_pixel(y: f64,height:u32) -> u32{
    ((9.9983 - y)/9.9983 * height as f64) as u32
}