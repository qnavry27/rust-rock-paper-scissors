use std::{io, process::exit};
use rand::{self, seq::SliceRandom, thread_rng};

fn main() {
    loop {
        let mut player_input = String::new();
        let computer_options = ["rock", "paper", "scissors"];
        let mut rng = thread_rng();
        println!("Welcome to Rock Paper Scissors!");
        println!("Player, please decide:");

        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read input\n");

        let player_input = player_input.trim().to_lowercase();

        if player_input == "rock" || player_input == "paper" || player_input == "scissors" {
            println!("You chose: {}", player_input);
        } else {
            println!("Invalid input, try again.\n");
            continue;
        }
         let computer_pick: &str = computer_options.choose(&mut rng).unwrap();
         println!("Computer chose: {}", computer_pick);

        if player_input == computer_pick {
            println!("Its a draw!\n");
        }
        else if (player_input == "rock" && computer_pick == "scissors") ||
        (player_input == "paper" && computer_pick == "rock") ||
        (player_input == "scissors" && computer_pick == "paper") {
            println!("You win!\n");
        }
        else {
            println!("Computer wins!\n")
        }

        println!("Want to play again? (y(es)/n(o))");
        let mut play_again = String::new();
        io::stdin()
            .read_line(  &mut play_again)
            .expect("Failed to read input\n");
        let play_again = play_again.trim().to_lowercase();
        if play_again == "yes" || play_again == "y" {
            println!("\n");
            continue;
        } else {
            println!("Goodbye!");
            exit(0);
        }
    }
 }