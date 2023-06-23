// #[cfg(test)]
// mod tests {
// 	#[test]
// 	fn it_works() {
// 		assert_eq!(2 + 2, 4);
// 	}
// }

// mod network {
// 	fn connect(){
		
// 	}
// }

// mod client {
// 	fn connect() {

// 	}
// }
// *********************
// communicator
// 	|-- network
// 	|-- client
// *********************
// mod network {
// 	fn connect() {

// 	}
	
// 	mod client {
// 		fn connect() {

// 		}
// 	}
// }
// *********************
// communicator
// 	|-- network
// 		|-- client
// *********************

// mod client {
// 	fn connect(){

// 	}
// }
// mod network {
// 	fn connect() {

// 	}
// 	mod sever {
// 		fn connect() {

// 		}
// 	}
// }
// *********************
// communicator
//	|-- client
//	|-- network
//		|-- server
// *********************

// mod client;

// mod network {
// 	fn connect() {

// 	}
// 	mod sever {
// 		fn connect() {

// 		}
// 	}
// }

// mod client;

// mod network;

// pub mod client;

// pub mod network;

// mod outermost {
// 	pub fn middle_function() {

// 	}

// 	fn middle_secret_function() {

// 	}

// 	mod inside {
// 		pub fn inner_function(){

// 		}

// 		fn secret_function(){

// 		}
// 	}

// }

// fn try_me() {
// 	outermost::middle_function();
// 	outermost::middle_secret_function();
// 	outermost::inside::inner_function();
// 	outermost::inside::secret_function();
// }

// pub mod client;

// pub mod network;

// #[cfg(test)]
// mod tests {
// 	#[test]
// 	fn it_works() {
// 		client::connect();
// 	}
// }

pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
	use super::client;
	#[test]
	fn it_works() {
		client::connect();
	}
}




