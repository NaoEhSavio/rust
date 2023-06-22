fn main(){
	let referencia_para_0_nada = soltar();
}

fn soltar() -> &String { 			// soltar retorna uma referencia a uma String
	let s = String::from("texto"); 	// s eh uma nova String
	&s 	// retornamos uma referencia a uma String, s
}		// aqui, s sai de escopo e eh destruida. Sua memoria eh devolvida.
		// Perigo!