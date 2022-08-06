# Rusty Poker
Learning Rust while playing Poker.


## Crates
- [**Core (`core`)**](./core) A library for running a poker game. Includes evaluators and basic computer players.

- [**CLI Basic (`cli_basic`)**](./cli_basic) A binary for games using basic text I/O on the terminal.

- [**CLI UI (`cli_ui`)**](./cli_ui) A binary for games on the terminal.



## Evaluators
- [**TwoPlusTwo**](https://www.codingthewheel.com/archives/poker-hand-evaluator-roundup/#2p2) This is widely regarded as the fastest algorithm. An initial test showed post-flop game calculations (~4m outcomes) takes 126ms. Unfortunately we need to send around a [125MB lookup table](https://raw.githubusercontent.com/tommy-a/zetebot/master/src/data/HandRanks.dat).

- [**PH Evaluator**](https://github.com/HenryRLee/PokerHandEvaluator) A modified and improved version of the original work on a 5 card evaluator by [Cactus Kev](http://suffe.cool/poker/evaluator.html). This requires relatively small lookup tables and is perfect for a small binary.

