#![allow(non_snake_case)]


use std::io::{self};

fn clear_console(){
    if cfg!(target_os = "windows"){
        std::process::Command::new("cls").status().unwrap();
    }
    else if cfg!(target_os = "linux") || cfg!(target_os = "macos"){
        std::process::Command::new("clear").status().unwrap();
    }
        
    }

fn verify_winner(play: [String; 9]) -> String {
let mut winner: &str = "-";
    // winner = "-";
    if play[0] ==  play[1] && play[0] ==  play[2] {
        winner = play[0].as_str();}

    else if play[3] == play[4] && play[3] == play[5] {
        winner = play[3].as_str();
    }
    else if play[6] == play[7] && play[6] == play[8] {
        winner = play[6].as_str();
    }
    else if play[0] == play[3] && play[0] == play[6] {
        winner = play[0].as_str();
    }
    else if play[1] == play[4] && play[1] == play[7] {
        winner = play[1].as_str();
    }
    else if play[2] == play[5] && play[2] == play[8] {
        winner = play[2].as_str();
    }
    else if play[0] == play[4] && play[0] == play[8] {
        winner = play[0].as_str();
    }
    else if play[2] == play[4] && play[2] == play[6] {
        winner = play[2].as_str();
        
    }    

    if play[0] != "-" && play[1] != "-" && play[2] != "-" && play[3] != "-" && play[4] != "-" && play[5] != "-" && play[6] != "-" && play[7] != "-" && play[8] != "-" {
        winner = "DRAW";
    }
        
        
    return winner.to_string();



}

fn verify_board(charact: &str, play: [String; 9]) -> bool {
    let mut verify: bool = true;
    let chr: usize = charact.parse().unwrap();

        if play[chr] != "-" {
            verify = false;

            println!("YOU CAN'T ENTER THAT CHARACTER\n");
        }

        
    return verify;
    }


fn game(){
    let X: String = "X".to_string();
    let O: String = "O".to_string();
    let mut is_X_time: bool = true;
    let mut win: String;
    let mut log: String = "".to_string();


    let mut board: [String; 9] = ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()];
    loop {
        clear_console();
        if is_X_time{
            println!("\nTic Tac Toe!!! | {} Times", X);

        }
        else {
            println!("\nTic Tac Toe!!! | {} Times", O);
        }
        println!("{}", log);
        println!("--------------------");   

        println!("| {} {} {}  | 1  2  3 |",board[0], board[1], board[2]);   
        println!("| {} {} {}  | 4  5  6 |",board[3], board[4], board[5]); 
        println!("| {} {} {}  | 7  8  9 |",board[6], board[7], board[8]); 

        win = verify_winner(board.clone());
        if win != "-".to_string() && win == "X".to_string() || win == "O".to_string() {
            println!("\nGAME OVER!! | {} WIN!! ", win);
            break;
        }

        else if win != "-".to_string() && win != "X".to_string() && win != "O".to_string() {
            println!("\nGAME OVER!! | {}!! ", win);
            break;
        }

        println!("Input a corresponding number: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input: usize = input.trim().parse().unwrap();

        if input > 9 || input < 1 {
            log = "INVALID INPUT".to_string();
            println!("\nINVALID INPUT");
            continue;
        }

        input = input - 1;
        
         if verify_board(input.to_string().as_str(), board.clone()) == true{

            let inp: u32 = input as u32;
            


             if is_X_time {
                 board[inp as usize] = "X".to_string();
                 is_X_time = false;
                 log = "".to_string();            
                }
                else {
                    board[inp as usize] = "O".to_string();
                    is_X_time = true;
                    log = "".to_string();

             
                }
            }

        else {
            log = "INVALID INPUT".to_string();
        }


    }}
    



fn main() {
    game();
}