pub use std::io::{Read,stdin};
use crate::logger::*;

pub struct Game {
    pub lives: i8,
    pub guessed_letters: Vec<char>,
    pub word: String,
    pub won: bool
}

impl Game {
    fn check_guess(&mut self, guess: char) {
        if self.guessed_letters.contains(&guess) {
            println!("You've already guessed that character!");
        } else {   
            let correct = self.word.to_lowercase().contains(guess);
            self.guessed_letters.push(guess);
    
            if !correct {
                self.lives -= 1;
                println!("Uh oh, you guessed wrong! You have {lives} lives left.", lives=self.lives);
            } else {
                println!("You're correct!");
            }
        }   
    }

    fn has_won(&mut self) {
        for c in self.word.to_lowercase().chars() {
            if self.guessed_letters.contains(&c) {
                continue
            } else {
                self.won = false;
                return
            }
        }
    
        self.won = true
    }
}

pub fn read_input() -> char {
    let optional: Option<char> = stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|c| c as char);

    
    return optional.unwrap().to_ascii_lowercase();
}

pub fn play(mut game: Game) {
    let mut proceed = true;

    let guess = read_input();
    if guess == '\n' {
        play(game);
        return
    }

    clear_term();

    game.check_guess(guess);

    print_hangman(game.lives);
    print_word(&game.word, &game.guessed_letters);

    game.has_won();
    if game.won {
        println!("Congratulations! You won!");
        return
    }

    if game.lives == 0 {
        clear_term();
        print_hangman(0);
        println!("Uh oh, you lost!");
        println!("Your word was {}.", game.word);
        proceed = false;
    }

    if proceed {
        play(game);
    }
}