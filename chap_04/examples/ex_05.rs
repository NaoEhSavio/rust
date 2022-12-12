fn main() {
	{
		let _s = String::from("texto"); // s é válida deste ponto em diante
		// faz alguma coisa com s 
	} 								   // agora este escopo terminou, e s não é mais válida
}