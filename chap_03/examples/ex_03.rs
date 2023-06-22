fn main() {
	let x = 5; 
	let x = x + 1; //Shadowing
	let x = x * 2; //Shadowing
	println!("O valor de X é: {}", x); 

}