fn main() {
	let minha_string = String::from("texto longo");
	// primeira_palavra funciona com slices de `String`s
	let palavra = primeira_palavra(&minha_string[..]);
	let minha_string_literal = "texto longo";
	// primeira_palavra funciona com strings literais
	let palavra = palavra_palavra(&minha_string_literal[..]);
	// umas vez que strings literais *sao* slices de strings,
	// isso tambem funciona, sem musar sitaxe de slice!
	let palavra = primeira_palavra(minha_string_literal);
}