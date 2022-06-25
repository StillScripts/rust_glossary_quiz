pub mod handlers {
	use std::io;

	/**
	 * Get user input and return a trimmed string of text
	 */
	pub fn handle_input(question: String) -> String {
		println!("{}", question);
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Error reading input");
		String::from(input.trim())
	}

	/**
	 * Display a message for when improper input has been provided
	 */
	pub fn handle_no_input() {
		println!("You need to enter 'add' or 'practice'. Or enter 'q' to quit program.");
	}
}
