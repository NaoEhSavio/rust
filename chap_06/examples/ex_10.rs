enum Moeda {
	Penny,
	Nickel,
	Dime,
	Quarter,
} 

// fn  valor_em_cents(moeda: Moeda) -> u32 {
// 	match moeda {
// 		Moeda::Penny => 1,
// 		Moeda::Nickel => 5,
// 		Moeda::Dime => 10,
// 		Moeda::Quarter => 25,
// 	}
// }

fn  valor_em_cents(moeda: Moeda) -> u32 {
	match moeda {
		Moeda::Penny => {
			println!("MOeda da sorte!");
			1
		},
		Moeda::Nickel => 5,
		Moeda::Dime => 10,
		Moeda::Quarter => 25,
	}
}