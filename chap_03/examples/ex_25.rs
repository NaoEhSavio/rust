fn main(){
	let x = soma_um(5);
	println!("O valor de x é: {}",x);
}
fn soma_um(x: i32) -> i32 {
	x + 1
}