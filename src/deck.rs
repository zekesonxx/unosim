//! An Uno Deck
//!
//! An Uno deck consists of:
//! * 4 zero cards (one per color): 4
//! * 1-9 (inclusive) numerical cards (two per color): 72
//! * Skip, Reverse, and Draw Two cards (two per color): 24
//! * Four Wild Cards: 4
//! * Four Draw Four Wild Cards: 4
//!
//! Total = 4+72+24+4+4 = 108
//!
//! [Nice CC0 Card SVG](https://en.wikipedia.org/wiki/File:UNO_cards_deck.svg)

use cards::*;

macro_rules! per_color {
    ($deck: ident, $a: expr) => (
        $deck.push(Card::new(Color::Red, $a));
        $deck.push(Card::new(Color::Green, $a));
        $deck.push(Card::new(Color::Blue, $a));
        $deck.push(Card::new(Color::Yellow, $a));
    );
    ($deck: ident, 2 $a: expr) => (
        per_color!($deck, $a);
        per_color!($deck, $a);
    );
}

pub fn sorted_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(108);
    per_color!(deck, Face::Zero);
    per_color!(deck, 2 Face::One);
    per_color!(deck, 2 Face::Two);
    per_color!(deck, 2 Face::Three);
    per_color!(deck, 2 Face::Four);
    per_color!(deck, 2 Face::Five);
    per_color!(deck, 2 Face::Six);
    per_color!(deck, 2 Face::Seven);
    per_color!(deck, 2 Face::Eight);
    per_color!(deck, 2 Face::Nine);
    per_color!(deck, 2 Face::Skip);
    per_color!(deck, 2 Face::Reverse);
    per_color!(deck, 2 Face::DrawTwo);
    for _ in 0..4 {
        deck.push(Card::new(Color::Black, Face::Wild(Color::Black)));
        deck.push(Card::new(Color::Black, Face::WildDrawFour(Color::Black)));
    }
    deck
}
