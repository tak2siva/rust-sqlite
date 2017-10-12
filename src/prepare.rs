// mod statement;
use statement;

#[allow(non_camel_case_types)]
pub enum PrepareResult {
	PREPARE_SUCCESS, 
	PREPARE_UNRECOGNIZED_STATEMENT
}

pub fn prepare_statement(input: &String, statement: &mut statement::Statement) -> PrepareResult {
	match input.as_ref() {
		"insert" => {
			statement.setType(statement::StatementType::STATEMENT_INSERT);
			return PrepareResult::PREPARE_SUCCESS;
		},
		"select" => {
			statement.setType(statement::StatementType::STATEMENT_SELECT);
			return PrepareResult::PREPARE_SUCCESS;
		}
		_ => {
			return PrepareResult::PREPARE_UNRECOGNIZED_STATEMENT;
		}
	}
}