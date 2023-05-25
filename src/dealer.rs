pub mod dealer {
    pub fn check_as(count_a: i8, sum: i8) -> i8 {
        if count_a >= 1 && sum > 10 {
            sum += 1 * count_a;
        } else if count_a == 1 && sum <= 10 {
            sum += 11;
        }

        sum
    }

    pub fn check_winner(hand1: Vec<Card>, hand2: Vec<Card>) -> (str, i8, i8) {
        let h1: i8 = 0;
        let h2: i8 = 0;
        let count_a_h1: i8 = 0;
        let count_a_h2: i8 = 0;

        for card in hand1 {
            if card.ticker == 'A' {
                count_a_h1 += 1;
            } else {
                h1 += card.value
            }
        }

        for card in hand2 {
            if card.ticker == 'A' {
                count_a_h1 += 1;
            } else {
                h1 += card.value
            }
        }

        let verify_h1 = check_as(count_a_h1, h1);
        let verify_h2 = check_as(count_a_h2, h2);

        if verify_h1 > 21 && verify_h2 > 21 {
            ("no win", verify_h1, verify_h2)
        } else if verify_h1 <= 21 && verify_h2 > 21 {
            ("h1", verify_h1, verify_h2)
        } else if h1 > 21 && h2 <= 21 {
            ("h2", verify_h1, verify_h2)
        } else if h1 == h2 {
            ("draw", verify_h1, verify_h2)
        } else if h1 == 21 {
            ("h1", verify_h1, verify_h2)
        } else if h2 == 21 {
            ("h2", verify_h1, verify_h2)
        } else if h1 > h2 {
            ("h1", verify_h1, verify_h2)
        } else if h2 > h1 {
            ("h2", verify_h1, verify_h2)
        }
    }
}