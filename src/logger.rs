pub fn print_word(word: &String, guesses: &Vec<char>) {
    for c in word.chars().map(|c| c.to_ascii_lowercase()) {
        if guesses.contains(&c) {
            print!("{}", c);
        } else {
            print!("_")
        }
    }

    println!();
}

pub fn print_hangman(lives: i8) {
    match lives {
        0 => {
            println!("|---|");
            println!("|   |");
            println!("|   O");
            println!("|  /|\\");
            println!("|  / \\")
        }

        1 => {
            println!("|---|");
            println!("|");
            println!("|   O");
            println!("|  /|\\");
            println!("|  / \\")
        }

        2 => {
            println!("|---|");
            println!("|");
            println!("|   O");
            println!("|  /|\\");
            println!("|    \\")
        }

        3 => {
            println!("|---|");
            println!("|");
            println!("|   O");
            println!("|  /|\\");
            println!("|")
        }

        4 => {
            println!("|---|");
            println!("|");
            println!("|   O");
            println!("|   |\\");
            println!("|")
        }

        5 => {
            println!("|---|");
            println!("|");
            println!("|   O");
            println!("|   |");
            println!("|")
        }

        6 => {
            println!("|---|");
            println!("|");
            println!("|");
            println!("|");
            println!("|")
        }

        _ => return
    }
}

pub(crate) fn clear_term() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{}[2J", 27 as char);
}