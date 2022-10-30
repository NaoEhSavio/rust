extern crate rand;
// cargo doc --open
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let numero_secreto = rand::thread_rng().gen_range(1..100);
    println!("Advinhe o número!");
    println!("O número é: {}", numero_secreto);
    
    loop {
            
        println!("Digite o seu palpite.");
        
        let mut palpite = String::new();
        //std::io::Stdin
        io::stdin().read_line(&mut palpite).expect("Falhar ao ler a entrada");
        
        // let palpite: u32 = palpite.trim().parse().expect("Por favor, digite um número!");
        let palpite: u32 = match palpite.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número!");
                continue;
            }
        };

        println!("voce disse {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Equal => {
                println!("Você acertou!!!");
                break;
            }
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Less => println!("Muito baixo!"),
            
        }
    }
}