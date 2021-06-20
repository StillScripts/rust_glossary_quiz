use std::io;
use std::io::Write;
use std::env;
use std::process;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::cmp::Ordering;
use rand::Rng;

// STRUCT FOR CONFIG VARIABLE
struct Config {
    option: Option<String>,
    topic: Option<String>,
}

// IMPL FOR INITIALISING CONFIG VARIABLE
impl Config {
    // FUNCTION FOR CREATING NEW CONFIG
    fn new(args: &[String]) -> Result<Config, &str> {
        // IF USER ENTERS 'cargo run option topic'
        if args.len() == 3 {
            Ok(Config { 
                option: Some(String::from(args[1].clone().trim())), 
                topic: Some(String::from(args[2].clone().trim())) 
            })
        }
        // IF USER ENTERS 'cargo run option' GENERATE TOPIC WITH INPUT
        else if args.len() == 2 {
            Ok(Config { 
                option: Some(String::from(args[1].clone().trim())), 
                topic: Some(handle_input(String::from("Select a topic:")))
            })      
        }
        // IF USER ENTERS 'cargo run' GENERATE CONFIG WITH INPUT
        else {
            Ok(generate_config())
        }
    }
}

// MAIN FUNCTION TO RUN PROGRAM
fn main() {
    println!("Welcome to the Glossary CLI APP!");

    // Collect the 'cargo run option topic' arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Run program using config
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// FUNCTION TO RUN ACTION BASED ON CONFIG STATUS
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running program");
    
    if config.option == Some(String::from("practice")) {
        run_practice(config.topic.unwrap());
    }
    else if config.option == Some(String::from("add")) {
        add_new_term(config.topic.unwrap());
    }
    else {
        println!("Improper arguments received. You entered {}.", config.option.unwrap());
        println!("The activity option must be 'add' or 'practice'");
    }
    Ok(())
}


// FUNCTION TO GENERATE CONFIG WITH USER INPUT
fn generate_config() -> Config {
    println!("Select an activity ('add' to add new term, 'practice' to practice, or 'q' to quit).");
    let mut option = String::new();
    let mut topic = String::new();
    loop {
        println!("{}", option);
        option = handle_input(String::from("Select an activity ('add' to add new term, 'practice' to practice, or 'q' to quit)."));     
        println!("{}", option);
        match option.to_lowercase().as_str() {
            "add" => {
                topic = handle_input(String::from("Select the topic to add a new term to:"));
                break;
            },
            "practice" => {
                topic = handle_input(String::from("Select the topic for the practice:"));
                break;
            },
            "q" => {
                println!("Quitting Program.");
                break;
            }
            _ => {
                handle_no_input();
            },
        };
    }
    Config { option: Some(String::from(option.trim())), topic: Some(String::from(topic.trim())) }
}

// FUNCTION TO CREATE NEW .txt FILE
fn create_file(filename: &str) -> std::io::Result<()> {
    fs::File::create(String::from(filename))?;
    Ok(())
}

// FUNCTION TO OPEN .txt FILE & RETURN VECTOR of LINES
fn open_file(filename: String) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut lines = Vec::new();
    for line in contents.lines() {
        lines.push(String::from(line));
    }
    Ok(lines)
}

// FUNCTION TO HANDLE USER INPUT and RETURN a STRING
fn handle_input(question: String) -> String {
    println!("{}", question);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    String::from(input.trim())
}

// FUNCTUON TO HANDLE WHEN IMPROPER INPUT IS RECEIVED
fn handle_no_input() {
    println!("You need to enter 'add' or 'practice'. Or enter 'q' to quit program.");
}

// #### ADDING NEW TERM PROCESSES (add.rs)
// FUNCTION TO HANDLE ADDING A NEW TERM
fn add_new_term(topic: String) {
    // Generate the name and meaning of the term from user input
    let name = handle_input(String::from("What is the name of the new glossary term:"));
    let meaning = handle_input(String::from("What is the meaning of this term:"));
    // Create the string for the filename path
    let filename_path = format!("src/topics/{}.txt", &topic.to_lowercase());
    // If the file does not exist, create a new file
    let file_exists = Path::new(&filename_path).exists();
    if !file_exists {
        #[allow(unused)]
        let file = create_file(&filename_path);        
    } 
    // Open file in append mode and write a new line to it
    let mut file = fs::OpenOptions::new().append(true).open(&filename_path).expect("Error opening file");
    file.write_all(format!("\n{} -$- {}", &name, &meaning).as_bytes()).expect("Write failed");
}

// #### HANDLING PRACTICE PROCESSES (practice.rs)
// STRUCT FOR INDIVIDUAL QUESTION OPTION
pub struct QuestionOption {
    letter: String,
    meaning: String,
}

// IMPL FOR INITIALISING QUESTION OPTION VARIABLE
impl QuestionOption {
    // FUNCTION FOR CREATING NEW QUESTION OPTION
    fn new(letter: String, meaning: String) -> QuestionOption {
		QuestionOption {
			letter, meaning
		}
    }
}

// STRUCT FOR A  MULTIPLE-CHOICE QUESTION
pub struct MultipleChoice {
    options: Vec<QuestionOption>,
    question: String, 
    correct: String,
}

// IMPL FOR INITIALISING QUESTION OPTION VARIABLE
impl MultipleChoice {
    // FUNCTION FOR CREATING NEW QUESTION OPTION
    fn new(options: Vec<QuestionOption>, question: String, correct: String) -> MultipleChoice {
		MultipleChoice {
			options, question, correct,
		}
    }
}

// FUNCTION TO CONVERT VECTOR INTO MULTIPLE CHOICE QUESTION
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
            String::from(letter_options[index_num]), String::from(meaning)
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
    let questions = MultipleChoice::new(
        options, question, correct_letter
    );
    questions
}

// FUNCTION TO GENERATE RANDOM NUMBER EXCLUDING A VALUE
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
	};
	random
}

// FUNCTION TO TEST INPUT USING A MULTIPLE CHOICE QUESTION
#[allow(unused)]
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
    }
    else {
        println!("You are wrong!");
    }
}

// FUNCTION TO RUN A MULTIPLE CHOICE PRACTICE
pub fn run_practice(topic: String) {
    // Create the string for the filename path
    let filename_path = format!("src/topics/{}.txt", &topic.trim().to_lowercase());
    // If the file does not exist cancel the operation
    let file_exists = Path::new(&filename_path).exists();
    if !file_exists {
        println!("A file for this topic does not exist");
    }
    else {
		// Open file and generate vector
        let topic_lines = open_file(filename_path).unwrap();
		// If there are at least 3 items, a question can be generated
        if topic_lines.len() > 3 {
            loop {
                // Set default variables
                let current_number: u64;
                let mut used_numbers: Vec<u64> = Vec::new(); 
                // Generate correct number for the question 
                loop {
                    let random_number = rand::thread_rng().gen_range(1..topic_lines.len()) as u64;
                    let mut is_used = false;
                    for number in &used_numbers {
                        match number.cmp(&random_number) {
                            Ordering::Equal => { is_used = true; },
                            _ => { is_used = false; },
                        }
                    }
                    if !is_used {
                        current_number = random_number;
                        used_numbers.push(random_number);
                        break;
                    }
                }
                // Generate the decoy number for the question
                #[allow(unused)]
                let first_decoy: u64 = generate_random_number(
                    topic_lines.len() as u64, [current_number].to_vec()
                );
                let second_decoy: u64 = generate_random_number(
                    topic_lines.len() as u64, [current_number, first_decoy].to_vec()
                );
                let third_decoy: u64 = generate_random_number(
                    topic_lines.len() as u64, [current_number, first_decoy, second_decoy].to_vec()
                );

                #[allow(unused)]
                // Create the question based on the four random lines chosen
                let multiple_choice_question = create_question([
                    &topic_lines[current_number as usize], &topic_lines[first_decoy as usize],
                    &topic_lines[second_decoy as usize], &topic_lines[third_decoy as usize] 
                ].to_vec());
                // Run a single test
                test_user(multiple_choice_question);
                // Provide option to loop or break.
                let new_test = handle_input(String::from("Try another question? (Y/N)"));
                if new_test.to_lowercase().as_str() == "y" {
                    println!("New test incoming!");
                }
                else {
                    println!("Ok, have a nice day!");
                    break;
                }
            }
        }
        else {
            println!("There are not enough lines in this file to run a test");
        }
    }
}