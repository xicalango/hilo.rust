use std::io;
use std::str;
use std::cmp::Ordering;

extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct GameConfiguration {
    min: u32,
    max: u32
}

#[derive(Debug)]
struct GameState<'a> {
    config: &'a GameConfiguration,
    num_tries: u32,
    number: u32,
    playing: bool
}

impl <'a>GameState<'a> {
    fn from_config(config: &'a GameConfiguration) -> GameState<'a> {
        let number = GameState::generate_number(config, &mut rand::thread_rng());
        GameState { config: config, num_tries: 0, number: number, playing: true }
    }

    fn generate_number<R: Rng>(config: &GameConfiguration, rng: &mut R) -> u32 {
        Range::new(config.min, config.max).ind_sample(rng)
    }
}

impl GameConfiguration {
    
    fn read_from_stdin() -> GameConfiguration {
        let min_nr: u32 = read_user_input("min number: ");
        let max_nr: u32 = read_user_input("max number: ");

        GameConfiguration { min: min_nr, max: max_nr }
    }
}

fn read_user_input<T: str::FromStr>(prompt: &'static str) -> T {
    let reader = &mut io::stdin();
    let buf = &mut String::new();

    println!("{}", prompt);
    reader.read_line(buf).ok().expect("Reading from terminal failed.");

    buf.trim().parse::<T>().ok().expect("Invalid input format.")
}

fn play(state: &mut GameState) {
    while state.playing {
        state.num_tries += 1;
        let guess:u32 = read_user_input("guess: ");

        let result = guess.cmp(&state.number);

        match result {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct!");
                state.playing = false;
            }
        }
    }
}

fn main() {
    let config = GameConfiguration::read_from_stdin();
    let game_state = &mut GameState::from_config(&config);
    
    play(game_state);
}
