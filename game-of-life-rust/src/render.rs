
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;


use crate::consts::{ALIVE_COLOR, CELL_SIZE, DEAD_COLOR, STROKE_COLOR};
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
    context.stroke();
 }

 pub fn render_cells(context:&CanvasRenderingContext2d,uni:&Universe){
    context.begin_path();

    for row in 0..uni.height(){
        for col in 0..uni.width(){
            
            let cell_style = match uni.get_cell_at(row, col) {
                crate::universe::Cell::Dead =>DEAD_COLOR,
                crate::universe::Cell::Alive => ALIVE_COLOR,
            };

          context.set_fill_style(&JsValue::from_str(cell_style));
        
          let rect_width = (col * (CELL_SIZE + 1) + 1) as f64;
          let rect_height = (row * (CELL_SIZE + 1) + 1) as f64;
          let cell_size = CELL_SIZE as f64;
         context.fill_rect(rect_width, rect_height, cell_size, cell_size);
        }
    }
    context.stroke();
 }