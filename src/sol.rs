pub fn analyse_sudoku(grid : [[u8;9]; 9]) -> [[[bool;9]; 9]; 9] {
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

pub fn solve_sudoku(orig_grid: [[u8;9];9]) -> Result<[[u8;9];9], (String, [[u8;9];9], (usize, usize))> {
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


