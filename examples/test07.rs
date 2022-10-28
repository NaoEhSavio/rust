use rand::Rng; // 0.8.5
use std::cmp::Ordering;
use std::io;

fn main() {
    let numero = rand::thread_rng().gen_range(1..11);
    println!("O número é : {}", numero);

    loop {
        println!("Digiti um número entre 1 e 10: ");

        let mut dig = String::new();

        io::stdin()
            .read_line(&mut dig)
            // .unwrap();
            .expect("Falhar ao ler a linha");

        println!("voce Digitou: {}", dig);

        let dig: u64 = dig.trim().parse().expect("Por favor digiti um número");

        match dig.cmp(&numero) {
            Ordering::Less => println!("É Menor"),
            Ordering::Greater => println!("É Maior"),
            Ordering::Equal => {
                println!("Voce acertou!!!");
                break;
            }
        }
    }
}
