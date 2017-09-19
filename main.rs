use std::io;

fn main() {
	let mut done = false;

	while !done {
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
		    Ok(_n) => {
		    	match input.trim().as_ref() {
		    		"exit" => {
		    			done = true;
		    			println!("Bye!");
		    		}
		    		_ => {
		    			println!("{}", input);
		    		}
		    	}
		    }
		    Err(error) => println!("error: {}", error),
		}
	}
}