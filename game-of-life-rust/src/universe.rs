use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[repr(u8)]
#[derive (Clone, Copy, Debug, PartialEq, Eq)]
pub enum  Cell {Dead = 0, Alive = 1}

#[wasm_bindgen]
pub struct Universe {
    width: u32, height: u32, cells: Vec<Cell>
}

/** *
 * Create if divisible by 2 or 7
*/
fn create_if_divisible_by_2_or_7 (x:u32) -> Cell {
    if x % 2 == 0 || x % 7 == 0 {
        Cell::Alive   
    }else {
        Cell::Dead
    }
}

impl Universe{
    /** *
     * Map 
    */

    pub fn get_index(&self,row:u32,col:u32) -> usize {
        (row*self.width + col) as usize         
    }

    pub fn New_alive_at_2_or_7_divisible () -> Universe {
        Universe::New(create_if_divisible_by_2_or_7)
    }

    pub fn New (creation_rule: fn(u32) -> Cell) -> Universe{
        let width = 64;
        let height = 64;

        let Cellz = (0..height*width).into_iter()
                .map(creation_rule).collect();



        return Universe{
            width: width,
            height: height,
            cells: Cellz
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width (&self) -> u32 {self.width}
    pub fn height (&self) -> u32 {self.height}
}