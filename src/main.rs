use std::fs;
use std::env;
use colored::Colorize;

fn load_grid(filepath : &String) -> Result<[[u8;9];9], String> {
    let contents = fs::read_to_string(filepath)
                        .expect("Reading file ran into problems!");
    let mut g = [[0_u8;9];9];
    let n_lines = contents.lines().count();
    if n_lines != 9 {
        let err_msg = format!("Invalid sudoku format! Found {nr_lines} lines; should be 9.", 
                                                             nr_lines=n_lines);
        return Err(err_msg);
    }
    for (lnr, l) in contents.lines().enumerate() {
        if l.len() != 9 {
            let err_msg = format!("Invalid sudoku format! Line {linenr} has invalid length {len}.", 
                                                                linenr=lnr+1,               len=l.len());
            return Err(err_msg);
        }
        for (cnr,c) in l.chars().enumerate() {
            if c.is_digit(10) {
                g[lnr][cnr] = (c as u8)-('0' as u8);
            } else {
                let err_msg = format!("Invalid character found in sudoku! Line {linenr}, column {colnr}: {ch}.", 
                                                                                linenr=lnr+1,colnr=cnr+1, ch=c);
                return Err(err_msg);
            }
        }
    }

    return Ok(g);
}

fn show_original_sudoku(orig_grid: [[u8;9];9]) {
    let top_h = format!("{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}", h='\u{2550}', t='\u{2564}');
    let bot_h = format!("{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}", h='\u{2550}', t='\u{2567}');
    let interior_horizontal_rule_heavy = format!(
        "{ent}{hh}{xhl}{hh}{xhl}{hh}{xhh}{hh}{xhl}{hh}{xhl}{hh}{xhh}{hh}{xhl}{hh}{xhl}{hh}{rent}",
        ent='\u{255F}', hh='\u{2501}', xhl='\u{253F}', xhh='\u{254B}', rent='\u{2562}'
        );
    let interior_horizontal_rule_light = format!(
        "{ent}{hl}{xll}{hl}{xll}{hl}{xlh}{hl}{xll}{hl}{xll}{hl}{xlh}{hl}{xll}{hl}{xll}{hl}{rent}",
        ent='\u{255F}', hl='\u{2508}', xll='\u{253C}', xlh='\u{2542}', rent='\u{2562}'
        );
    println!("{corner1}{ht}{corner2}", corner1='\u{2554}', corner2='\u{2557}', ht=top_h );
    for r in 0..9 {
        print!("{v}", v='\u{2551}');
        for c in 0..9 {
            if c > 0 && c % 3 == 0 {
                print!("{sol_v_heavy}", sol_v_heavy = '\u{2503}');
            } else if c > 0 {
                print!("{dashed_v_light}", dashed_v_light='\u{250A}')
            }
            if orig_grid[r][c] == 0 {
                print!(" ")
            } else {
                print!("{}", orig_grid[r][c]);
            }
        }
        println!("{v}", v='\u{2551}');
        if r < 8  && r % 3 == 2{
            println!("{interior_horizontal_rule_heavy}");
        } else if r < 8 {
            println!("{interior_horizontal_rule_light}");
        }
    }
    println!("{corner1}{ht}{corner2}", corner1='\u{255A}', corner2='\u{255D}', ht=bot_h );
}

fn show_solved_sudoku(orig_grid: [[u8;9];9], solved_grid:[[u8;9];9]) {
    let top_h = format!("{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}", h='\u{2550}', t='\u{2564}');
    let bot_h = format!("{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}", h='\u{2550}', t='\u{2567}');
    let interior_horizontal_rule_heavy = format!(
        "{ent}{hh}{xhl}{hh}{xhl}{hh}{xhh}{hh}{xhl}{hh}{xhl}{hh}{xhh}{hh}{xhl}{hh}{xhl}{hh}{rent}",
        ent='\u{255F}', hh='\u{2501}', xhl='\u{253F}', xhh='\u{254B}', rent='\u{2562}'
        );
    let interior_horizontal_rule_light = format!(
        "{ent}{hl}{xll}{hl}{xll}{hl}{xlh}{hl}{xll}{hl}{xll}{hl}{xlh}{hl}{xll}{hl}{xll}{hl}{rent}",
        ent='\u{255F}', hl='\u{2508}', xll='\u{253C}', xlh='\u{2542}', rent='\u{2562}'
        );
    println!("{corner1}{ht}{corner2}", corner1='\u{2554}', corner2='\u{2557}', ht=top_h );
    for r in 0..9 {
        print!("{v}", v='\u{2551}');
        for c in 0..9 {
            if c > 0 && c % 3 == 0 {
                print!("{sol_v_heavy}", sol_v_heavy = '\u{2503}');
            } else if c > 0 {
                print!("{dashed_v_light}", dashed_v_light='\u{250A}')
            }
            if orig_grid[r][c] == 0 {
                print!("{}", solved_grid[r][c].to_string().green())
            } else {
                print!("{}", orig_grid[r][c]);
            }
        }
        println!("{v}", v='\u{2551}');
        if r < 8  && r % 3 == 2{
            println!("{interior_horizontal_rule_heavy}");
        } else if r < 8 {
            println!("{interior_horizontal_rule_light}");
        }
    }
    println!("{corner1}{ht}{corner2}", corner1='\u{255A}', corner2='\u{255D}', ht=bot_h );
}

fn show_err_sudoku(orig_grid: [[u8;9];9], err_grid: [[u8;9];9], r_err : usize, c_err: usize) {
    let top_h = format!("{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}", h='\u{2550}', t='\u{2564}');
    let bot_h = format!("{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}{t}{h}", h='\u{2550}', t='\u{2567}');
    let interior_horizontal_rule_heavy = format!(
        "{ent}{hh}{xhl}{hh}{xhl}{hh}{xhh}{hh}{xhl}{hh}{xhl}{hh}{xhh}{hh}{xhl}{hh}{xhl}{hh}{rent}",
        ent='\u{255F}', hh='\u{2501}', xhl='\u{253F}', xhh='\u{254B}', rent='\u{2562}'
        );
    let interior_horizontal_rule_light = format!(
        "{ent}{hl}{xll}{hl}{xll}{hl}{xlh}{hl}{xll}{hl}{xll}{hl}{xlh}{hl}{xll}{hl}{xll}{hl}{rent}",
        ent='\u{255F}', hl='\u{2508}', xll='\u{253C}', xlh='\u{2542}', rent='\u{2562}'
        );
    println!("{corner1}{ht}{corner2}", corner1='\u{2554}', corner2='\u{2557}', ht=top_h );
    for r in 0..9 {
        print!("{v}", v='\u{2551}');
        for c in 0..9 {
            if c > 0 && c % 3 == 0 {
                print!("{sol_v_heavy}", sol_v_heavy = '\u{2503}');
            } else if c > 0 {
                print!("{dashed_v_light}", dashed_v_light='\u{250A}')
            }
            if r == r_err && c == c_err {
                print!("{}", "X".red());
            } else if err_grid[r][c] == 0 {
                print!(" ");
            } else if orig_grid[r][c] == 0 {
                print!("{}", err_grid[r][c].to_string().green())
            } else {
                print!("{}", err_grid[r][c]);
            }
        }
        println!("{v}", v='\u{2551}');
        if r < 8  && r % 3 == 2{
            println!("{interior_horizontal_rule_heavy}");
        } else if r < 8 {
            println!("{interior_horizontal_rule_light}");
        }
    }
    println!("{corner1}{ht}{corner2}", corner1='\u{255A}', corner2='\u{255D}', ht=bot_h );
}

fn analyse_sudoku(grid : [[u8;9]; 9]) -> [[[bool;9]; 9]; 9] {
    //simple checking for the time being.
    let mut result = [[[false; 9]; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] == 0 {
                for d in 0..9 {
                    let mut found = false;
                    // check box
                    let box_x = c / 3;
                    let box_y = r / 3;
                    for r_ in (box_y * 3) .. (box_y * 3 + 3) {
                        for c_ in (box_x * 3) .. (box_x * 3 + 3) {
                            if grid[r_][c_] == d+1 {
                                found = true;
                            }
                        }
                    }
                    // check row
                    for c_ in 0..9 {
                        if grid[r][c_] == d+1 {
                            found = true;
                        }
                    }
                    // check column
                    for r_ in 0..9 {
                        if grid[r_][c] == d+1 {
                            found = true;
                        }
                    }

                    // mark possibility if no blocking number found.
                    if !found {
                        result[r][c][d as usize] = true;
                    }
                }
            }
        }
    }
    return result;
}

fn solve_sudoku(orig_grid: [[u8;9];9]) -> Result<[[u8;9];9], (String, [[u8;9];9], (usize, usize))> {
    let mut grid = orig_grid.clone();

    let mut filled_in = [[true;9];9];
    for c in 0..9{
        for r in 0..9 {
            if grid[r][c] == 0 {
                filled_in[r][c] = false;
            }
        }
    }
    let mut possibilities = analyse_sudoku(grid);
    
    // fill in any items that can be found straight away
    for r in 0..9 {
        for c in 0..9 {
            if !filled_in[r][c]{
                let mut count = 0;
                let mut index = 0;
                for d in 0..9 {
                    if possibilities[r][c][d] {
                        count = count + 1;
                        index = d;
                    }
                }
                if count == 0 {
                    let err_msg = format!("Error: Row {}, column {} has no solution.", r+1, c+1);
                    return Err((err_msg, grid, (r,c)));
                } else if count == 1 {
                    grid[r][c] = (index as u8) + 1;
                    filled_in[r][c] = true;
//                    println!("Placed {} at row {} col {}", grid[r][c], r+1, c+1);
                    // update possibilities for squares affected by this placement.
                    for x in 0..9 {
                        possibilities[x][c][index] = false;
                        possibilities[r][x][index] = false;
                    }
                    for d_ in 0..9 {
                        possibilities[r][c][d_] = false;
                    }

                    let box_x = c / 3;
                    let box_y = r / 3;
                    for r_ in (box_y * 3) .. (box_y * 3 + 3) {
                        for c_ in (box_x * 3) .. (box_x * 3 + 3) {
                            possibilities[r_][c_][index] = false;
                        }
                    }
                }
            }
        }
    }

    // apply backtracking algorithm. Cleverer techniques can be inserted before this stage in order
    // to make solution cleaner & more efficient.
    
    let mut r_guess = 0;
    let mut c_guess = 0;
    while r_guess < 9 && filled_in[r_guess][c_guess] {
        c_guess = c_guess + 1;
        r_guess = r_guess + c_guess / 9;
        c_guess = c_guess % 9;
    }
    if r_guess == 9 {
        return Ok(grid);
    }
    // try go through all possible choices of digit for this place. If any one of them returns a
    // solution, simply pass this up. Else, return an error.
    
    for d in 0..9 {
        if possibilities[r_guess][c_guess][d] {
            grid[r_guess][c_guess] = (d as u8)+1;
            let outcome = solve_sudoku(grid);
            match outcome {
                Ok(g) => return Ok(g),
                Err(_) => {}
            }
        }
    }
    grid[r_guess][c_guess] = 0;
    let err_msg = format!("No options for row {}, column {} lead to solution!", r_guess+1, c_guess+1);
    return Err((err_msg, grid, (r_guess, c_guess)));
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let filepath = &args[1];
    
    let grid_res = load_grid(filepath);
    match grid_res {
        Ok(g) => {
            let grid = g;
            println!("Solving Sudoku: \n");
            show_original_sudoku(grid);
            let solution = solve_sudoku(grid);
            match solution {
                Ok(solved_g) => { 
                    println!("Found solution! \n");
                    show_solved_sudoku(grid, solved_g);
                },
                Err((msg, err_g, (r_err, c_err))) => {
                    println!("{msg}"); 
                    show_err_sudoku(grid, err_g, r_err, c_err);
                }
            }

        },
        Err(msg) => {println!("{msg}"); return}
    }

}
