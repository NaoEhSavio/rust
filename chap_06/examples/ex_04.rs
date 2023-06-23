enum EnderecoIP {
	V4(u8,u8,u8,u8),
	V6(String),
}

let local = EnderecoIp::V4(127,0,0,1);
let loopback = EnderecoIP::V6(String::from("::1"));