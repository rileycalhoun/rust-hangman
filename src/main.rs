
pub mod logger;
pub mod game;
pub mod word;

use crate::logger::{print_word,clear_term};
use crate::game::{play,Game};
use crate::word::select_word;

fn main() {
    let lives: i8 = 7;
    let word: String = select_word();

    clear_term();
    println!("Welcome to Riley's Game of Hangman!");
    println!("Your word is {word} letters long, and you have {lives} lives!", word=word.len(), lives=lives);
    println!("To start, type your guess into the terminal window!");
    print_word(&word, &Vec::new());

    let game = Game {
        lives: 7,
        guessed_letters: Vec::new(),
        word: select_word(),
        won: false
    };

    play(game);
}