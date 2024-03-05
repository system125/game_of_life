
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;


use crate::consts::{CELL_SIZE, STROKE_COLOR};
/**
 * Module to help render game of life board
 **/

 use crate::universe::Universe;
 
 pub fn render_grid(context:&CanvasRenderingContext2d,uni:&Universe){
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str(STROKE_COLOR));

    for i in 0..=uni.width() {
        let start_x = (i * (CELL_SIZE + 1) + 1) as f64;
        let end_y =  (uni.height()*(CELL_SIZE + 1) + 1) as f64;
        
        context.move_to(start_x.into(), 0.0);
        context.line_to(start_x,end_y)
    }

    for j in 0..=uni.height() {
        let start_y = (j * (CELL_SIZE + 1) + 1) as f64;
        let end_x = (uni.width()*(CELL_SIZE + 1)+1 ) as f64;

        context.move_to(0.0, start_y.into());
        context.line_to(end_x, start_y);
    }
 }