pub mod handlers {
	use std::io;

	// FUNCTION TO HANDLE USER INPUT and RETURN a STRING
	pub fn handle_input(question: String) -> String {
		println!("{}", question);
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Error reading input");
		String::from(input.trim())
	}

	// FUNCTIION TO HANDLE WHEN IMPROPER INPUT IS RECEIVED
	pub fn handle_no_input() {
		println!("You need to enter 'add' or 'practice'. Or enter 'q' to quit program.");
	}
}
