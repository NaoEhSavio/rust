fn main(){
    let a = 12;
    
    if a < 10 {
        println!("True");
    } else {
        println!("False");
    }
    
    loop {
        println!("Again");
        break;
    }
    let mut number = 3;
    while number !=0 {
        println!("{}!", number);
        number = number - 1;
    }
    let array = [1,2,3,4,5,6];
    for element in array.iter() {       
        println!("Value is: {}", element);
    }
}
