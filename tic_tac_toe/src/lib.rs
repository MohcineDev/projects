pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut res = String::from("tie");
    
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table){
        res = "player X won".to_string();

    }else if diagonals('O', table) || horizontal('O', table) || vertical('O', table){
        
        res = "player O won".to_string();
    }
    res
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let mut res = false;

    if table[0][0] == player && table[1][1] == player && player == table[2][2] {
        res = true;
    }
    if table[0][2] == player &&  table[1][1] == player && player == table[2][0] {
        res = true;
    }

    res
}
/*
0 0  0
0 0  0
0 0  0

*/
pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let mut res = false;

    for i in 0..table.len() {
        let mut counter = 0;
        for j in 0..table[i].len() {
            if table[i][j] == player {
                counter += 1;
            }
        }
        if counter == 3 {
            res = true;
            break;
        }
    }

    res
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut res = false;
    // let mut j = 0;

    for  j in 0..table[0].len() {
        let mut counter = 0;
        for i in 0..table.len() {
            if table[i][j] == player {
                counter += 1;
            }
        }
        if counter == 3 {
            res = true;
            break;
        }
        // println!("");

        // if j == 3 {
        //     i += 1;
        // }
        // if table[i][j] == player {
        //     counter += 1;
        // }
        // j += 1;

        // if counter == 3 {
        //     res = true;
        //     break;
        // }
        // println!("-- {}", i);
    }
    res
}
