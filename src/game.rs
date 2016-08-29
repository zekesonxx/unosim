
use cards::*;
use rand;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>
}

pub struct Game {
    deck: Vec<Card>,
    discard: Vec<Card>,
    players: Vec<Player>
}


impl Game {
    pub fn new() -> Self {
        Game {
            deck: super::deck::sorted_deck(),
            discard: Vec::with_capacity(108),
            players: vec![]
        }
    }
    pub fn add_player(&mut self) {
        self.players.push(Player {
            hand: vec![]
        });
    }
    pub fn check_win(players: &Vec<Player>) -> bool {
        for player in players {
            if player.hand.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn draw_card(&mut self) -> Card {
        if let Some(card) = self.deck.pop() {
            return card;
        } else {
            //deck is empty, shuffle the discard pile
            println!("  Deck empty, shuffing discard pile");
            self.deck = self.discard.clone();
            self.discard = vec![];
            return self.draw_card();
        }
    }

    pub fn shuffle_deck(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(self.deck.as_mut_slice());
    }

    pub fn run_game(&mut self) {
        let playercount = self.players.len().clone();
        println!("Staring Uno Game with {} players", self.players.len());
        self.shuffle_deck();
        for i in 0..playercount {
            for _ in 0..7 {
                let newcard = self.draw_card();
                self.players[i].hand.push(newcard);
            }
        }
        let mut lastcard = self.draw_card();
        while lastcard.is_wildcard() {
            //if we get a wildcard, mix it back in and redraw.
            self.deck.push(lastcard);
            self.shuffle_deck();
            lastcard = self.draw_card();
        }
        println!("Starting card: {}", lastcard);

        // Enter the main game loop
        let mut players = self.players.clone();
        let mut names = ["Jane", "Alberta", "Xavier", "Pop", "Ruby"];
        rand::thread_rng().shuffle(&mut names);
        let mut current_player: usize = 0;
        let mut going_forward = true;
        macro_rules! iterate {
            () => (
                if going_forward {
                    if current_player == players.len()-1 {
                        current_player = 0;
                    } else {
                        current_player += 1;
                    }
                } else {
                    if current_player == 0 {
                        current_player = players.len()-1;
                    } else {
                        current_player -= 1;
                    }
                }
            );
        }
        macro_rules! header {
            () => (
                println!("{}'s turn (P{}, {} cards)", names[current_player], current_player+1, players[current_player].hand.len());
            );
        }
        loop {
            header!();
            let mut card_was_played = false;
            if can_play(&players[current_player].hand, lastcard) {
                // Current player can play, process their play
                let played = super::ai::clive::play(&mut players, current_player, lastcard);
                self.discard.push(lastcard);
                lastcard = played;
                card_was_played = true;
            } else {
                //Current player can't play, draw a card.
                let card = self.draw_card();
                println!("  Card drawn.");
                if lastcard.can_other_card_be_played_on(&card) {
                    // Got a playable card, add it to their hand
                    players[current_player].hand.push(card);
                    // and rerun the AI
                    // (so that it can decide on a wildcard color if needed)
                    let played = super::ai::clive::play(&mut players, current_player, lastcard);
                    self.discard.push(lastcard);
                    lastcard = played;
                    card_was_played = true;
                } else {
                    println!("  Couldn't play a card.");
                    // Not a playable card
                    players[current_player].hand.push(card);
                }
            }

            /* */
            if card_was_played {
                println!("  Played card: {}", lastcard);
                match lastcard.face {
                    Face::Reverse => going_forward = !going_forward,
                    Face::DrawTwo => {
                        iterate!();
                        header!();
                        println!("  Drawing 2 cards");
                        players[current_player].hand.push(self.draw_card());
                        players[current_player].hand.push(self.draw_card());
                    },
                    Face::Skip => {
                        iterate!();
                        header!();
                        println!("  Turn Skipped");
                    },
                    Face::WildDrawFour(_) => {
                        iterate!();
                        header!();
                        println!("  Drawing 4 cards");
                        players[current_player].hand.push(self.draw_card());
                        players[current_player].hand.push(self.draw_card());
                        players[current_player].hand.push(self.draw_card());
                        players[current_player].hand.push(self.draw_card());
                    },
                    _ => {}
                }
            }

            iterate!();
            if Game::check_win(&players) {
                println!("A player won!");
                break;
            }
        }

    }
}
