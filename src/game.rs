use std::fs::File;
use std::io::prelude::*;
use rand::Rng;


pub struct Game {
    word: Vec<Letter>,
    guesses: u8
}

impl Game {
    pub fn get_guesses(&self) -> u8 {
        self.guesses
    }

    pub fn is_won(&self) -> bool {
        let mut victory = true;
        for letter in self.word.iter() {
            if !letter.revealed {
                victory = false;
            }
        };
        victory
    }

    pub fn get_word(&self) -> String {
        let mut word_str = String::new();

        for letter in self.word.iter() {
            word_str.push(letter.character);
        };

        word_str
    }

    pub fn show_progress(&self) {
        let mut progress = String::from("Progress: ");
        for letter in self.word.iter() {
            progress.push(letter.show());
            progress.push(' ');
        }

        println!("{}", progress);
    }

    pub fn accept_guess(&mut self){
        let guess = get_user_input();

        let mut correct_guess = false;
        for letter in self.word.iter_mut() {
            if letter.character == guess {
                letter.reveal();
                correct_guess = true;
            }
        }
        if !correct_guess { self.guesses -= 1; }
    }
}

pub fn new() -> Game {
   
    Game {  
        word: generate_letters(select_word()), 
        guesses: 6
    }
}

struct Letter {
    character: char,
    revealed: bool
}

impl Letter {
    fn new (c:char) -> Letter {
        Letter {
            character:c,
            revealed:false
        }
    }
    fn show(&self) -> char {
        if self.revealed {
            self.character
        }
        else {
            '-'
        }
    }
    fn reveal(&mut self){
        self.revealed = true;
    }
}

fn select_word() -> String {
    let mut file = File::open("words.txt")
        .expect("Error trying to open file words.txt");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Error trying to parse the file contents into a string");
    
    let words: Vec<&str> = file_contents.lines().collect();

    String::from(words[rand::thread_rng().gen_range(0, words.len())])

}

fn generate_letters(word:String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter::new(c));
    };

    letters
}

fn get_user_input() -> char {

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error reading user input");

    user_input.chars().next()
        .expect("Error getting character from user inputed string")
}
