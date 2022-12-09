use std::fs;

// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut result = 0;
    for line in input.lines(){
        let parts: Vec<&str> = line.split(" ").collect();
        let enemy = symbol2Num(parts[0]);
        let mine = match (enemy, parts[1]) {

        };
        let win_score = match (enemy, mine) {
            (1,2) => 6,
            (2,3) => 6,
            (3,1) => 6,
            (enemy, mine) if (enemy == mine) => 3,
            _ => 0
        };

        result += win_score + mine;
    }

    println!("{}", result);
}

fn symbol2Num(symbol: &str) -> i32{
    let n = match symbol {
        "X" => 0,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };

    return n;
}


/*
 *  Task 1

// X for Rock, Y for Paper, and Z for Scissors

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut result = 0;
    for line in input.lines(){
        let parts: Vec<&str> = line.split(" ").collect();
        let enemy = symbol2Num(parts[0]);
        let mine = symbol2Num(parts[1]);
        let win_score = match (enemy, mine) {
            (1,2) => 6,
            (2,3) => 6,
            (3,1) => 6,
            (enemy, mine) if (enemy == mine) => 3,
            _ => 0
        };

        result += win_score + mine;
    }

    println!("{}", result);
}

fn symbol2Num(symbol: &str) -> i32{
    let n = match symbol {
        "X" | "A" => 1,
        "Y" | "B" => 2,
        "Z" | "C" => 3,
        _ => 0
    };

    return n;
}*/
