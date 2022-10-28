fn main() {
	let mut x = 5;
	println!("Valor de x: {}", x);
	x = 6;
    println!("Valor de x: {}", x);
    let y = x;
	println!("Valor de y: {}", y);
    println!("Valor de x + y: {}", sum(x,y)) ;
}
fn sum(x:i32 , y:i32) -> i32{
    let a = x + y;
    return a;
    // x + y;
}
