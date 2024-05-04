use std::io::{self, Write};

use rand::seq::SliceRandom;

fn selected_rand_keyword() -> &'static str {
    let keywords = ["Luffy", "Comida", "Programação"];
    let keyword = keywords.choose(&mut rand::thread_rng()).unwrap_or(&"Isso não vai da error");
    keyword
}

fn check_attempt(attempt: &mut String, keyword: &str, hint: &mut Vec<String>) -> bool {
    let replace_char = attempt.trim().chars().next().expect("Error: Replace char");

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

fn test() {
    println!("Slaaaa");
}

fn main() {
    let mut input_user = String::new();
    let mut hits: Vec<String> = Vec::new();
    let mut attempts = 0;
    let keyword = selected_rand_keyword();
    let mut message = String::from("Digite uma letra.");


    for _ in 0..keyword.chars().count() {
        hits.push(" _".to_string());
    }

    loop {
        clear_scream();
        println!("Jogo da forca, descubra a palavra pela sua vida!!!");
        println!("{}", hits.join(""));

        println!("{}", message);
        input_user.clear();
        io::stdout().flush().expect("Error clear buffer");

        io::stdin().read_line(&mut input_user).expect("Invalid input");

        if attempts == 5 {
            println!("Você perdeu!!!");
            break;
        }

        if input_user.trim().len() > 1 {
            message.clear();
            message.push_str("Digite apenas uma letra!!!");
            attempts -= 1;
            continue;
        }

        println!("{}", keyword);
        
        let checked_attempt = check_attempt(&mut input_user, &keyword, &mut hits);

        if checked_attempt {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Invalid input");
            
            if keyword.eq(guess.trim()) {
                test();
                println!("Você acertou!!!");
                break;
            }
            message.push_str("Você errou, tente novamente.");
        }
        
    }
}