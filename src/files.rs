pub mod local {
	use std::fs;
	use std::error::Error;

	pub fn add_to_waitlist(name: String) {
		println!("Customer is {}", name);
	}
	// FUNCTION TO CREATE NEW .txt FILE
	pub fn create_file(filename: &str) -> std::io::Result<()> {
		fs::File::create(String::from(filename))?;
		Ok(())
	}

	// FUNCTION TO OPEN .txt FILE & RETURN VECTOR of LINES
	pub fn open_file(filename: String) -> Result<Vec<String>, Box<dyn Error>> {
		let contents = fs::read_to_string(filename)?;
		let mut lines = Vec::new();
		for line in contents.lines() {
			lines.push(String::from(line));
		}
		Ok(lines)
	}
}
