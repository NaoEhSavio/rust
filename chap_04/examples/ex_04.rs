fn main(){
	let mut s = String::from("Olá");
	s.push_str(", mundo!"); // push_str() adiciona um literal à String
	println!("{}", s); // isso vai exibir `olá, mundo!`
}