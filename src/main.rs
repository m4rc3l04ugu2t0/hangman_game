mod error;

use std::io::{stdin, stdout, Write};

use rand::{seq::SliceRandom, thread_rng};

use colored::*;

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
        Self {
            attempts: 6,
            message: "SLDLDLDL".to_string(),
            input_hunch: String::new(),
            keyword: ("", ""),
            hits: Vec::new(),
        }
    }

    fn running(&mut self) -> Result<GameActions> {
        self.draw_hangman();
        self.verify_input(self.get_input()?)?;
        self.attempts -= 1;

        if self.verify_attempt() {
            self.clear_scream();
            self.show_message();
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

    fn set_up(&mut self) {
        self.selected_rand_keyword();
        self.hidden_keyword();
    }

    fn update(&mut self) {
        self.message = "".to_string();
    }

    fn take_a_guess(&mut self) -> Result<String> {
        self.draw_hangman();
        self.message = "Já sabe qual a palavras? Caso sim, digite a palavra ou [N]ão".to_string();
        self.show_message();
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

    fn verify_attempt(&mut self) -> bool {
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
            self.show_message();
            false
        }
    }
    fn attempt_word(&mut self, attempt_word: &str) -> GameActions {
        if self.keyword.0 == attempt_word {
            self.message = "Tu ganhou!!! AEEEEE!".to_string();
            self.show_message();
            return GameActions::Win;
        }

        self.clear_scream();
        self.message = "Continue tentando :)".to_string();
        self.show_message();
        GameActions::RestartLoop
    }

    fn verify_input(&mut self, input: String) -> Result<()> {
        if input.len() > 1 {
            return Err(GameError::Error("Gigite somente uma letra".to_string()));
        }
        self.input_hunch = input.trim().to_ascii_lowercase();
        Ok(())
    }

    fn show_message(&self) {
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

    fn hidden_keyword(&mut self) {
        for _ in 0..self.keyword.0.len() {
            self.hits.push(" _".to_string());
        }
    }

    fn selected_rand_keyword(&mut self) {
        let words_and_hints = vec![
            (
                "computador",
                "É uma máquina usada para processar informações.",
            ),
            ("bicicleta", "Um veículo de duas rodas movido a pedal."),
            ("elefante", "É um animal grande com tromba."),
            ("montanha", "Uma formação geológica elevada."),
            ("floresta", "Um lugar com muitas árvores e vida selvagem."),
            ("oceano", "Corpo de água que cobre a maior parte da Terra."),
            (
                "futebol",
                "Esporte popular jogado com uma bola e dois gols.",
            ),
            ("astronauta", "Pessoa que viaja pelo espaço."),
            (
                "universo",
                "Tudo o que existe, incluindo galáxias e estrelas.",
            ),
            ("planeta", "Corpo celeste que orbita uma estrela."),
            ("satélite", "Objeto que orbita um planeta ou estrela."),
            ("estrela", "Corpo celeste que brilha no céu noturno."),
            ("foguete", "Veículo usado para viagens espaciais."),
            ("amizade", "Relação afetiva entre pessoas."),
            ("chocolate", "Doce feito a partir do cacau."),
            (
                "relâmpago",
                "Fenômeno elétrico visível durante tempestades.",
            ),
            ("vulcão", "Estrutura geológica que pode liberar lava."),
            ("jardim", "Local onde plantas e flores são cultivadas."),
            ("dinossauro", "Animal pré-histórico extinto."),
            ("arqueologia", "Estudo de sociedades antigas."),
            ("pirâmide", "Estrutura histórica encontrada no Egito."),
            ("continente", "Uma das grandes massas de terra do planeta."),
            ("civilização", "Sociedade humana com cultura desenvolvida."),
            ("maratona", "Corrida de longa distância."),
            ("esquiar", "Deslizar na neve usando equipamentos."),
            ("glaciar", "Grande massa de gelo em movimento."),
            ("baleia", "Grande mamífero que vive no oceano."),
            ("máquina", "Equipamento projetado para realizar tarefas."),
            (
                "jornalista",
                "Pessoa que trabalha com notícias e informações.",
            ),
            (
                "paralelepipedo",
                "Pedra de calçamento com forma geométrica.",
            ),
            ("biblioteca", "Lugar onde são guardados muitos livros."),
            ("borboleta", "Inseto com asas coloridas."),
            ("parafuso", "Peça metálica usada para fixação."),
            ("garrafa", "Recipiente para armazenar líquidos."),
        ];

        let selected_word_and_hint = words_and_hints.choose(&mut thread_rng()).unwrap();
        self.keyword = *selected_word_and_hint;
    }

    fn get_input(&self) -> Result<String> {
        let mut input = String::new();

        if let Err(e) = stdout().flush() {
            return Err(GameError::Error(e.to_string()));
        }

        if stdin().read_line(&mut input).is_ok() {
            Ok(input.trim().to_ascii_lowercase())
        } else {
            Err(GameError::Error("Invalid input".to_string()))
        }
    }
    fn clear_scream(&self) {
        print!("{esc}c", esc = 27 as char);
    }
}

fn main() {
    let mut game = Game::new();
    game.clear_scream();
    game.set_up();

    loop {
        match game.running() {
            Ok(e) => match e {
                GameActions::Win => break,
                GameActions::Lose => {
                    game.clear_scream();
                    game.update();
                    game.draw_hangman();
                    game.message = "Você perdeu :(".to_string();
                    game.show_message();
                    break;
                }
                GameActions::RestartLoop => {
                    game.update();
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
