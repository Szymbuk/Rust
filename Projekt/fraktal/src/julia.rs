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
    vec![1]

}