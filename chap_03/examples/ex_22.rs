fn main(){
	let x = 5;
	let y = {
		let x = 3; //declaração
		x + 1 //expressão
		//x + 1; -> declaração
	};
	println!("O valor de y é: {}", y);
}