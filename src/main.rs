use std::io;

fn main() {
    // Getting bet numbers (1 set for now)
    let mut bet_nums = String::new();

    println!(
        "Enter your bet sequence. Each number to be followed by a space. Press <Enter> when done."
    );

    // Getting betting numbers from CLI
    // TODO: Get a set of bet numbers rather than just once
    io::stdin()
        .read_line(&mut bet_nums)
        .expect("Not a proper number!");

    // Parse the string with the whitesapces and return a vector, using collect(), to strings
    let vec_bet_nums = extract_nums(&bet_nums);

    // Getting winnng number (from CLI for now)
    let mut winning_num = String::new();

    println!("Enter the winning number. Press <Enter> when done.");

    io::stdin()
        .read_line(&mut winning_num)
        .expect("Not a proper number!");

    let vec_winning_num = extract_nums(&winning_num);

    // Compare the bet numbers with the winning numbers
    let mut vec_matching_win: Vec<u32> = Vec::new();

    for each_bet_num in &vec_bet_nums {
        println!("Bet num is: {}", each_bet_num);

        // match each bet number with each winning number
        if vec_winning_num.contains(each_bet_num) {
            vec_matching_win.push(*each_bet_num);
        }
    }

    println!("Matching winning numbers are: {:?}", vec_matching_win);
}

// Parse string vector into integer vector and returns a sorted vec
fn extract_nums(delimited_list: &str) -> Vec<u32> {
    let vec_winning_num: Vec<&str> = delimited_list.split(' ').collect();

    let mut vec_nums: Vec<u32> = Vec::new();

    for a_bet_num in vec_winning_num {
        let a_bet_num: u32 = match a_bet_num.trim().parse() {
            // reuse a_bet_num as a u32 type
            Ok(num) => num, // IMPORTANT! do not annotate type here else there will compilation errors!
            Err(_) => 0,    // default to zero if number cannot be parsed
        };

        println!("bet num: {}", a_bet_num);

        vec_nums.push(a_bet_num); // store the set of u32 bet numbers in a vector
        vec_nums.sort(); // sort by ascending order
    }

    vec_nums // returning the integer vec
}
