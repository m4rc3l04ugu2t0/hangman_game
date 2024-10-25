use rand::{seq::SliceRandom, thread_rng};

pub fn selected_rand_keyword() -> (&'static str, &'static str) {
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

    *words_and_hints.choose(&mut thread_rng()).unwrap()
}
