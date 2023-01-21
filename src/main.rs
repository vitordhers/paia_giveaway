use owo_colors::OwoColorize;
use owo_colors::Rgb;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use std::{thread, time};

fn main() {
    let color_library = get_color_library();
    clear_console();
    println!("Paia shuffler v1.0 - by dinaiscoding@cannislabs!");
    pause();
    run_disclaimer(color_library.texas_buying_club_color);
    let dina_phone = run_description(&color_library, None);
    pause();
    log_paia_prices(&color_library, dina_phone);
    let mut participants = log_participants();
    loop {
        let winner_result = suffle_participants(&mut participants);
        match winner_result {
            Some(winner) => {
                announce_winner(winner, &color_library);
                break;
            }
            _ => {
                continue;
            }
        }
    }
    outro()
}

fn get_color_library() -> ColorLibrary {
    let color_library = ColorLibrary {
        texas_buying_club_color: Rgb(135, 62, 35),
        paulistinha_tradicional: Rgb(226, 135, 67),
        paulistinha_menta: Rgb(0, 255, 149),
        paulistinha_ouro: Rgb(191, 188, 0),
        phone_number: Rgb(150, 190, 37),
        pix_color: Rgb(46, 189, 175),
    };
    color_library
}

fn clear_console() {
    std::process::Command::new("clear").status().unwrap();
}

fn pause() {
    let mut stdout = stdout();
    stdout
        .write(
            br#"
    Aperte enter para continuar..."#,
        )
        .unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn run_disclaimer(color: Rgb) {
    println!(
        "Esse sorteiro é um oferecimento do {}!",
        "Clube de Compras Texas".color(color)
    );
}

fn run_description<'a>(color_library: &ColorLibrary, phone: Option<&'a str>) -> String {
    let mut phone_number = String::new();

    match phone {
        None => {
            println!("Phone do Dina:");

            io::stdin()
                .read_line(&mut phone_number)
                .expect("Insira um número válido!");
        }
        Some(loaded_phone) => {
            phone_number = String::from(loaded_phone);
        }
    }

    println!(
        r#"
        O {} é um grupo de compras coletivas de paieiros por nós esalqueanos, SEM fins lucrativos 💸.
        Toda semana o Dina 🧑 chama o fornecedor para que compremos em conjunto as paias da semana. 🚀
        Como o fornecedor só vende box 📦 de 10 paias 🚬, faz sentido nos juntarmos pra comprar juntos. 💪
        As paias geralmente vêm de segunda-feira, pelas 17h30 🕠.
        Até lá, cabe a você fazer o pedido pelo grupo de Whatsapp 🗨 de quantas paias vai querer, mandar o {} 💠 e o comprovante 🧾 até às 16h00 🕓 para o Dina 🧑.
        Mande mensagem 💬 para

        {}
        pedindo pra entrar no grupo! 🔥🔥🔥 "#,
        "Clube de Compras Texas".color(color_library.texas_buying_club_color),
        "Pix".color(color_library.pix_color),
        phone_number.color(color_library.phone_number).bold()
    );
    phone_number
}

fn log_paia_prices(color_library: &ColorLibrary, phone: String) {
    let mut displayed_items: [bool; 3] = [false, false, false];
    clear_console();
    loop {
        run_description(color_library, Some(&phone));
        println!("Qual o preço 💰 dos paias 🚬, você se pergunta? Apoi, escolha aí 🤔!");
        println!(
            r#"
            1 - 🟤 {} 
            2 - 🌿 {}
            3 - 🔸 {}"#,
            "Tradicional".color(color_library.paulistinha_tradicional),
            "Menta".color(color_library.paulistinha_menta),
            "Ouro".color(color_library.paulistinha_ouro),
        );

        if !displayed_items.contains(&false) {
            println!(
                r#"            4 - 🤬 CHEGA DE GRACINHA, PORRA, SORTEIA LOGO O CARALHO DO MEU CIGARRO, FDPTA!
                "#
            )
        }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Insira um número válido!");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                clear_console();
                println!(
                    r#"        Paia 🟤 {} - R$ 16,50 cada
                    "#,
                    "Tradicional".color(color_library.paulistinha_tradicional)
                );
                displayed_items[0] = true;
            }
            2 => {
                clear_console();
                println!(
                    r#"        Paia 🌿 {} - R$ 18,70 cada"#,
                    "Menta".color(color_library.paulistinha_menta)
                );
                displayed_items[1] = true;
            }
            3 => {
                clear_console();
                println!(
                    r#"        Paia 🔸 {} - R$ 19,80 cada"#,
                    "Ouro".color(color_library.paulistinha_ouro)
                );
                displayed_items[2] = true;
            }
            4 => {
                clear_console();
                if !displayed_items.contains(&false) {
                    println!(
                        r#"
                    Beleza, 🆒zão, boa sorte aí pros participante 🍀🍀🍀
                    
                    "#
                    );
                    break;
                } else {
                    println!("Escolha um número válido KRL 😤");
                    continue;
                }
            }
            _ => {
                clear_console();
                println!("Escolha um número válido KRL 😤");
                continue;
            }
        }
    }
}

fn log_participants() -> Vec<String> {
    let stdin = io::stdin();
    let mut data = String::new();
    println!("Dina, pra nego não ficar de gracinha que houve patifaria, faz esse bem pra mim, cara, cola o JÊIZO (JSON) dos tokens participantes: ");

    stdin
        .read_line(&mut data)
        .expect("Insira um número válido!");
    // let json: Vec<ParticipantToken> = serde_json::from_str(&data)?;

    let participants: Vec<String> = serde_json::from_str(&data).unwrap();
    participants
}

fn suffle_participants(participants: &mut Vec<String>) -> Option<&String> {
    clear_console();
    println!(
        r#"TOP 🆒ZÃO, aki tá essas putaria de JSON: 
            {:?}
        "#,
        participants
    );

    loop {
        println!(
            r#"IAI, BOY, ki ki vai ser?
            1 - 🃏 Baraia os participante
            2 - 🎲 SORTEIA ESSE KRL LOGO"#
        );
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Insira um número válido!");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                clear_console();
                println!("🃏♣♥♠♦ BARAIA OS PARTICIPANTE, FI! ♦♠♥♣🃏");
                participants.shuffle(&mut thread_rng());
                println!(
                    r#"TOP 🆒ZÃO, aki tá essas putaria de JSON: 
                        {:?}
                    "#,
                    participants
                );

                continue;
            }
            2 => {
                clear_console();
                println!("👀 👀 👀 OLHO NO LANCE...!");

                let five_secs = time::Duration::from_secs(5);
                thread::sleep(five_secs);


                let winner = participants.choose(&mut rand::thread_rng());
                return winner;
            }
            _ => {
                println!("Escolha um número válido KRL 😤")
            }
        }
    }
}

fn announce_winner(winner: &String, color_library: &ColorLibrary) {
    print!(
        r#"
        O TOKEN VENCEDOR FOI ✨ {} ✨ !!!
        🎊 🎉 PARABÉNS 🎈🎈🎈 SEU(UA) CORNO(A) 🐃, VENHA BUSCAR SEUS CIGARROS QUANDO DER.
        "#,
        winner.color(color_library.paulistinha_ouro).bold()
    )
}

fn outro() {
    println!(
        r#"
        🙏 OBRIGADO A TODOS, QUEM NÃO PARTICIPOU, VÁ COÇAR O 🆒 COM UM GARFO 🍴!
        
        PAULISTINHA, FAZ UM PAIEIRO MENOS EMPENADO, PLMDDS
    "#
    );
}

struct ColorLibrary {
    texas_buying_club_color: Rgb,
    paulistinha_tradicional: Rgb,
    paulistinha_menta: Rgb,
    paulistinha_ouro: Rgb,
    phone_number: Rgb,
    pix_color: Rgb,
}
