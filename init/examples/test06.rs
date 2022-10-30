enum  Option<T> {
	Some(T),
	None,
}
enum Result<T, E> {
	OK(T),
	Error(E),
}

fn main(){
	let some_number: Option<i32> = Option::Some(5);
	let some_string: Option<&str> = Option::Some("Uma String");
	let _absent_number: Option<i32> = Option::None;

	match some_number {
		Option::Some(x) => println!("O número é: {}",x),
		Option::None =>	println!("Isso não é um número!"),
	};
	
	match some_string {
		Option::Some(x) => println!("A String é: {}",x),
		Option::None =>	println!("Isso não é uma String!"),
	};
}