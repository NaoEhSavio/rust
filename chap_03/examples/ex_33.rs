fn main() {
	let numero = 6;
	if numero % 4 == 0 {
		println!("Número é divisível por 4");
	} else if numero % 3 == 0 {
		println!("Número é divisível por 3");
	} else if numero % 2 == 0 {
		println!("Número é divisível por 2");
	} else {
		println!("Número não é divisível por 4, 3 ou 2");
	}
}