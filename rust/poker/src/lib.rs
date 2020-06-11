use self::Suit::*;
use self::PokerHands::*;

pub type Cards = Vec<(usize, Suit)>;

#[derive(Debug)]
pub enum Error {
    RankNotFound,
    SuitNotFound
}

pub enum PokerHands {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

pub struct Hand {
    cards: Cards
}

impl Hand {
    pub fn new(hand: Cards) -> Self {
        Self{ cards: hand }
    }
    
    pub fn consecutive(&self) -> bool {
        let (min_rank, _) = self.cards[0];
        let (max_rank, _) = self.cards[self.cards.len() - 1];
        // only check if the range is correct
        if max_rank - min_rank + 1 == self.cards.len() {
            (0..(self.cards.len() - 1)).fold(false, |isit, x| {
                let (a, _) = self.cards[x];
                let (b, _) = self.cards[x + 1];
                a + 1 == b
            })
        } else {
            false
        }
    }

    // TODO: implement these checks
    // n are the same
    // pub fn hasDuplicates(&self, number: usize) -> bool {}
    // same suit
    // pub fn sameSuits(&self, number: usize) -> bool {}

    // if cards are same suit, consecutive and start at J (11)
    // pub fn isRoyalFlush(&self) -> bool {
    //     // self.consecutive() && self.hand
    // }
    // // if cards are same suit, consecutive
    // pub fn isStraightFlush(&self) -> bool {};
    // // if four as the same
    // pub fn isFourOfAKind(&self) -> bool {};
    // // if 3 are same and 4 are same
    // pub fn isFullHouse(&self) -> bool {};
    // // if suits are same
    // pub fn isFlush(&self) -> bool {};
    // // if is consecutive
    // pub fn isStraight(&self) -> bool {};
    // // if 3 are same
    // pub fn isThreeOfAKind(&self) -> bool {};
    // // if 2 are same and 2 are same
    // pub fn isTwoPair(&self) -> bool {};
    // // if 2 are same
    // pub fn isOnePair(&self) -> bool {};
    // // highest card
    // pub fn isHighCard(&self) -> bool {};
}

pub fn parse_hand<'a>(hand: Vec<String>) -> Cards {
    hand.iter().map(|card| {
       let formatted_card = format!("{:0>3}", card); 
       let chars: Vec<char> = formatted_card.chars().collect();
       let suit = match chars.last() {
            Some('S') => Ok(Spade),
            Some('H') => Ok(Heart),
            Some('D') => Ok(Diamond),
            Some('C') => Ok(Club),
            _ => Err(Error::SuitNotFound),
       };
       let rank = match [chars[0], chars[1]] {
            ['0', '1'] => Ok(1),
            ['0', '2'] => Ok(2),
            ['0', '3'] => Ok(3),
            ['0', '4'] => Ok(4),
            ['0', '5'] => Ok(5),
            ['0', '6'] => Ok(6),
            ['0', '7'] => Ok(7),
            ['0', '8'] => Ok(8),
            ['0', '9'] => Ok(9),
            ['1', '0'] => Ok(10),
            ['0', 'J'] => Ok(11),
            ['0', 'Q'] => Ok(12),
            ['0', 'K'] => Ok(13),
            ['0', 'A'] => Ok(14),
            _ => Err(Error::RankNotFound),
       };
       (rank.unwrap(), suit.unwrap())
   }).collect::<Cards>() 
}

pub fn sort_hand(mut hand: Cards) -> Cards {
    hand.sort_by(|(a, _), (b, _)| a.cmp(b));
    hand
}

// pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
//     unimplemented!("Out of {:?}, which hand wins?", hands)
// }

pub fn main() {
    let test = &["4S 5S 6H 7D 8C"];
    let vec: Vec<String> = test.iter().flat_map(|x| x.split(' ').map(|b| b.to_string())).collect();
    let resp1 = parse_hand(vec);
    let resp = sort_hand(resp1);
    let hand = Hand::new(resp);
    println!("{:?}", hand.consecutive());
}
