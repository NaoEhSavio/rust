struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

let user1 = User {
	email: String::from("alguem@exemplo.com"),
	username: String::from("algum"),
	active: true,
	sign_in_count: 1,
};

let mut user2 = User {
	email: String::from("alguem@exemplo.com"),
	username: String::from("algum"),
	active: true,
	sign_in_count: 1,
};

user1.email = String::from("outroemail@exemplo.com");

fn build_user (email: String, username: String) -> User {
	User {
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
}

fn build_user2 (email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}

let user3 = User {
	email: String::from("outroemail@exemplo.com");
	username: String::from("outronome567");
	active: user1.active,
	sign_in_count: user1.sign_in_count,
};

let user4 = User {
	email: String::from("outro@example.com"),
	username: String::from("outronome567"),
	..user1
};
