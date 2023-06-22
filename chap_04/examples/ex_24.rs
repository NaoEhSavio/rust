fn main() {
	let mut s = String::from("texto longo");
	let palavra = primeira_palavra(&s); // palavra vai ter o valor 5.
	s.clear(); // Isso esvazia a String, deixando ela igual a "".
	// palavra ainda tem o valor 5 aqui, mas ja nao ha ,ais uma String para 
	// qual o valor 5 faca algum sentido. palavra agora eh totalmente invalida!
}

fn primeira_palavra(s: &String) -> usize {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' '{
			return i;
		} 
	}
	s.len()
}