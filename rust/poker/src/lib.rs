use self::Suit::*;
use self::Rank::*;
use self::PokerHands::*;

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

#[derive(Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight, 
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

// pub struct Hand<'a>(Vec<&'a str>);

// impl<'a> Hand<'a> {
//     pub fn new(hand: Vec<&'a str>) -> Self {
//         Self(hand)
//     }
//     pub fn isRoyalFlush(&self) -> bool {}
//     pub fn isStraightFlush(&self) -> bool {};
//     pub fn isFourOfAKind(&self) -> bool {};
//     pub fn isFullHouse(&self) -> bool {};
//     pub fn isFlush(&self) -> bool {};
//     pub fn isStraight(&self) -> bool {};
//     pub fn isThreeOfAKind(&self) -> bool {};
//     pub fn isTwoPair(&self) -> bool {};
//     pub fn isOnePair(&self) -> bool {};
//     pub fn isHighCard(&self) -> bool {};
// }

pub fn parse_hand<'a>(hand: Vec<String>) -> Vec<(Suit, Rank)> {
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
            ['0', '1'] => Ok(One),
            ['0', '2'] => Ok(Two),
            ['0', '3'] => Ok(Three),
            ['0', '4'] => Ok(Four),
            ['0', '5'] => Ok(Five),
            ['0', '6'] => Ok(Six),
            ['0', '7'] => Ok(Seven),
            ['0', '8'] => Ok(Eight),
            ['0', '9'] => Ok(Nine),
            ['1', '0'] => Ok(Ten),
            ['0', 'J'] => Ok(Jack),
            ['0', 'Q'] => Ok(Queen),
            ['0', 'K'] => Ok(King),
            ['0', 'A'] => Ok(Ace),
            _ => Err(Error::RankNotFound),
       };
       (suit.unwrap(), rank.unwrap())
   }).collect::<Vec<(Suit, Rank)>>() 
}

pub fn sort_hand(hand: Vec<(Suit, Rank)>) -> Vec<(Suit, Rank)> {
    hand
}

// pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
//     unimplemented!("Out of {:?}, which hand wins?", hands)
// }

pub fn main() {
    let test = &["4S 5S 7H 8D JC"];
    let vec: Vec<String> = test.iter().flat_map(|x| x.split(' ').map(|b| b.to_string())).collect();
    let resp1 = parse_hand(vec);
    let resp = sort_hand(resp1);
    println!("{:?}", resp);
}
