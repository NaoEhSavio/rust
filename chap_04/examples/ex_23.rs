fn primeira_palavra(s: &String) -> usize {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' '{
			return i;
		} 
	}
	s.len()
}