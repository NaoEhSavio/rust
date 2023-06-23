enum Mensagem {
	Sair, // nao ten nenhum dado associado.
	Mover{ // contem uma struct anonima
		x: i32,
		y: i32
	},
	Escrever(String) // contem  uma unica String
	MudarCor(i32,i32,i32), // contem tres valores do tipo i32
}