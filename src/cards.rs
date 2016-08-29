use rand;
use rand::Rng;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Card {
    pub color: Color,
    pub face: Face
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Face::*;
        match self.face {
            Wild(_) | WildDrawFour(_) => write!(f, "{}", self.face),
            _ => write!(f, "{:?} {}", self.color, self.face)
        }
        //write!(f, "{:?} {:?}", self.color, self.face)
    }
}


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black
}

impl rand::Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use self::Color::*;
        let arr = [Red, Green, Blue, Yellow];
        *rng.choose(&arr).unwrap()
    }
}

impl Default for Color {
    fn default() -> Color { Color::Black }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Face {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Skip,
    Reverse,
    DrawTwo,
    Wild(Color),
    WildDrawFour(Color)
}

impl Face {
    pub fn is_wildcard(self) -> bool {
        match self {
            Face::Wild(_) | Face::WildDrawFour(_) => true,
            _ => false
        }
    }
    pub fn is_action_card(self) -> bool {
        match self {
            Face::Skip | Face::Reverse | Face::DrawTwo => true,
            _ => false
        }
    }
    pub fn is_number_card(self) -> bool {
        !(self.is_wildcard() || self.is_action_card())
    }
    pub fn wildcard_color(self) -> Color {
        if let Face::Wild(color) = self {
            color
        } else if let Face::WildDrawFour(color) = self {
            color
        } else {
            panic!("called wildcard_color on a non-wildcard!");
        }
    }
}

impl rand::Rand for Face {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use self::Face::*;
        let arr = [Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Skip, Reverse, DrawTwo, Wild(Color::Black), WildDrawFour(Color::Black)];
        *rng.choose(&arr).unwrap()
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Face::*;
        match *self {
            Wild(color) => if color == Color::Black {
                write!(f, "Wildcard")
            } else {
                write!(f, "Wildcard ({:?})", color)
            },
            WildDrawFour(color) => if color == Color::Black {
                write!(f, "Wildcard Draw Four")
            } else {
                write!(f, "Wildcard Draw Four ({:?})", color)
            },
            DrawTwo => write!(f, "Draw Two"),
            _ => write!(f, "{:?}", self)
        }
    }
}


impl Card {
    pub fn new(color: Color, face: Face) -> Self {
        match (color, face) {
            // Can't have a black non-wildcard
            (Color::Black, Face::Wild(_)) | (Color::Black, Face::WildDrawFour(_)) => {},
            (Color::Black, _) => panic!("Invalid card (black non-wildcard): {:?} {:?}", color, face),

            // Non-black wildcard
            (_, Face::Wild(_)) | (_, Face::WildDrawFour(_)) => panic!("Invalid card (non-black wildcard): {:?} {:?}", color, face),
            (_, _) => {}
        }
        Card {
            color: color,
            face: face
        }
    }
    pub fn new_hand() -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut out = Vec::with_capacity(7);
        for _ in 0..7 {
            out.push(rng.gen())
        }
        out
    }

    /// Determines if a card can be played on this card
    pub fn can_other_card_be_played_on(&self, other: &Card) -> bool {
        let colorcheck = if self.face.is_wildcard() {
            self.face.wildcard_color()
        } else {
            self.color
        };
        if other.face.is_wildcard() {
            //wildcard can be played on top of any card
            true
        } else if colorcheck == other.color || self.face == other.face {
            //same color or face
            true
        } else {
            false
        }
    }

    //proxy for card.face.is_wildcard()
    pub fn is_wildcard(&self) -> bool {
        self.face.is_wildcard()
    }

    pub fn set_wildcard_color(&mut self, color: Color) {
        use self::Face::{Wild, WildDrawFour};
        if let Wild(_) = self.face {
            self.face = Wild(color);
        } else if let WildDrawFour(_) = self.face {
            self.face = WildDrawFour(color);
        } else {
            panic!("called set_wildcard_color on a non-wildcard!");
        }
    }
}

impl rand::Rand for Card {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let face = rng.gen::<Face>();
        let color = if face.is_wildcard() {
            Color::Black
        } else {
            rng.gen()
        };
        Card::new(color, face)
    }
}

pub fn can_play(hand: &[Card], current_card: Card) -> bool {
    for card in hand {
        if current_card.can_other_card_be_played_on(card) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Face::*;
    use super::Color::*;

    #[test]
    fn it_works() {
        let card = Card::new(Blue, Nine);

    }

    #[test]
    fn card_playing_logic() {
        let lastcard = Card::new(Blue, Nine);

        // Literally the same card
        assert_eq!(lastcard.can_other_card_be_played_on(&Card::new(Blue, Nine)), true);

        // Same colour
        assert_eq!(lastcard.can_other_card_be_played_on(&Card::new(Blue, Eight)), true);

        // Same number
        assert_eq!(lastcard.can_other_card_be_played_on(&Card::new(Red, Nine)), true);

        // Wildcard
        assert_eq!(lastcard.can_other_card_be_played_on(&Card::new(Black, Wild(Green))), true);
    }

}
