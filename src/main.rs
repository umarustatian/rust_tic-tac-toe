use std::io;

fn main(){
    // Board cells
    let mut cells: [&str; 9] = [" "; 9];
    // Player turn
    let mut turn_x: bool = true;

    // Win Conditions
    let wins = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],

        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],

        [0, 4, 8],
        [2, 4, 6],
    ];

    // Game loop
    loop {
        println!("Choose a cell from 1-9");

        //Player input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading line");
        let mut input_par: usize = input.trim().parse().expect("Error while parsing input");
        input_par -= 1;
        
        if cells[input_par] == " "{
            if turn_x == true{
                println!("X turn");
                cells[input_par] = "X";
                board(&mut cells);
                turn_x = false;
            }
            else if turn_x == false{
                println!("O turn");
                cells[input_par] = "O";
                board(&mut cells);
                turn_x = true;
            }
        }
        else{
            println!("Enter a value empty cell between 1 - 9");
        }
        // Check winner
        for [a, b, c] in wins {
            if cells[a] != " " && cells[a] == cells[b] && cells[b] == cells[c] {
                println!("Winner: {}", cells[a]);
                break;
            }
        }
        // Game loop break for testing
        if input_par == 10{
            break
        }
    }
}

// Print board function
fn board(cells: &mut [&str; 9]){
    println!(" {} | {} | {}", cells[0], cells[1], cells[2]);
    println!(" - + - + - ");
    println!(" {} | {} | {} ", cells[3], cells[4], cells[5]);
    println!(" - + - + - ");
    println!(" {} | {} | {} ", cells[6], cells[7], cells[8]);
}
