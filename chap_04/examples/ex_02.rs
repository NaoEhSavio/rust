#![allow(unused)]
fn main() { // s não é válida aqui, ainda não está declarada
	{			
	let s = "texto";  // s é válida deste ponto em diante 
	// faz alguma coisa com s
	}// agora esta escopo terminou, e s não é mais válida
}