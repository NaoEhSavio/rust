fn main() {
	let mut s = String::from("texto longo");
	let palavra = primeira_palavra(&s);
	s.clear(); //Erro!
}
fn primeira_palavra (s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..]
}