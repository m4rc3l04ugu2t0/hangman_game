use std::io;

use rand::seq::SliceRandom;

fn selected_rand_keyword() -> &'static str {
    let keywords = ["Luffy", "Comida", "Programação"];
    let keyword = keywords.choose(&mut rand::thread_rng()).unwrap_or(&"Isso não vai da error");
    keyword
}

fn main() {
    let mut input_user = String::new();

    println!("Jogo da forca, descubra a palavra pela sua vida!!!");

    loop {
        let keyword = selected_rand_keyword();
        io::stdin().read_line(&mut input_user).expect("Invalid input");
        println!("{}", keyword);
        if input_user.contains("sair") {
            break;
        }
    }
}