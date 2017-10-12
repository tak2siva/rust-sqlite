#[allow(non_camel_case_types)]
pub enum StatementType {
	STATEMENT_INSERT, 
	STATEMENT_SELECT,
	UNKNOWN
}

pub struct Statement {
	st_type: StatementType
}

pub fn defaultStatement() -> Statement {
	return Statement{ st_type: StatementType::UNKNOWN };
}

impl Statement {
	pub fn setType(&mut self, st_type: StatementType) {
		self.st_type = st_type;
	}
}