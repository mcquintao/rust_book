extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Mensagem de início do jogo
    println!("Advinhe o número");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    //println!("O número secreto é: {}", numero_secreto);

    loop {
        println!("Digite seu palpite");

        // Craidno uma variável mutável
        let mut palpite = String::new();

        // Lendo entrada do usuário
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite apenas números inteiros de 1 a 100!");
                continue;
            },
        };

        let x = 5;
        let y = 10;

        println!("x = {x}, y = {y}");
        println!("x = {}, y = {}", x, y);

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Você acertou");
                break;
            }
        }
    }
}
