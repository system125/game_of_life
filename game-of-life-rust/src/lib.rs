
mod universe;
mod render;
mod utils;
mod consts;

use consts::CELL_SIZE;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document};


use crate::{render::render_grid, universe::Universe};
use std::{cell, f64, panic};



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


/**
 * Initializes canvas with width 5 and returns context for this canvas
 */
pub fn getCanvasContext (width:u32,height:u32) -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let canvas = document.get_element_by_id("game-of-life-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();


    let canvas_height =((CELL_SIZE + 1) * height + 1) as u32;
    let canvas_Width = ((CELL_SIZE+1)*width + 1) as u32;
    canvas.set_height(canvas_height);
    canvas.set_width(canvas_Width);

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
        let context = getCanvasContext(universe.width(),universe.height());

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
        // context
        //     .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        //     .unwrap();
        render_grid(context, &self.universe);

        context.stroke();
    
    }
}
