pub mod cards {

    pub struct Card {
        name: String,
        height: Vec<u8>,
        ticker: String
    }

    impl Card {
        pub fn new(name: String, height: Vec<u8>, ticker: String) -> Self {
            Card { name, height, ticker }
        }

        pub fn show_card(&self) {
            println!("{}", self.name)
        }
    }

}