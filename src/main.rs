use std::io;

use toto_num_matcher::{Bets, Wins};

fn main() {
    // For storing our Bets in a proper struct
    let mut bets = Bets::new();

    // For storing our wins in a proper struct
    let mut win = Wins::new();

    // create a loop to get the bet numbers; break when 'q' encountered.
    loop {
        // Getting bet numbers (set by set)
        println!(
            "\nEnter your bet sequence. Each number to be followed by a space. Type 'q' to quit. Press <Enter> when done."
        );

        // For storing CLI inputs
        let mut bet_nums_input = String::new();

        io::stdin()
            .read_line(&mut bet_nums_input)
            .expect("Not a proper number!"); // TODO - not propgating error when non number is input

        // If 'q' given, break the loop and end the program, else parse into u32
        if bet_nums_input.trim().contains("q") {
            break; // out of the loop to finish getting bet sequences
        } else {
            let a_bet = toto_num_matcher::str_to_u32_array(&bet_nums_input);
            toto_num_matcher::add_bet(&mut bets, a_bet).expect("Cannot add this bet");
            println!("bets: {:?}", bets);
        }
    }

    // Getting winnng number (from CLI for now)
    println!("\nEnter the winning number. Press <Enter> when done.");

    let mut winning_num_input = String::new();

    io::stdin()
        .read_line(&mut winning_num_input)
        .expect("Not a proper number!");

    let winning_seq = toto_num_matcher::str_to_u32_array(&winning_num_input);
    toto_num_matcher::add_winning_seq(&mut win, winning_seq);

    let winning_combo = toto_num_matcher::calculate_wins(bets, &mut win);

    println!("Matching winning numbers are: {:?}", winning_combo);
}
