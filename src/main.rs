mod displays;
mod io;
mod sol;

use std::env;


fn main() {
    let args : Vec<String> = env::args().collect();
    let filepath = &args[1];
    
    let grid_res = io::load_grid(filepath);
    match grid_res {
        Ok(g) => {
            let grid = g;
            println!("Solving Sudoku: \n");
            displays::show_original_sudoku(grid);
            let solution = sol::solve_sudoku(grid);
            match solution {
                Ok(solved_g) => { 
                    println!("Found solution! \n");
                    displays::show_solved_sudoku(grid, solved_g);
                },
                Err((msg, err_g, (r_err, c_err))) => {
                    println!("{msg}"); 
                    displays::show_err_sudoku(grid, err_g, r_err, c_err);
                }
            }

        },
        Err(msg) => {println!("{msg}"); return}
    }
}
