struct User { //struct User<'a>
	username: String,// &'a str // &str
	email: String,// &'a str // &str 
	sign_in_count: u64,
	active: bool,
}
fn main (){
	let mut NeS = User{
		username: String::from("NeS"),// ("NeS") // Receber uma String
		email: "nes@gmail.com".to_owned(),// ("NeS@gmail.com") // Receber uma String
		sign_in_count: 2,
		active: true,
	};
	if NeS.active {
		NeS.sign_in_count = NeS.sign_in_count +1 ;
		println!("Bem-vindo, {}", NeS.username);
	} else {
		println!("Por favor verifiquir seu email: {}", NeS.email);
	}
	println!("Contagem de Login de NeS {}", NeS.sign_in_count);
}