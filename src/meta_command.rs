use std;

#[allow(non_camel_case_types)]
pub enum MetaCommandResult {
	META_COMMAND_SUCCESS,
	META_COMMAND_UNRECOGNIZED_COMMAND
}

pub fn do_meta_command(input: &String) -> MetaCommandResult {
	match input.as_ref() {
		".exit" => {
			std::process::exit(0);
		}
		_ => {
			return MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND;
		}
	}
}