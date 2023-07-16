use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Bets {
    // holding up to 6 bet sequences
    bet_seq_1: [u32; 12],
    bet_seq_2: [u32; 12],
    bet_seq_3: [u32; 12],
    bet_seq_4: [u32; 12],
    bet_seq_5: [u32; 12],
    bet_seq_6: [u32; 12],
}

impl Bets {
    pub fn new() -> Bets {
        Bets {
            bet_seq_1: [0; 12],
            bet_seq_2: [0; 12],
            bet_seq_3: [0; 12],
            bet_seq_4: [0; 12],
            bet_seq_5: [0; 12],
            bet_seq_6: [0; 12],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Wins {
    winning_seq: [u32; 12],
    won_combos: HashMap<[u32; 12], usize>,
}

impl Wins {
    pub fn new() -> Wins {
        Wins {
            winning_seq: [0; 12],
            won_combos: HashMap::<[u32; 12], usize>::new(),
        }
    }
}

pub fn add_bet(bets: &mut Bets, bet: [u32; 12]) -> Result<&Bets, &'static str> {
    if bets.bet_seq_1.iter().fold(0, |a, &b| a + b) == 0 {
        bets.bet_seq_1 = bet;
    } else if bets.bet_seq_2.iter().fold(0, |a, &b| a + b) == 0 {
        bets.bet_seq_2 = bet;
    } else if bets.bet_seq_3.iter().fold(0, |a, &b| a + b) == 0 {
        bets.bet_seq_3 = bet;
    } else if bets.bet_seq_4.iter().fold(0, |a, &b| a + b) == 0 {
        bets.bet_seq_4 = bet;
    } else if bets.bet_seq_5.iter().fold(0, |a, &b| a + b) == 0 {
        bets.bet_seq_5 = bet;
    } else if bets.bet_seq_6.iter().fold(0, |a, &b| a + b) == 0 {
        bets.bet_seq_6 = bet;
    } else {
        return Err("Too many bets");
    }

    Ok(bets)
}

pub fn add_winning_seq(wins: &mut Wins, winning_nums: [u32; 12]) {
    wins.winning_seq = winning_nums;
}

pub fn calculate_wins(bets: Bets, wins: &mut Wins) -> Result<&Wins, &'static str> {
    // Calculate the number of hits for each bet sequence
    let (bet_seq_1, hits) = match_bets_helper(bets.bet_seq_1, wins.winning_seq);
    if hits >= 3 {
        wins.won_combos.insert(bet_seq_1, hits);
    }

    let (bet_seq_2, hits) = match_bets_helper(bets.bet_seq_2, wins.winning_seq);
    if hits >= 3 {
        wins.won_combos.insert(bet_seq_2, hits);
    }

    let (bet_seq_3, hits) = match_bets_helper(bets.bet_seq_3, wins.winning_seq);
    if hits >= 3 {
        wins.won_combos.insert(bet_seq_3, hits);
    }

    let (bet_seq_4, hits) = match_bets_helper(bets.bet_seq_4, wins.winning_seq);
    if hits >= 3 {
        wins.won_combos.insert(bet_seq_4, hits);
    }

    let (bet_seq_5, hits) = match_bets_helper(bets.bet_seq_5, wins.winning_seq);
    if hits >= 3 {
        wins.won_combos.insert(bet_seq_5, hits);
    }

    let (bet_seq_6, hits) = match_bets_helper(bets.bet_seq_6, wins.winning_seq);
    if hits >= 3 {
        wins.won_combos.insert(bet_seq_6, hits);
    }

    Ok(wins)
}

pub fn str_to_u32_array(str: &str) -> [u32; 12] {
    let mut arr: [u32; 12] = [0; 12];

    let vec_str: Vec<&str> = str.split(' ').collect();
    println!("vec_str: {:?}", vec_str);

    for (i, s) in vec_str.iter().enumerate() {
        println!("i: {}, s: {}", i, s);
        arr[i] = s.trim().parse::<u32>().unwrap();
    }

    arr
}

/* --------- Private helper functions --------- */
fn match_bets_helper(bet_seq: [u32; 12], winning_seq: [u32; 12]) -> ([u32; 12], usize) {
    let max_length_of_bet_seq = bet_seq.len(); // every bet seq has the same length
    let max_length_of_winning_seq = winning_seq.len();
    let mut hits = 0;

    for i in 0..max_length_of_bet_seq {
        for j in 0..max_length_of_winning_seq {
            if bet_seq[i] == winning_seq[j] && winning_seq[j] != 0 {
                hits += 1;
            }
        }
    }

    (bet_seq, hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_bet() {
        let mut bets = Bets::new();
        println!("bets: {:?}", bets);

        let bet_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let bet_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let bet_3 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let bet_4 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let bet_5 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let bet_6 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        add_bet(&mut bets, bet_1); // warning -ignore for now
        add_bet(&mut bets, bet_2);
        add_bet(&mut bets, bet_3);
        add_bet(&mut bets, bet_4);
        add_bet(&mut bets, bet_5);
        add_bet(&mut bets, bet_6);

        println!("Added into bets: {:?}", bets);

        assert_eq!(&bets.bet_seq_1, &bet_1);
        assert_eq!(&bets.bet_seq_2, &bet_2);
        assert_eq!(&bets.bet_seq_3, &bet_3);
        assert_eq!(&bets.bet_seq_4, &bet_4);
        assert_eq!(&bets.bet_seq_5, &bet_5);
        assert_eq!(&bets.bet_seq_6, &bet_6);
    }

    #[test]
    fn test_str_to_u32_array() {
        let str = "1 2 3 4 5 6 7 8 9 10 11 12";
        let arr = str_to_u32_array(str);

        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_add_winning_seq() {
        let mut wins = Wins::new();
        println!("Empty Wins struct: {:?}", wins);

        let winning_nums = [1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0];

        add_winning_seq(&mut wins, winning_nums);
        println!("Added into wins: {:?}", wins);

        assert_eq!(&wins.winning_seq, &winning_nums);
    }

    #[test]
    fn test_calculate_wins() {
        let mut bets = Bets::new();
        let mut wins = Wins::new();

        // Setup winning sequence first
        let winning_nums = [1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0];
        add_winning_seq(&mut wins, winning_nums);
        println!("Added into wins: {:?}\n", wins);

        // Setup different bets
        let bet_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        add_bet(&mut bets, bet_1);

        let bet_2 = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];
        add_bet(&mut bets, bet_2);

        let bet_3 = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36];
        add_bet(&mut bets, bet_3);

        println!("Added into bets: {:?}\n", bets);

        calculate_wins(bets, &mut wins);
        println!("Calculated wins: {:?}", wins);

        // Assert different winning combos
        assert_eq!(wins.won_combos.get(&bet_1), Some(&7));
        assert_eq!(wins.won_combos.get(&bet_2), Some(&3));
        assert_eq!(wins.won_combos.get(&bet_3), None);
    }
}
