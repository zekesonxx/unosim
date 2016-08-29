//! Clive Uno AI
//! Plays in a "standard" way
use rand;
use rand::Rng;
use cards::*;
use super::super::game::Player;
#[allow(unused_variables)]
pub fn play(players: &mut Vec<Player>, me: usize, lastcard: Card) -> Card {
    let help = super::generate_helper(&players[me].hand, lastcard);
    let mut rng = rand::thread_rng();
    let index = *(rng.choose(help.playable.as_slice()).unwrap());
    if players[me].hand[index].is_wildcard() {
        players[me].hand[index].set_wildcard_color(help.most);
    }
    players[me].hand.remove(index)
}
