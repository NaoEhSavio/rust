struct Ipv4Addr {
	// detalhes omitidos
}
struct Ipv6Addr {
	//detalhes omitidos
}

enum IpAddr {
	V4(Ipv4Addr),
	V6(Ipv6Addr),
}