use std::io;
use rand::{self, seq::SliceRandom, thread_rng};

fn main() {
    loop {
        let mut player_input = String::new();
        let computer_options = ["Rock", "Paper", "Scissors"];
        let mut rng = thread_rng();
        println!("Welcome to Rock Paper Scissors!");
        println!("Player, please decide.");

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
        if let Some(computer_pick) = computer_options.choose(&mut rng){
            println!("The computer's choice is: {}\n ", computer_pick);
        } else {
            println!("The list was empty");
        }
    }
}
