import * as wasm from "rusty_poker_web";

wasm.enable_debug();
console.log(wasm);

console.time('t1');
console.log(wasm.chance_to_win_preflop(BigInt(0b11000000000), 5));
console.timeEnd('t1');


console.time('t2');
console.log(wasm.chance_to_win(BigInt(0b11010), BigInt(0b110000000)));
console.timeEnd('t2');


const game = wasm.create_game();
console.log(game);

console.log(wasm.iterate_game(game));
console.log('getting state');
console.log(wasm.get_game_state(game));


