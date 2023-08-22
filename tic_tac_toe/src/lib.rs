// todo: tic_tac_toe
/*
Instructions
You must create some functions for a tic-tac-toe checker.

Create a function named tic_tac_toe, which receives a tic-tac-toe table. 
It should return the appropriate string: "player O won", "player X won" or "tie".

Also create the following functions, which each accept a player and a table. 
These functions should return true if the player has completed one of the diagonals, rows or columns:

Expected functions
pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
} */
pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let players = ["X", "O"];

    for &player in &players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            return format!("player {} won", player);
        }
    }

    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut diag1 = true;
    let mut diag2 = true;

    for i in 0..table.len() {
        if table[i][i] != player {
            diag1 = false;
        }
        if table[i][table.len() - i - 1] != player {
            diag2 = false;
        }
    }

    diag1 || diag2
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        let row_complete = row.iter().all(|&cell| cell == player);
        if row_complete {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for col in 0..table[0].len() {
        let col_complete = table.iter().all(|row| row[col] == player);
        if col_complete {
            return true;
        }
    }
    false
}
