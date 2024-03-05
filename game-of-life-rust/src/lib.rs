
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document};
pub mod universe;

use crate::universe::Universe;
use std::f64;



// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen] 
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen]
    fn alert(st: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub fn getCanvasContext () -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

     canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
    
}

#[wasm_bindgen]
pub struct Universe_Runner{
    universe: Universe,
}

#[wasm_bindgen]
impl Universe_Runner{
    pub fn init() -> Universe_Runner {
        let universe = Universe::New_alive_at_2_or_7_divisible();

        Universe_Runner {
            universe: universe
        }
    }
}
