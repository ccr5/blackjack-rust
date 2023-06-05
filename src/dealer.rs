pub mod dealer {
    use crate::cards::cards::Card;

    fn check_as(count_a: i8, sum: i8) -> i8 {
        let mut new_sum = sum;

        if count_a == 1 && sum <= 10 {
            new_sum += 11;
        } else {
            new_sum += 1 * count_a;
        }

        new_sum
    }

    pub fn check_winner(hand1: &Vec<Card>, hand2: &Vec<Card>) -> Option<(String, i8, i8)> {
        let mut h1: i8 = 0;
        let mut h2: i8 = 0;
        let mut count_a_h1: i8 = 0;
        let mut count_a_h2: i8 = 0;

        for card in hand1 {
            if card.get_ticker().to_string() == "A" {
                count_a_h1 += 1;
            } else {
                h1 += card.get_height().get(0).unwrap()
            }
        }

        for card in hand2 {
            if card.get_ticker().to_string() == "A" {
                count_a_h2 += 1;
            } else {
                h2 += card.get_height().get(0).unwrap()
            }
        }

        let verify_h1 = check_as(count_a_h1, h1);
        let verify_h2 = check_as(count_a_h2, h2);

        if verify_h1 > 21 && verify_h2 > 21 {
            Some(("no win".to_string(), verify_h1, verify_h2))
        } else if verify_h1 <= 21 && verify_h2 > 21 {
            Some(("h1".to_string(), verify_h1, verify_h2))
        } else if h1 > 21 && h2 <= 21 {
            Some(("h2".to_string(), verify_h1, verify_h2))
        } else if h1 == h2 {
            Some(("draw".to_string(), verify_h1, verify_h2))
        } else if h1 == 21 {
            Some(("h1".to_string(), verify_h1, verify_h2))
        } else if h2 == 21 {
            Some(("h2".to_string(), verify_h1, verify_h2))
        } else if h1 > h2 {
            Some(("h1".to_string(), verify_h1, verify_h2))
        } else if h2 > h1 {
            Some(("h2".to_string(), verify_h1, verify_h2))
        } else {
            None
        }
    }
}