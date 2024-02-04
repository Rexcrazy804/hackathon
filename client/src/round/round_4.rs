// TIC TAC TOE
/*
* Parse through the inputs containing several tic tac toe games
* return their sum
*/

pub(super) fn compute(input_string: &str) -> i32 {
    let (mut win_x, mut win_o, mut draw) = (1, 1, 1);

    for line in input_string.lines() {
        let game: Vec<char> = line.chars().filter(|&x| "XO_".contains(x)).collect();

        if let Some(stroke) = table_result(game) {
            match stroke {
                'X' => win_x += 1,
                'O' => win_o += 2,
                _ => unreachable!(),
            }
        } else { draw += 3}
    }
    win_x * win_o * draw
}

fn table_result(table: Vec<char>) -> Option<char> {
    // rowks
    for row in table.chunks(3) {
        let (x, y, z) = (row[0], row[1], row[2]);
        if x != '_' && x == y && x == z {
            return Some(x);
        }
    }

    //columns
    for index in 0..3 {
        if table[index] != '_' && table[index] == table[index+3] && table[index] == table[index+6] {
            return Some(table[index])
        }
    }

    // Diagonal left
    if table[0] != '_' && table[0] == table[4] && table[0] == table[8] {
        return Some(table[0])
    }
    // Diagonal Right
    if table[2] != '_' && table[2] == table[4] && table[2] == table[6] {
        return Some(table[2]);
    }

    None
}
