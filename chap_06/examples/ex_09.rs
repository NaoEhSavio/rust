enum Option<T> {
	Some(T), // algum valor
	None, // nenhum valor
}

let algum_numero = Some(5);
let algum_texto = Some("um texto");
let numero_ausente = Option<i32> = None;
let x: i8 = 5;
let y Option<i8> = Some(5);
let soma = x + y