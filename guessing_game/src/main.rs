use rand::Rng;
use std::{cmp::Ordering, io, num::IntErrorKind};

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..=100);
    loop {
        println!("Chute um número entre 0 e 100!");
        let mut guess = String::with_capacity(21);
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => match e.kind() {
                IntErrorKind::InvalidDigit => {
                    println!("Caractere inválido.");
                    continue;
                }
                IntErrorKind::PosOverflow => {
                    println!("Inteiro grande demais, seu maluco.");
                    continue;
                }
                _ => panic!("{e}"),
            },
        };
        println!("Você chutou: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pitico."),
            Ordering::Greater => println!("Muito grande, seu guloso."),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }
}
