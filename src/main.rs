use std::collections::HashMap;
use std::error::Error;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "DFA CLI", about = "A CLI program for creating and testing DFAs.")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Create a DFA.")]
    Create { alphabet: String, states: String, transitions: String, start: String, accept: String },
    #[structopt(about = "Test a DFA with an input string.")]
    Test { input: String },
}

#[derive(Debug, Clone)]
struct DFA {
    alphabet: Vec<char>,
    states: Vec<String>,
    transitions: HashMap<(String, char), String>,
    start: String,
    accept: Vec<String>,
}

impl DFA {
    fn new(alphabet: String, states: String, transitions: String, start: String, accept: String) -> Result<Self, Box<dyn Error>> {
        let alphabet: Vec<char> = alphabet.chars().collect();
        let states: Vec<String> = states.split(',').map(|s| s.to_string()).collect();

        let transitions: HashMap<(String, char), String> = transitions.split(',')
            .map(|s| s.split("->").collect::<Vec<&str>>())
            .map(|v| ((v[0].to_string(), v[1].chars().next().unwrap()), v[2].to_string()))
            .collect();

        let accept: Vec<String> = accept.split(',').map(|s| s.to_string()).collect();

        Ok(Self {
            alphabet,
            states,
            transitions,
            start,
            accept,
        })
    }

    fn test(&self, input: String) -> Result<bool, Box<dyn Error>> {
        let mut current_state = self.start.clone();

        for c in input.chars() {
            if let Some(next_state) = self.transitions.get(&(current_state, c)) {
                current_state = next_state.clone();
            } else {
                return Err("Invalid transition".into());
            }
        }

        Ok(self.accept.contains(&current_state))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut dfa: Option<DFA> = None;

    loop {
        println!("Enter a command (create, test, help, exit):");
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        match command {
            "create" => {
                println!("Enter the alphabet (e.g. 01):");
                let mut alphabet = String::new();
                io::stdin().read_line(&mut alphabet)?;

                println!("Enter the states (e.g. q0,q1):");
                let mut states = String::new();
                io::stdin().read_line(&mut states)?;

                println!("Enter the transitions (e.g. q0->0->q0,q0->1->q1,q1->0->q1,q1->1->q0):");
                let mut transitions = String::new();
                io::stdin().read_line(&mut transitions)?;

                println!("Enter the start state (e.g. q0):");
                let mut start = String::new();
                io::stdin().read_line(&mut start)?;

                println!("Enter the accept states (e.g. q1):");
                let mut accept = String::new();
                io::stdin().read_line(&mut accept)?;

                dfa = Some(DFA::new(
                    alphabet.trim().to_string(),
                    states.trim().to_string(),
                    transitions.trim().to_string(),
                    start.trim().to_string(),
                    accept.trim().to_string(),
                )?);
                println!("DFA created: {:?}", dfa.as_ref().unwrap());
            }
            "test" => {
                if let Some(dfa) = &dfa {
                    println!("Enter the input string:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim();

                    match dfa.test(input.to_string()) {
                        Ok(result) => println!("DFA test result: {}", result),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Error: No DFA has been created yet. Use the 'create' command first.");
                }
            }
            "help" => {
                println!("Commands:");
                println!("  create - Create a new DFA");
                println!("  test - Test the current DFA with an input string");
                println!("  help - Show help information");
                println!("  exit - Exit the program");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid command. Type 'help' for available commands.");
            }
        }
    }

    Ok(())
}

