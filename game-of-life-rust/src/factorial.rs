use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FactorialMemo{
    input:usize,output:usize
}

fn calculate_fibonnaci (x:usize) -> usize {
    match x {
        0 => 1,
        1 => 1,
       _ => x*calculate_fibonnaci(x - 1)
    }
}

#[wasm_bindgen]
impl FactorialMemo {
    pub fn new (x:usize) -> FactorialMemo {
        let input = x;
        let fibb = calculate_fibonnaci(x);

        return FactorialMemo{
            input: input,
            output: fibb
        }
    }

    pub fn output(&self) -> usize { self.output}
    pub fn input (&self) -> usize { self.input}
}