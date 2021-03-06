pub mod quiz {
	use rand::Rng;
	use std::io;

	/**
	 * Struct for a single answer option to a question
	 */
	pub struct QuestionOption {
		pub letter: String,
		pub meaning: String,
	}

	/**
	 * Methods for the question option
	 */
	impl QuestionOption {
		/**
		 * Initialise the question option
		 */
		fn new(letter: String, meaning: String) -> QuestionOption {
			QuestionOption { letter, meaning }
		}
	}

	/**
	 * Struct for a single multiple-choice question
	 */
	pub struct MultipleChoice {
		pub options: Vec<QuestionOption>,
		pub question: String,
		pub correct: String,
	}

	/**
	 * Methods for the multiple-choice question
	 */
	impl MultipleChoice {
		/**
		 * Initialise the multiple-choice question
		 */
		fn new(options: Vec<QuestionOption>, question: String, correct: String) -> MultipleChoice {
			MultipleChoice {
				options,
				question,
				correct,
			}
		}
	}

	/**
	 * Use a list of answer options to create a multiple-choice question
	 */
	pub fn create_question(answer_options: Vec<&String>) -> MultipleChoice {
		// Set default variables
		let mut options: Vec<QuestionOption> = Vec::new();
		let mut index_num = 0;
		let letter_options = ["A", "B", "C", "D"];
		let random_number = rand::thread_rng().gen_range(0..3) as u8;
		let mut correct_letter = String::from("A");
		let mut correct_keyword = "Atom";
		// Iterate through answer options and assign them to options object
		for option in answer_options {
			// Convert option into the meaning
			let split_options = option.split("-$-");
			let split_options: Vec<&str> = split_options.collect();
			let meaning = split_options[1];
			let question = QuestionOption::new(
				String::from(letter_options[index_num]),
				String::from(meaning),
			);
			options.push(question);
			if index_num == random_number as usize {
				correct_letter = String::from(letter_options[index_num]);
				correct_keyword = split_options[0];
			}
			index_num += 1;
		}

		// Assign question based on current variables
		let question = format!("What does {} refer to?", correct_keyword.trim());
		let questions = MultipleChoice::new(options, question, correct_letter);
		questions
	}

	/**
	 * Generate a random number within a range and exclude a list of values
	 */
	pub fn generate_random_number(max: u64, exclude: Vec<u64>) -> u64 {
		let random: u64;
		loop {
			let random_number = rand::thread_rng().gen_range(1..max) as u64;
			let mut is_used = false;
			for number in &exclude {
				if number == &random_number {
					is_used = true;
				}
			}
			if !is_used {
				random = random_number;
				break;
			}
		}
		random
	}

	/**
	 * Test a user on a multiple-choice question
	 */
	pub fn test_user(question: MultipleChoice) {
		// Initialize variables
		let mut answer = String::new();
		let correct = question.correct.as_str().to_lowercase();
		// Print information needed for users to choose an answer
		println!("{}", question.question);
		for answer_option in question.options {
			println!(" {} - {}", answer_option.letter, answer_option.meaning);
		}
		// Handle user input and determine if answer is correct or not
		io::stdin()
			.read_line(&mut answer)
			.expect("Error reading input");
		println!("Your choice was: {}", &answer.trim().to_lowercase());
		if answer.trim().to_lowercase() == correct {
			println!("You are correct!");
		} else {
			println!("You are wrong!");
		}
	}
}
