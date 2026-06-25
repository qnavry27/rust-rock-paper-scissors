use std::io;

fn main() {
    loop {
        let mut player_input = String::new();
    println!("Welcome to Rock Paper Scissors!");
    println!("Player, please decide.");

    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read input\n");

    let player_input = player_input.trim().to_lowercase();

    if player_input == "rock" || player_input == "paper" || player_input == "scissors" {
        println!("You chose: {}", player_input);
        break;
    } else {
       println!("Invalid input, try again.\n");
        }
    }
}

