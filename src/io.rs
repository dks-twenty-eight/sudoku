use std::fs;
pub fn load_grid(filepath : &String) -> Result<[[u8;9];9], String> {
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


