use std::io;

fn main() {
    println!("Gerar o n-ésimo número de Fibonacci!");
    println!("Por favor, digite um número!");
    loop {
        let mut number = String::new();
        //std::io::Stdin
        io::stdin().read_line(&mut number).expect("Falhar ao ler a entrada");
        
        // let palpite: u32 = palpite.trim().parse().expect("Por favor, digite um número!");
        let number: u64 = match number.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número!");
                continue;
            }
        };
        
    let fib = Fibonacci(number);

    println!("Número de Fibonacci é {}",fib);
    break;
    }
}
fn Fibonacci (n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        Fibonacci(n - 1) + Fibonacci(n - 2)
    }
}