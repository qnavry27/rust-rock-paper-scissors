//! # Rock Paper Scissors Game
//! 
//! This program provides a fully interactive implementation
//! of the classic Rock, Paper, Scissors game.

use std::io;
use rand::{self, seq::SliceRandom, thread_rng};

// ====================================
// 1. DATA STRUCTURES & IMPLEMENTATIONS
// ====================================

/// Represents the choices available for the player and computer to choose from.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Choice {
        Rock, 
        Paper,
        Scissors,
}

impl Choice {
    /// Compares the player's choice against the computer's pick
    /// and returns whether it was a Win, Loss, or Tie.
    fn check_outcome(self, other: Choice) -> Outcome {
        if self == other {
            Outcome::Tie
        } else {
            match (self, other) {
                (Choice::Rock, Choice::Scissors) |
                (Choice::Paper, Choice::Rock) |
                (Choice::Scissors, Choice::Paper) => Outcome::Win,
                _ => Outcome::Loss,
            }
        }
    }
}

/// Controls whether the game loop should keep running or terminate.
#[derive(Debug, PartialEq)]
enum GameState{
    Continue,
    Quit,
}

/// Represents the outcome of a single round from the player's perspective.
#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

// ===================
// 2. HELPER FUNCTIONS
// ===================

/// Prompts the player for input, cleans it, and maps it safely to a "Choice".
/// Loops internally until a valid option is provided.
fn get_player_input() -> Choice {
    loop{
        println!("Please decide (rock, paper, scissors):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "rock" => return Choice::Rock,
            "paper" => return Choice::Paper,
            "scissors" => return Choice::Scissors,
            _ => println!("Invalid choice, Please type rock, paper or scissors.\n"),
        }
    }
}

/// Asks the player if they want to play another round, returning a "GameState".
fn get_rematch_status() -> GameState {
    loop {
        println!("Dou you want to play another round (y/n):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return GameState::Continue,
            "n" | "no" | "quit" => return GameState::Quit,
            _ => println!("Please type 'y' for yes or 'n' for no.\n"),
        }
    }
}

// =================
// 3. MAIN GAME LOOP
// =================
fn main() {
    println!("Welcome to Rock Paper Scissors!");

    let computer_options = [Choice::Rock, Choice::Paper, Choice::Scissors];
    loop {
        let mut rng = thread_rng();
        
        let player_choice = get_player_input();
        let computer_pick =  *computer_options.choose(&mut rng).unwrap();
         
        println!("You chose: {:?}", player_choice);
        println!("Computer chose: {:?}\n", computer_pick);

        match player_choice.check_outcome(computer_pick) {
            Outcome::Tie => println!("Its a tie!"),
            Outcome::Win => println!("You win this round!"),
            Outcome::Loss => println!("Computer wins this round!"),
        };
        
        if get_rematch_status() == GameState::Quit {
            println!("Thanks for playing, Bye!");
            break;
        }
        println!("\n");
    }
 }