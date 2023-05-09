//Program 3: Tic-Tac-Toe Game
//Author Josh Ripoli

use std::io;

//displays the game board
fn print_board(spaces_list: &mut Vec<i32>, spaces_by_player: &mut Vec<char>) {
    let mut start_index: usize = 0;
    println!();
    for i in 0..6 {
        if i % 2 == 0 {
            if i != 0 {
                println!("   |   |");
            }
        } else {
            if spaces_by_player[start_index] == 'X' || spaces_by_player[start_index] == 'O' {
                print!("_{}_|", spaces_by_player[start_index]);
            } else {
                print!("_{}_|", spaces_list[start_index]);
            }
            if spaces_by_player[start_index + 1] == 'X' || spaces_by_player[start_index + 1] == 'O'
            {
                print!("_{}_|", spaces_by_player[start_index + 1]);
            } else {
                print!("_{}_|", spaces_list[start_index + 1]);
            }
            if spaces_by_player[start_index + 2] == 'X' || spaces_by_player[start_index + 2] == 'O'
            {
                print!("_{}_", spaces_by_player[start_index + 2]);
            } else {
                println!("_{}_", spaces_list[start_index + 2]);
            }
            start_index += 3;
        }
    }
}

//checks to see if user input matches the number 1-9
fn check_input(input: usize, spaces_list: &mut Vec<bool>) -> bool {
    if spaces_list[input - 1] {
        println!("This space is already in use\n");
        return false;
    }
    return true;
}

//checks to see if a player has won the game
fn check_for_winner(
    spaces_taken: &mut Vec<bool>,
    spaces_by_player: &mut Vec<char>,
    current_player: char,
) -> bool {
    for i in 0..3 {
        //3 in a row starting from top row
        //down for all columns
        if spaces_taken[i] == true && spaces_by_player[i] == current_player {
            if spaces_taken[i + 3] == true && spaces_by_player[i + 3] == current_player {
                if spaces_taken[i + 6] == true && spaces_by_player[i + 6] == current_player {
                    return true;
                }
            }
            //across top row
            if spaces_taken[i + 1] == true && spaces_by_player[i + 1] == current_player {
                if spaces_taken[i + 2] == true && spaces_by_player[i + 2] == current_player {
                    return true;
                }
            }
            //diagnol from top left
            if spaces_taken[i + 4] == true && spaces_by_player[i + 4] == current_player {
                if spaces_taken[i + 8] == true && spaces_by_player[i + 8] == current_player {
                    return true;
                }
            }
            //diagnol from top right
            if spaces_taken[i + 2] == true && spaces_by_player[i + 2] == current_player {
                if spaces_taken[i + 4] == true && spaces_by_player[i + 4] == current_player {
                    return true;
                }
            }
        }
    }
    //across for 2nd and 3rd rows
    if spaces_taken[3] == true && spaces_by_player[3] == current_player {
        if spaces_taken[4] == true && spaces_by_player[4] == current_player {
            if spaces_taken[5] == true && spaces_by_player[5] == current_player {
                return true;
            }
        }
    }
    if spaces_taken[6] == true && spaces_by_player[6] == current_player {
        if spaces_taken[7] == true && spaces_by_player[7] == current_player {
            if spaces_taken[8] == true && spaces_by_player[8] == current_player {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let mut current_player = 'X';
    let mut player_number = 1;
    let mut spaces_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut spaces_taken: Vec<bool> = vec![
        false, false, false, false, false, false, false, false, false,
    ];
    let mut spaces_by_player: Vec<char> = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut spaces_remaining = 9;

    //introduction
    println!("Welcome to the Tic-Tac-Toe program!");
    println!("Player 1 will be X");
    println!("Player 2 will be O\n");

    let mut game_over: bool =
        check_for_winner(&mut spaces_taken, &mut spaces_by_player, current_player);
    //runs the game checks for 3 in a row or no more spaces to play in
    while game_over != true && spaces_remaining > 0 {
        print_board(&mut spaces_list, &mut spaces_by_player);
        let mut input = String::new();
        println!(
            "\nPlayer {}, Enter the number for the space you want to choose: ",
            player_number
        );
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline")
            .to_string();
        let index: usize = input.trim().parse().expect("Input not an integer");

        //checks for space chosen being available
        let valid_space = check_input(index, &mut spaces_taken);
        if valid_space {
            spaces_by_player[index - 1] = current_player;
            spaces_taken[index - 1] = true;

            //checks if the player won the game
            game_over = check_for_winner(&mut spaces_taken, &mut spaces_by_player, current_player);
            //switches players
            if player_number == 2 {
                player_number -= 1;
                current_player = 'X';
            } else {
                player_number += 1;
                current_player = 'O';
            }
            spaces_remaining -= 1;
        }
    }

    //the game has been complete
    if current_player == 'X' {
        println!("\nPlayer 2 wins!")
    } else {
        println!("\nPlayer 1 wins!");
    }
    println!("Game over.");
}
