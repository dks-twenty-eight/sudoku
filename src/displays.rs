use colored::Colorize;
pub fn show_original_sudoku(orig_grid: [[u8;9];9]) {
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

pub fn show_solved_sudoku(orig_grid: [[u8;9];9], solved_grid:[[u8;9];9]) {
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

pub fn show_err_sudoku(orig_grid: [[u8;9];9], err_grid: [[u8;9];9], r_err : usize, c_err: usize) {
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
