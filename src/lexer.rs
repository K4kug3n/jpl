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
	Let,
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
	next_char: Option<char>,
	current_word: Option<String>,
}

impl Lexer<'_> {
	// TODO: Change this to static hashmap
	const RESERVED_KEYWORDS : [&'static str; 1] = ["let"];
	const RESERVED_SYMBOLS : [&'static str; 8] = ["+", "-", "*", "/", "(", ")", "=", ";"];

	pub fn new(program: &str) -> Lexer {
		let mut new_lexer = Lexer {
			program_iterator: program.chars(),
			next_char: None,
			current_word: None,
		};

		new_lexer.next_char = new_lexer.program_iterator.next();

		new_lexer.advance();

		new_lexer
	}

	fn is_blank_space(value : char) -> bool {
		value == ' ' || value == '\n' || value == '\t'
	}

	fn next(&mut self) -> Option<char> {
		let current_char = self.next_char;
		self.next_char = self.program_iterator.next();

		current_char
	}

	fn advance(&mut self) {
		let mut current_char = self.next();

		while current_char != None && Lexer::<'_>::is_blank_space(current_char.unwrap()) {
			current_char = self.next();
		}

		if current_char == None {
			self.current_word = None;
			return;
		}

		if Lexer::<'_>::RESERVED_SYMBOLS.contains(&current_char.unwrap().to_string().as_str()) {
			self.current_word = Some(current_char.unwrap().to_string());
			return;
		}

		let mut word = current_char.unwrap().to_string();
		while self.next_char != None && !Lexer::<'_>::is_blank_space(self.next_char.unwrap()) && !Lexer::<'_>::RESERVED_SYMBOLS.contains(&self.next_char.unwrap().to_string().as_str()) {
			word.push(self.next().unwrap());
		}
		self.current_word = Some(word);
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
			"let" => Token{ kind: TokenKind::Let, value: String::from("let") },
			_ => panic!("Unknow token")
		}
	}

	pub fn next_token(&mut self) -> Token {
		if self.current_word == None {
			return Token{ kind: TokenKind::Eof, value: String::new() };
		}

		let word : String = self.current_word.as_ref().unwrap().to_string();
		self.advance();

		if word.chars().nth(0).unwrap().is_digit(10){
			if word.contains('.') {
				return Token{ kind: TokenKind::Float, value: word };
			}
			else {
				return Token{ kind: TokenKind::Integer, value: word };
			}
		}

		if Lexer::<'_>::RESERVED_KEYWORDS.contains(&word.as_str()) || Lexer::<'_>::RESERVED_SYMBOLS.contains(&word.as_str()) {
			return Lexer::<'_>::to_token(word);
		}

		Token{ kind: TokenKind::Identifier, value: word }
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
	fn let_token() {
		expect_token_kind("let", TokenKind::Let);
	}

	#[test]
	fn eof_token() {
		expect_token_kind("", TokenKind::Eof);
	}
}