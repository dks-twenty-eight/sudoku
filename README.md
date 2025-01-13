# Sudoku

A little program written for fun, to get more experience with the Rust programming language. 

I suggested to a colleague he could challenge his new students to write a Sudoku solver, so I would be pretty hypocritical if I didn't do it myself...

To run this program, you need to have the Rust compiler (and ideally `rustup`) installed. Once `rustup`/`cargo` is installed, simply type `cargo build` to build the program. 

Write a sudoku into a file in `sudokus/` as a 9-by-9 grid of digits where `0`s represent unknown squares. 

Finally run `target/debug/sudoku sudokus/{filename}` to solve the given Sudoku.
