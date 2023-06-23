enum VersaoIP {
	V4,
	V6,
}

struct EnderecoIp {
	versao: VersaoIp,
	endereco: String,
}

let local = EnderecoIp {
	versao: VersaoIp::V4,
	endereco: String::from("127.0.0.1"),
};
let loopback = EnderecoIP {
	versao: VersaoIP::v6,
	endereco: String::from("::1"),
}