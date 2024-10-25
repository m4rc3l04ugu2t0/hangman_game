mod data;
mod error;

use std::io::{stdin, stdout, Write};

use colored::*;
use data::selected_rand_keyword;

pub use self::error::{GameError, Result};

struct Game {
    input_hunch: String,
    message: String,
    attempts: u8,
    keyword: (&'static str, &'static str),
    hits: Vec<String>,
}

enum GameActions {
    Win,
    Lose,
    RestartLoop,
}

impl Game {
    fn new() -> Self {
        let keyword = selected_rand_keyword();
        let hits = vec![" _".to_string(); keyword.0.len()];

        Self {
            attempts: 6,
            message: "".to_string(),
            input_hunch: String::new(),
            keyword,
            hits,
        }
    }

    fn running(&mut self) -> Result<GameActions> {
        self.draw_hangman();
        self.verify_input(self.get_input()?)?;
        self.attempts -= 1;

        if self.check_hit() {
            self.clear_scream();
            self.display_message();
            let answer = self.take_a_guess()?;
            if self.yas_no(&answer) {
                return Ok(self.attempt_word(&answer));
            }

            self.clear_scream();

            return Ok(GameActions::RestartLoop);
        }

        if self.attempts == 0 {
            return Ok(GameActions::Lose);
        }

        Ok(GameActions::RestartLoop)
    }

    fn take_a_guess(&mut self) -> Result<String> {
        self.draw_hangman();
        self.message = "Já sabe qual a palavras? Caso sim, digite a palavra ou [N]ão".to_string();
        self.display_message();
        if let Ok(i) = self.get_input() {
            return Ok(i);
        } else {
            return Err(GameError::Error("Error to read input".to_string()));
        };
    }

    fn yas_no(&self, answer: &str) -> bool {
        if answer.len() > 1 {
            return true;
        }

        false
    }

    fn check_hit(&mut self) -> bool {
        if self.keyword.0.contains(&self.input_hunch) && !self.hits.contains(&self.input_hunch) {
            self.message = "Muito bem :)".to_string();
            for (i, v) in self.keyword.0.chars().enumerate() {
                if v.to_string() == self.input_hunch {
                    self.hits[i] = v.to_string();
                }
            }

            true
        } else {
            self.message = "Você errou, tente novamanete!".to_string();
            self.clear_scream();
            self.display_message();
            false
        }
    }
    fn attempt_word(&mut self, attempt_word: &str) -> GameActions {
        if self.keyword.0 == attempt_word {
            self.message = "Tu ganhou!!! AEEEEE!".to_string();
            self.display_message();
            return GameActions::Win;
        }

        self.clear_scream();
        self.message = "Continue tentando :)".to_string();
        self.display_message();
        GameActions::RestartLoop
    }

    fn verify_input(&mut self, input: String) -> Result<()> {
        if input.len() > 1 {
            return Err(GameError::Error("Gigite somente uma letra".to_string()));
        }
        self.input_hunch = input;
        Ok(())
    }

    fn display_message(&self) {
        println!("{}", self.message);
    }

    fn draw_hangman(&mut self) {
        println!("{}", self.keyword.1);
        println!(" _____");
        println!("|     |");

        match self.attempts {
            0 => {
                println!("|");
                println!("|");
                println!("|");
                println!("|");
            }
            1 => {
                println!("|     O");
                println!("|");
                println!("|");
                println!("|");
            }
            2 => {
                println!("|     O");
                println!("|     |");
                println!("|");
                println!("|");
            }
            3 => {
                println!("|     O");
                println!("|    /|");
                println!("|");
                println!("|");
            }
            4 => {
                println!("|     O");
                println!("|    /|\\");
                println!("|");
                println!("|");
            }
            5 => {
                println!("|     O");
                println!("|    /|\\");
                println!("|    /");
                println!("|");
            }
            _ => {
                println!("|     O");
                println!("|    /|\\");
                println!("|    / \\");
                println!("|");
            }
        }

        println!("|__________             {}", self.hits.join(""));
    }

    fn get_input(&self) -> Result<String> {
        let mut input = String::new();
        stdout().flush().ok();
        stdin()
            .read_line(&mut input)
            .map_err(|_| GameError::Error("Erro ao ler entrada".to_string()))?;
        Ok(input.trim().to_lowercase())
    }

    fn clear_scream(&self) {
        print!("{esc}c", esc = 27 as char);
    }
}

fn main() {
    let mut game = Game::new();
    game.clear_scream();

    loop {
        match game.running() {
            Ok(e) => match e {
                GameActions::Win => break,
                GameActions::Lose => {
                    game.clear_scream();
                    game.draw_hangman();
                    game.message = "Você perdeu :(".to_string();
                    game.display_message();
                    break;
                }
                GameActions::RestartLoop => {
                    //
                }
            },
            Err(err) => match err {
                GameError::Error(e) => {
                    println!("{}", e.red());
                }
            },
        }
    }
}
