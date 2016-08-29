#![allow(dead_code)]
extern crate rand;
//#[macro_use] extern crate lazy_static;
//use rand::Rng;
extern crate time;

mod ai;
mod cards;
mod deck;
mod game;
use time::Duration;
use game::Game;

fn main() {
    let mut k = Game::new();
    k.add_player();
    k.add_player();
    k.add_player();
    k.add_player();
    k.run_game();
    let timing = Duration::span(|| k.run_game());
    println!("{}ns", timing.num_nanoseconds().unwrap());
}
