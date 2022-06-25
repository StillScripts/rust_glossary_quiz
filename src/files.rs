pub mod local {
	use std::fs;
	use std::error::Error;

	/**
	 * Create a file that stores the glossary terms for a topic
	 */
	pub fn create_file(filename: &str) -> std::io::Result<()> {
		fs::File::create(String::from(filename))?;
		Ok(())
	}

	/**
	 * Open a file that stores the glossary terms for a topic, then
	 * read and return the content of that file
	 */
	pub fn open_file(filename: String) -> Result<Vec<String>, Box<dyn Error>> {
		let contents = fs::read_to_string(filename)?;
		let mut lines = Vec::new();
		for line in contents.lines() {
			lines.push(String::from(line));
		}
		Ok(lines)
	}
}
