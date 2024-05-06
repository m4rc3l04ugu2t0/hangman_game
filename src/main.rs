use std::io::{self, Write};

use rand::seq::SliceRandom;

enum ActionsGame {
    YouLose,
    RestartLoop,
    YouWin,
}

fn selected_rand_keyword() -> (&'static str, &'static str) {
    let words_and_hints = vec![
        ("banana", "É uma fruta amarela."),
        ("gato", "É um animal de estimação com quatro patas."),
        ("carro", "É um meio de transporte com rodas."),
        // Adicione mais palavras e dicas conforme necessário
    ];
    let selected_word_and_hint = words_and_hints.choose(&mut rand::thread_rng()).unwrap();
    *selected_word_and_hint
}

fn check_attempt(attempt: &str, keyword: &str, hint: &mut [String]) -> bool {
    if attempt.trim().is_empty() {
        return false;
    }

    let replace_char = match attempt.trim().chars().next() {
      Some(char) => char,
      None => return false, 
    };

    if keyword.contains(attempt.trim()) {
        for (i, word) in keyword.chars().enumerate() {
            if word == replace_char {
                hint[i] = attempt.trim().to_string()
            }
        }
        return true;
    }
    false
}

fn clear_scream() {
    print!("{esc}c", esc = 27 as char);
}

fn manager_actions_game(input_user: &str, keyword: &str, message: &mut String, hits: &[String], check: bool) -> ActionsGame {
        println!("{}", hits.join(""));

        println!("{}", message);

        if input_user.trim().len() > 1 {
            message.clear();
            message.push_str("Digite apenas uma letra!!!");
            return ActionsGame::RestartLoop;
        }

        if check {
            println!("Deseja tenta acerta a palavra? Digite `y` para sim, qualquer tecla para não.");
            let mut guess = String::new();

            io::stdout().flush().expect("Error clear buffer");
            io::stdin().read_line(&mut guess).expect("Invalid input");

            if guess.trim().to_lowercase() != "y" {
                return ActionsGame::RestartLoop;
            } 

            println!("Digite a palavra que você acha que é.");
            guess.clear();
            io::stdin().read_line(&mut guess).expect("Invalid input");

            if keyword == guess.trim().to_lowercase() {
                message.clear();
                message.push_str("Você vanceu!!!");
                return ActionsGame::YouWin;
            }
            message.clear();
            message.push_str("Palavra incorreta. Você perdeu");
            return ActionsGame::YouLose;
        }
    
    message.clear();
    message.push_str("Tente novamente");
    ActionsGame::RestartLoop
}

fn draw_hangman(tries: usize) {
    println!(" _____");
    println!("|     |");

    match tries {
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

    println!("|__________");
}

fn main() {
    let mut input_user = String::new();
    let (keyword, hint) = selected_rand_keyword();
    let mut hits: Vec<String> = Vec::new();
    let mut message = String::from("Digite uma letra.");
    let mut attempt = 6;

    for _ in 0..keyword.len() {
        hits.push(" _".to_string());
    }

    loop {
        clear_scream();
        println!("Jogo da forca, descubra a palavra pela sua vida!!!\n\
        Você tem direito a uma unica tentativa de tenta acerta a palavra\n\
         e 5 tentativas de tenta acerta uma letra.\n");

        println!("Dica: {}\n", hint);

        println!("Tentativas restantes: {}", attempt);

        draw_hangman(attempt);
        attempt -= 1;

        if attempt == 0 {
            println!("Você Perdeu!!");
            break;
        }

        input_user.clear();
        io::stdout().flush().expect("Error clear buffer");

        println!("{}", hits.join(""));
        println!("{}", message);

        io::stdin().read_line(&mut input_user).expect("Invalid input");

        let checked_attempt = check_attempt(&input_user, keyword, &mut hits);

        match manager_actions_game(&input_user,keyword, &mut message, &hits, checked_attempt) {
            ActionsGame::YouWin => {
                println!("Você Venceu");
                break;
            },
            ActionsGame::YouLose => {
                break;
            },
            ActionsGame::RestartLoop => {
                continue;
            },
        }
     }
}