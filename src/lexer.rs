#[derive(Debug, PartialEq)]
pub enum TokenKind {
	Add,
	Minus,
	Product,
	Divide,
	Integer,
	Float,
	Identifier,
	LParenthesis,
	RParenthesis,
	Equal,
	Semilicon,
	Eof
}

#[derive(Debug)]
pub struct Token {
	pub kind: TokenKind,
	pub value: String
}

pub struct Lexer<'a> {
	program_iterator: std::str::Chars<'a>,
	current_value: Option<char>,
}

impl Lexer<'_> {
	pub fn new(program: &str) -> Lexer {
		let mut new_lexer = Lexer {
			program_iterator: program.chars(),
			current_value: None
		};

		new_lexer.advance();

		new_lexer
	}

	fn advance(&mut self) {
		self.current_value = self.program_iterator.next();

		while self.current_value != None && self.current_value.unwrap() == ' ' {
			self.current_value = self.program_iterator.next();
		}
	}

	fn is_valid_identifier(value: Option<char>) -> bool {
		match value {
			None => false,
			Some(i) => {
				i.is_alphabetic()
			}
		}
	}

	fn is_valid_number(value: Option<char>) -> bool {
		match value {
			None => false,
			Some(i) => {
				i.is_numeric() || i == '.'
			}
		}
	}

	fn parse_number(&mut self) -> String {
		let mut word = String::new();

		while Lexer::<'_>::is_valid_number(self.current_value) {
			word.push(self.current_value.unwrap());

			self.advance();
		}

		return word;
	}

	fn parse_identifier(&mut self) -> String {
		let mut word = String::new();

		while Lexer::<'_>::is_valid_identifier(self.current_value) {
			word.push(self.current_value.unwrap());

			self.advance();
		}

		return word;
	}

	fn to_token(current_word: String) -> Token {
		match current_word.as_str() {
			"+" => Token{ kind: TokenKind::Add, value: String::from("+") },
			"-" => Token{ kind: TokenKind::Minus, value: String::from("-") },
			"*" => Token{ kind: TokenKind::Product, value: String::from("*") },
			"/" => Token{ kind: TokenKind::Divide, value: String::from("/") },
			"(" => Token{ kind: TokenKind::LParenthesis, value: String::from("(") },
			")" => Token{ kind: TokenKind::RParenthesis, value: String::from(")") },
			"=" => Token{ kind: TokenKind::Equal, value: String::from("=") },
			";" => Token{ kind: TokenKind::Semilicon, value: String::from(";") },
			_ => panic!("Unknow token")
		}
	}

	pub fn next_token(&mut self) -> Token {
		if self.current_value == None {
			return Token{ kind: TokenKind::Eof, value: String::new() };
		}

		let value : char = self.current_value.unwrap();

		if value.is_digit(10){
			let number = self.parse_number();
			if number.contains('.') {
				return Token{ kind: TokenKind::Float, value: number };
			}
			else {
				return Token{ kind: TokenKind::Integer, value: number };
			}
		}

		if value.is_alphabetic() {
			return Token{ kind: TokenKind::Identifier, value: self.parse_identifier() };
		}

		self.advance();

		Lexer::<'_>::to_token(value.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn expect_token_kind(program : &str, token_kind : TokenKind) {
		let mut lexer = Lexer::new(program);
		let token = lexer.next_token();

		assert_eq!(token.kind, token_kind);
	}

	#[test]
	fn add_token() {
		expect_token_kind("+", TokenKind::Add);
	}

	#[test]
	fn minus_token() {
		expect_token_kind("-", TokenKind::Minus);
	}

	#[test]
	fn product_token() {
		expect_token_kind("*", TokenKind::Product);
	}

	#[test]
	fn divide_token() {
		expect_token_kind("/", TokenKind::Divide);
	}

	#[test]
	fn integer_token() {
		expect_token_kind("3325", TokenKind::Integer);
		expect_token_kind("3", TokenKind::Integer);
	}

	#[test]
	fn float_token() {
		expect_token_kind("321596.3", TokenKind::Float);
		expect_token_kind("3.3", TokenKind::Float);
		expect_token_kind("3.3333666", TokenKind::Float);
	}

	#[test]
	fn identifier_token() {
		expect_token_kind("identifier", TokenKind::Identifier);
		expect_token_kind("Testing", TokenKind::Identifier);
		expect_token_kind("iTesting", TokenKind::Identifier);
	}

	#[test]
	fn left_parenthesis_token() {
		expect_token_kind("(", TokenKind::LParenthesis);
	}

	#[test]
	fn right_parenthesis_token() {
		expect_token_kind(")", TokenKind::RParenthesis);
	}

	#[test]
	fn equal_token() {
		expect_token_kind("=", TokenKind::Equal);
	}

	#[test]
	fn semilicon_token() {
		expect_token_kind(";", TokenKind::Semilicon);
	}

	#[test]
	fn eof_token() {
		expect_token_kind("", TokenKind::Eof);
	}
}