import * as wasm from "game-of-life-rust/game_of_life_rust"


// let universe = wasm.UniverseRunner.init_default();

// universe.render();

// let fact = wasm.FactorialMemo.new(3);
// console.log(`Factorial of ${fact.input()} is ${fact.output()}`)

let unit = wasm.Universe_Runner.init();
unit.render();