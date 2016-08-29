use cards::*;
pub mod clive;

#[derive(Default)]
pub struct AIHelper {
    blue: u8,
    red: u8,
    green: u8,
    yellow: u8,
    wild: u8,
    wilddraw: u8,
    playable: Vec<usize>,
    most: Color
}

pub fn generate_helper(cards: &Vec<Card>, lastcard: Card) -> AIHelper {
    let mut help = AIHelper::default();
    for i in 0..cards.len() {
        let ref card = cards[i];
        match card.color {
            Color::Blue => help.blue += 1,
            Color::Red => help.red += 1,
            Color::Green => help.green += 1,
            Color::Yellow => help.yellow += 1,
            Color::Black => match card.face {
                                Face::Wild(_) => help.wild += 1,
                                Face::WildDrawFour(_) => help.wilddraw += 1,
                                _ => {}
                            }
        }
        if lastcard.can_other_card_be_played_on(&card) {
            help.playable.push(i);
        }
    }

    if help.green > help.blue && help.green > help.yellow && help.green > help.red {
        help.most = Color::Green;
    } else if help.blue > help.yellow && help.blue > help.red && help.blue > help.green {
        help.most = Color::Blue;
    } else if help.yellow > help.blue && help.yellow > help.red {
        help.most = Color::Yellow
    } else {
        help.most = Color::Red;
    }

    println!("  Helper (R{}, G{}, B{}, Y{}, W{}, WD{}, most: {:?})", help.red, help.green, help.blue, help.yellow, help.wild, help.wilddraw, help.most);
    for i in help.playable.as_slice() {
        println!("         - {}", cards[*i]);
    }
    help
}
