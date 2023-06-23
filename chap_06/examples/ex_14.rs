fn main() {
	let algum_valor_u8 = Some(0u8);
	match algum_valor_u8 {
		Some(3) => println!("tres"),
		_ => (),
	}

	if let Some(3) = algum_valor_u8 {
		println!("tres");
	}

	let mut contagem = 0;
	match moeda {
		Moeda::Quarter(estado) => println!(
			"Quarter  do estado {;?}!", 
			estado
		),
		_ => contagem += 1,
	} 

	if let Moeda::Quarter(estado) = moeda {
		println!(
			"Quarter do estado {;?}!",
			estado
		);
	} else {
		contagem += 1;
	}
}