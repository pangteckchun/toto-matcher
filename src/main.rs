use std::collections::HashMap;
use std::io;

use toto_num_matcher::Bets;

fn main() {
    // Hold the bet numbers in vector
    let mut vec_bet_seqs: Vec<_> = Vec::new();

    // create a loop to get the bet numbers; break when 'q' encountered.
    loop {
        // Getting bet numbers (set by set)
        println!(
            "Enter your bet sequence. Each number to be followed by a space. Type 'q' to quit. Press <Enter> when done."
        );

        let mut bet_nums_input = String::new(); // for storing CLI inputs
        let mut bets = Bets::new(); // for storing our Bets in a proper struct

        io::stdin()
            .read_line(&mut bet_nums_input)
            .expect("Not a proper number!"); // TODO - not propgating error when non number is input

        // If 'q' given, break the loop and end the program, else parse into u32
        if bet_nums_input.trim().contains("q") {
            break; // out of the loop to finish getting bet sequences
        } else {
            let a_bet = toto_num_matcher::str_to_u32_array(&bet_nums_input);
            toto_num_matcher::add_bet(&mut bets, a_bet); // TODO: handle error properly

            println!("bets: {:?}", bets);

            let vec_bet_nums = extract_nums(&bet_nums_input); // vec_bet_nums is local scope in the if-else block only
            vec_bet_seqs.push(vec_bet_nums);
        }
    }
    println!("Bet sequences are: {:?}", vec_bet_seqs);

    // Getting winnng number (from CLI for now)
    println!("Enter the winning number. Press <Enter> when done.");

    let mut winning_num = String::new();
    let mut vec_matching_win: Vec<u32> = Vec::new(); // Hold the matching winning numbers

    // Prepare HashMap to store the bet numbers and their num of hits of the winning numbers
    let mut bet_num_hits: HashMap<Vec<u32>, Vec<u32>> = HashMap::<Vec<u32>, Vec<u32>>::new();

    io::stdin()
        .read_line(&mut winning_num)
        .expect("Not a proper number!");

    let vec_winning_num = extract_nums(&winning_num);

    for vec_bet_nums in &vec_bet_seqs {
        vec_matching_win.clear(); // clear the vector for the next bet sequence

        // vec_bet_nums is local scope in the for loop only
        for each_bet_num in vec_bet_nums {
            // Match each bet number with each winning number
            if vec_winning_num.contains(each_bet_num) {
                vec_matching_win.push(*each_bet_num);
            }
        }

        // Check if matching win exceeds 3 numbers to consider it a win
        if vec_matching_win.len() >= 3 {
            bet_num_hits.insert(vec_bet_nums.to_vec(), vec_matching_win.to_vec());
        }
    }

    println!(
        "Matching winning numbers are: {:?}. You striked {} sequences!",
        bet_num_hits,
        bet_num_hits.len()
    );
}

// Parse string vector into integer vector and returns a sorted vec
fn extract_nums(delimited_str: &str) -> Vec<u32> {
    // Parse the string with the whitesapces and return a vector, using collect(), to strings
    let vec_winning_num: Vec<&str> = delimited_str.split(' ').collect();

    let mut vec_nums: Vec<u32> = Vec::new();

    for a_bet_num in vec_winning_num {
        let a_bet_num: u32 = match a_bet_num.trim().parse() {
            // reuse a_bet_num as a u32 type
            Ok(num) => num, // IMPORTANT! do not annotate type here else there will compilation errors!
            Err(_) => 0,    // default to zero if number cannot be parsed
        };

        vec_nums.push(a_bet_num); // store the set of u32 bet numbers in a vector
        vec_nums.sort(); // sort by ascending order
    }

    vec_nums // returning the integer vec
}
