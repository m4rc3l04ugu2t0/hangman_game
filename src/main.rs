use rand::seq::SliceRandom;
use std::io;

struct GameInfor {
    guess_word: String,
    user_input: String,
    yes_no: bool,
    message: String,
    words_and_hints: Vec<(&'static str, &'static str)>,
}

impl GameInfor {
    fn infors() -> GameInfor {
        let mut game_infors = GameInfor {
            guess_word: String::new(),
            user_input: String::new(),
            yes_no: false,
            message: String::new(),
            words_and_hints: vec![
                ("banana", "É uma fruta amarela."),
                ("gato", "É um animal de estimação com quatro patas."),
                ("carro", "É um meio de transporte com rodas."),
            ],
        };
        game_infors
    }

    fn get_random_word(&self) -> (&str, &str) {
        *self
            .words_and_hints
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

fn main() {
    let keyword = String::from("Sardinha");
    // let s_len = keyword.len();
    let mut word: Vec<String> = Vec::new();
    let mut input = String::new();
    let mut attempts = 5;
    let mut yes_no = String::new();
    let mut sera = String::new();
    let mut alert = String::from("Dite uma letra");
    let game = GameInfor::infors();

    let (wordk, hint) = game.get_random_word();

    for _ in 0..wordk.len() {
        word.push("_ ".to_string());
    }

    loop {
        clearscreen::clear().expect("failed to clear screen");
        println!("Dica: {}", hint);
        println!("Tentativas restantes: {}", attempts);
        println!("{}", word.join(""));
        input.clear();
        if yes_no.trim().contains('y') {
            println!("De seu palpite");
            io::stdin()
                .read_line(&mut sera)
                .expect("Error: Inavlid input");
            if sera.trim() == keyword {
                println!("Ganhou");
                break;
            }
            println!("Errado");
        }
        yes_no.clear();
        println!("{}", alert);
        io::stdin().read_line(&mut input).expect("Ai que saco");

        let char_to_replace = input.trim().chars().next().expect("AAAA");
        // let input = input.trim();

        if keyword.contains(input.trim()) {
            println!("Boa");
            for (index, char) in keyword.chars().enumerate() {
                if char == char_to_replace {
                    word[index] = char_to_replace.to_string();
                };
            }
            println!("{}", word.join(""));
            println!("Deseja tenta adivinha a palavra? y para SIM, qualquer outra tecla para não");
            io::stdin()
                .read_line(&mut yes_no)
                .expect("Error: Inavlid input");
            attempts -= 1;
        } else if attempts == 0 {
            println!("Perdeu!!!");
            break;
        } else {
            alert.clear();
            alert.push_str("Tente novamente");
            attempts -= 1;
        }
    }
}
