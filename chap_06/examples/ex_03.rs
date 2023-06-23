enum EnderecoIp {
	V4(String),
	V6(String),
}

let local = EnderecoIp::V4(String::from("127.0.0.1"));
let loopback = EnderecoIp::V6(String::from("::1"));