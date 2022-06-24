use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

mod files;
mod input;
mod practice;

pub use crate::files::local;
pub use crate::input::handlers;
pub use crate::practice::quiz;

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
                topic: Some(String::from(args[2].clone().trim())),
            })
        }
        // IF USER ENTERS 'cargo run option' GENERATE TOPIC WITH INPUT
        else if args.len() == 2 {
            Ok(Config {
                option: Some(String::from(args[1].clone().trim())),
                topic: Some(handlers::handle_input(String::from("Select a topic:"))),
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
    local::add_to_waitlist(String::from("Daniel"));

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
    } else if config.option == Some(String::from("add")) {
        add_new_term(config.topic.unwrap());
    } else {
        println!(
            "Improper arguments received. You entered {}.",
            config.option.unwrap()
        );
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
        option = handlers::handle_input(String::from(
            "Select an activity ('add' to add new term, 'practice' to practice, or 'q' to quit).",
        ));
        println!("{}", option);
        match option.to_lowercase().as_str() {
            "add" => {
                topic =
                    handlers::handle_input(String::from("Select the topic to add a new term to:"));
                break;
            }
            "practice" => {
                topic = handlers::handle_input(String::from("Select the topic for the practice:"));
                break;
            }
            "q" => {
                println!("Quitting Program.");
                break;
            }
            _ => {
                handlers::handle_no_input();
            }
        };
    }
    Config {
        option: Some(String::from(option.trim())),
        topic: Some(String::from(topic.trim())),
    }
}

// #### ADDING NEW TERM PROCESSES (add.rs)
// FUNCTION TO HANDLE ADDING A NEW TERM
fn add_new_term(topic: String) {
    // Generate the name and meaning of the term from user input
    let name = handlers::handle_input(String::from("What is the name of the new glossary term:"));
    let meaning = handlers::handle_input(String::from("What is the meaning of this term:"));
    // Create the string for the filename path
    let filename_path = format!("src/topics/{}.txt", &topic.to_lowercase());
    // If the file does not exist, create a new file
    let file_exists = Path::new(&filename_path).exists();
    if !file_exists {
        #[allow(unused)]
        let file = local::create_file(&filename_path);
    }
    // Open file in append mode and write a new line to it
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&filename_path)
        .expect("Error opening file");
    file.write_all(format!("\n{} -$- {}", &name, &meaning).as_bytes())
        .expect("Write failed");
}

// #### HANDLING PRACTICE PROCESSES (practice.rs)

// FUNCTION TO TEST INPUT USING A MULTIPLE CHOICE QUESTION
#[allow(unused)]
pub fn test_user(question: quiz::MultipleChoice) {
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

// FUNCTION TO RUN A MULTIPLE CHOICE PRACTICE
pub fn run_practice(topic: String) {
    // Create the string for the filename path
    let filename_path = format!("src/topics/{}.txt", &topic.trim().to_lowercase());
    // If the file does not exist cancel the operation
    let file_exists = Path::new(&filename_path).exists();
    if !file_exists {
        println!("A file for this topic does not exist");
    } else {
        // Open file and generate vector
        let topic_lines = local::open_file(filename_path).unwrap();
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
                            Ordering::Equal => {
                                is_used = true;
                            }
                            _ => {
                                is_used = false;
                            }
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
                let first_decoy: u64 = quiz::generate_random_number(
                    topic_lines.len() as u64,
                    [current_number].to_vec(),
                );
                let second_decoy: u64 = quiz::generate_random_number(
                    topic_lines.len() as u64,
                    [current_number, first_decoy].to_vec(),
                );
                let third_decoy: u64 = quiz::generate_random_number(
                    topic_lines.len() as u64,
                    [current_number, first_decoy, second_decoy].to_vec(),
                );

                #[allow(unused)]
                // Create the question based on the four random lines chosen
                let multiple_choice_question = quiz::create_question(
                    [
                        &topic_lines[current_number as usize],
                        &topic_lines[first_decoy as usize],
                        &topic_lines[second_decoy as usize],
                        &topic_lines[third_decoy as usize],
                    ]
                    .to_vec(),
                );
                // Run a single test
                test_user(multiple_choice_question);
                // Provide option to loop or break.
                let new_test = handlers::handle_input(String::from("Try another question? (Y/N)"));
                if new_test.to_lowercase().as_str() == "y" {
                    println!("New test incoming!");
                } else {
                    println!("Ok, have a nice day!");
                    break;
                }
            }
        } else {
            println!("There are not enough lines in this file to run a test");
        }
    }
}
