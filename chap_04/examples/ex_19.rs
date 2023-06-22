fn main(){
	let mut s = String::from("texto");
	let r1 = &s;
	let r2 = &s;
	let r2 = &mut s;
}