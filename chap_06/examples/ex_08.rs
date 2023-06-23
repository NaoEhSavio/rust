enum Mensagem {
	Sair, // nao ten nenhum dado associado.
	Mover{ // contem uma struct anonima
		x: i32,
		y: i32
	},
	Escrever(String) // contem  uma unica String
	MudarCor(i32,i32,i32), // contem tres valores do tipo i32
}

impl Mensagem {
	fn invocar(&self) {
		// o corpo do metodo eh definido aqui
	}
}

let m = Mensagem::Escrever(String::from("ola"));
m.invocar();