
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document};
pub mod universe;
pub mod render;
pub mod utils;

use crate::universe::Universe;
use std::{f64, panic};



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


pub fn getCanvasContext () -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game-of-life-canvas").unwrap();
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
    context: CanvasRenderingContext2d
}

#[wasm_bindgen]
impl Universe_Runner{
    pub fn init() -> Universe_Runner {
        set_panic_hook();

        console_log!("Creating Universe!");
        let universe = Universe::New_alive_at_2_or_7_divisible();
        console_log!("Creating context!");
        let context = getCanvasContext();

        Universe_Runner {
            universe: universe,
            context:context
        }
    }

    pub fn render (&self) {
        let context = &self.context;
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str("#ffffff"));

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    
    }
}
