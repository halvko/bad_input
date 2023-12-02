# bad_input

A library for input parsing with panicy error handeling, and an unreasonable amout of string
cloning.

# Why

I write [Advent of Code](https://adventofcode.com) in Rust, and I realised that quite a bit of my
code was input parsing and validation. This crate is my attempt of abstracting a lot of that
boilerplate.

# Example

The second day, first exercise of Advent of Code 2023

```rust
use std::io::stdin;

use bad_input::BadInput;

fn main() {
    let input = bad_input::BadInput::new(stdin());
    let res = run(input);
    println!("{res}");
}

fn run(mut input: BadInput<impl std::io::Read>) -> u64 {
    let mut sum = 0;
    for game in input.lines() {
        let [_, draws] = game.split_n(": ");
        let [mut min_red, mut min_blue, mut min_green] = [0, 0, 0];
        for draw in draws.split("; ") {
            for count_colour in draw.split(", ") {
                let [count, colour] = count_colour.split_n(" ");
                let count = count.parse::<u64>();
                match colour.as_str() {
                    "red" => min_red = min_red.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    "green" => min_green = min_green.max(count),
                    _ => (),
                }
            }
        }
        sum += min_red * min_blue * min_green;
    }
    sum
}

#[test]
fn small_input() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(run(BadInput::new(input.as_bytes())), 2286);
}
```

# License

See the LICENSE file, but all of this is released under the MIT license
