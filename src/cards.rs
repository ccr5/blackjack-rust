pub mod cards {

    pub struct Card {
        name: String,
        height: Vec<i8>,
        ticker: String
    }

    impl Card {
        pub fn new(name: String, height: Vec<i8>, ticker: String) -> Self {
            Card { name, height, ticker }
        }

        pub fn get_height(&self) -> &Vec<i8> {
            &self.height
        }

        pub fn get_ticker(&self) -> &String {
            &self.ticker
        }

        pub fn show_card(&self) {

            if self.name == "Ten" {
                println!("- - - -");
                println!("|     |");
                println!("| {}  |", self.ticker);
                println!("|     |");
                println!("- - - -");
            } else {
                println!("- - - -");
                println!("|     |");
                println!("|  {}  |", self.ticker);
                println!("|     |");
                println!("- - - -");
            }
        }
    }

}