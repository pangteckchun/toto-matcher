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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_bet() {
        let mut bets = Bets::new();
        println!("bets: {:?}", bets);

        let bet = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        add_bet(&mut bets, bet); // warning -ignore for now
        println!("Added into bets: {:?}", bets);

        assert_eq!(&bets.bet_seq_1, &bet);
    }
}
