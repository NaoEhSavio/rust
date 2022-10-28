enum Message {
	Quit,
	Move { x: i32, y: i32},
	Write(String),
	ChangeColor(u8,u8,u8),
	//Paste,
}
fn main(){
	let move_msg = Message::Move { x: (15), y: (0) };
	handle_message(move_msg);
	let change_color_msg = Message::ChangeColor(255, 255, 255);
	handle_message(change_color_msg);
	let write_msg = Message::Write("Ola mundo".to_owned());
	handle_message(write_msg);
	let quit_msg = Message::Quit;
	handle_message(quit_msg);
}

fn handle_message(message: Message) {
    use Message::*; // para não colocar Message::Quit
	match message {
		Quit => println!("Saindo da App..."),
		Move { x, y } => println!("Movendo o cursor para ({},{})", x,y),
		Write(texto) => println!(">>> {}", texto),
		ChangeColor(red, green, blue) => {
			println!("Alterar cor para RGB:({},{},{})", red,green,blue)
		}
	};
}



