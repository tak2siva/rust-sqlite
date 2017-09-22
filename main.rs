use std::io;
use std::io::Write;

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