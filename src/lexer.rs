use crate::text_iterator::{TextIterator, Symbol};

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
	pub value: String,

	pub start_col: usize,
	pub start_line: usize,
}

pub struct Lexer<'a> {
	program_iterator: TextIterator<'a>,
	next_symbol: Option<Symbol>,
	current_word: Option<String>,

	start_word_col: usize,
	start_word_line: usize
}

impl Lexer<'_> {
	// TODO: Change this to static hashmap
	const RESERVED_KEYWORDS : [&'static str; 1] = ["let"];
	const RESERVED_SYMBOLS : [&'static str; 8] = ["+", "-", "*", "/", "(", ")", "=", ";"];

	pub fn new(program: &str) -> Lexer {
		let mut new_lexer = Lexer {
			program_iterator: TextIterator::new(program),
			next_symbol: None,
			current_word: None,

			start_word_col: 0,
			start_word_line: 0,
		};

		new_lexer.next_symbol = new_lexer.program_iterator.next();

		new_lexer.advance();

		new_lexer
	}

	fn is_blank_space(value : char) -> bool {
		value == ' ' || value == '\n' || value == '\t'
	}

	fn next(&mut self) -> Option<Symbol> {
		let current_symbol = self.next_symbol;
		self.next_symbol = self.program_iterator.next();

		current_symbol
	}

	fn advance(&mut self) {
		let mut current_symbol = self.next();

		while current_symbol != None && Lexer::<'_>::is_blank_space(current_symbol.unwrap().value) {
			current_symbol = self.next();
		}

		if current_symbol == None {
			self.current_word = None;
			return;
		}

		if Lexer::<'_>::RESERVED_SYMBOLS.contains(&current_symbol.unwrap().value.to_string().as_str()) {
			self.current_word = Some(current_symbol.unwrap().value.to_string());
			self.start_word_col = current_symbol.unwrap().col;
			self.start_word_line = current_symbol.unwrap().line;
			return;
		}

		let mut word = current_symbol.unwrap().value.to_string();
		self.start_word_col = current_symbol.unwrap().col;
		self.start_word_line = current_symbol.unwrap().line;
		while self.next_symbol != None && !Lexer::<'_>::is_blank_space(self.next_symbol.unwrap().value) && !Lexer::<'_>::RESERVED_SYMBOLS.contains(&self.next_symbol.unwrap().value.to_string().as_str()) {
			word.push(self.next().unwrap().value);
		}
		self.current_word = Some(word);
	}

	fn to_token(current_word: String, word_col: usize, word_line: usize) -> Token {
		match current_word.as_str() {
			"+" => Token{ kind: TokenKind::Add, value: String::from("+"), start_col: word_col, start_line: word_line },
			"-" => Token{ kind: TokenKind::Minus, value: String::from("-"), start_col: word_col, start_line: word_line },
			"*" => Token{ kind: TokenKind::Product, value: String::from("*"), start_col: word_col, start_line: word_line },
			"/" => Token{ kind: TokenKind::Divide, value: String::from("/"), start_col: word_col, start_line: word_line },
			"(" => Token{ kind: TokenKind::LParenthesis, value: String::from("("), start_col: word_col, start_line: word_line },
			")" => Token{ kind: TokenKind::RParenthesis, value: String::from(")"), start_col: word_col, start_line: word_line },
			"=" => Token{ kind: TokenKind::Equal, value: String::from("="), start_col: word_col, start_line: word_line },
			";" => Token{ kind: TokenKind::Semilicon, value: String::from(";"), start_col: word_col, start_line: word_line },
			"let" => Token{ kind: TokenKind::Let, value: String::from("let"), start_col: word_col, start_line: word_line },
			_ => panic!("Unknow token")
		}
	}

	pub fn next_token(&mut self) -> Token {
		if self.current_word == None {
			return Token{ 
				kind: TokenKind::Eof,
				value: String::new(),
				start_col: self.start_word_col,
				start_line: self.start_word_line 
			};
		}

		let word : String = self.current_word.as_ref().unwrap().to_string();
		let word_col = self.start_word_col;
		let word_line = self.start_word_line;
		self.advance();

		if word.chars().nth(0).unwrap().is_digit(10){
			if word.contains('.') {
				return Token{ kind: TokenKind::Float, value: word, start_col: word_col, start_line: word_line };
			}
			else {
				return Token{ kind: TokenKind::Integer, value: word, start_col: word_col, start_line: word_line };
			}
		}

		if Lexer::<'_>::RESERVED_KEYWORDS.contains(&word.as_str()) || Lexer::<'_>::RESERVED_SYMBOLS.contains(&word.as_str()) {
			return Lexer::<'_>::to_token(word, word_col, word_line);
		}

		Token{ kind: TokenKind::Identifier, value: word, start_col: word_col, start_line: word_line }
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

	#[test]
	fn kind_lexing() {
		let mut lexer = Lexer::new("let test=(2.5*3 ) + 2;");

		assert_eq!(lexer.next_token().kind, TokenKind::Let);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Equal);
		assert_eq!(lexer.next_token().kind, TokenKind::LParenthesis);
		assert_eq!(lexer.next_token().kind, TokenKind::Float);
		assert_eq!(lexer.next_token().kind, TokenKind::Product);
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::RParenthesis);
		assert_eq!(lexer.next_token().kind, TokenKind::Add);
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::Semilicon);
	}

	#[test]
	fn index_lexing() {
		let mut lexer = Lexer::new("let test=(2.5*3 ) + 2;");

		assert_eq!(lexer.next_token().start_col, 0);
		assert_eq!(lexer.next_token().start_col, 4);
		assert_eq!(lexer.next_token().start_col, 8);
		assert_eq!(lexer.next_token().start_col, 9);
		assert_eq!(lexer.next_token().start_col, 10);
		assert_eq!(lexer.next_token().start_col, 13);
		assert_eq!(lexer.next_token().start_col, 14);
		assert_eq!(lexer.next_token().start_col, 16);
		assert_eq!(lexer.next_token().start_col, 18);
		assert_eq!(lexer.next_token().start_col, 20);
		assert_eq!(lexer.next_token().start_col, 21);
	}
}