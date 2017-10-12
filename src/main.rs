use std::io;
use std::io::Write;

mod meta_command;
mod prepare;
mod statement;

use statement::Statement;

fn print_prompt() {
	print!("sqlite>");
	io::stdout().flush().unwrap();
}

fn read_input() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return String::from(input.trim());
}

fn main() {
	let mut done = false;

	while !done {
		print_prompt();
		let input = read_input();

		if (input.chars().nth(0).unwrap() == '.') {
			match meta_command::do_meta_command(&input) {
				meta_command::MetaCommandResult::META_COMMAND_SUCCESS => {
					println!("meta command success");
				}
				meta_command::MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND => {
					println!("Un recognized command");
				} 
			}
		}

		let mut statement: Statement = statement::defaultStatement();

		match prepare::prepare_statement(&input, &mut statement) {
			prepare::PrepareResult::PREPARE_SUCCESS => {
				println!("sucess!");
			}
			prepare::PrepareResult::PREPARE_UNRECOGNIZED_STATEMENT => {
				println!("Unable to preapare statement");
			}
		}

		match input.as_ref() {
    		"exit" => {
    			done = true;
    			println!("Bye!");
    		}
    		_ => {
    			println!("{}", input);
    		}
		}
	}
}