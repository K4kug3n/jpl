use crate::text_iterator::{TextIterator, Symbol};

#[derive(PartialEq)]
struct Word {
	value: String,
	start_col: usize,
	start_line: usize,
}

impl Word {
	fn from_symbol(symbol: Symbol) -> Word {
		Word {
			value: symbol.value.to_string(),
			start_col: symbol.col,
			start_line: symbol.line,
		}
	}

	fn is_numeric(&self) -> bool {
		for c in self.value.chars() {
			if !c.is_numeric() && c != '.' {
				return false;
			}
		}

		return true;
	}
}

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

impl Token {
	fn from_word(kind: TokenKind, word: Word) -> Token {
		Token {
			kind: kind,
			value: word.value,
			start_col: word.start_col,
			start_line: word.start_line
		}
	}
}

pub struct Lexer<'a> {
	program_iterator: TextIterator<'a>,
	next_symbol: Option<Symbol>,
}

impl Lexer<'_> {
	// TODO: Change this to static hashmap
	const RESERVED_KEYWORDS : [&'static str; 1] = ["let"];
	const RESERVED_SYMBOLS : [&'static str; 8] = ["+", "-", "*", "/", "(", ")", "=", ";"];

	pub fn new(program: &str) -> Lexer {
		let mut new_lexer = Lexer {
			program_iterator: TextIterator::new(program),
			next_symbol: None,
		};

		new_lexer.next_symbol = new_lexer.program_iterator.next();

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

	fn advance(&mut self) -> Option<Word> {
		let mut current_symbol = self.next();

		while current_symbol != None && Lexer::<'_>::is_blank_space(current_symbol.unwrap().value) {
			current_symbol = self.next();
		}

		if current_symbol == None {
			return None;
		}
 
		if Lexer::<'_>::RESERVED_SYMBOLS.contains(&current_symbol.unwrap().value.to_string().as_str()) {
			return Some(Word::from_symbol(current_symbol.unwrap()));
		}

		let mut word = Word::from_symbol(current_symbol.unwrap());
		while let Some(symbol) = self.next_symbol {
			if Lexer::<'_>::is_blank_space(symbol.value) || Lexer::<'_>::RESERVED_SYMBOLS.contains(&symbol.value.to_string().as_str()) {
				break;
			}

			word.value.push(symbol.value);
			self.next();
		}

		Some(word)
	}

	fn get_kind(value: &str) -> TokenKind {
		match value {
			"+" => TokenKind::Add,
			"-" => TokenKind::Minus,
			"*" => TokenKind::Product,
			"/" => TokenKind::Divide,
			"(" => TokenKind::LParenthesis,
			")" => TokenKind::RParenthesis,
			"=" => TokenKind::Equal,
			";" => TokenKind::Semilicon,
			"let" => TokenKind::Let,
			_ => panic!("Unknow token")
		}
	}

	pub fn next_token(&mut self) -> Token {
		let opt_word = self.advance();

		if opt_word == None {
			return Token{ 
				kind: TokenKind::Eof,
				value: String::new(),
				start_col: 0,
				start_line: 0 
			};
		}

		let word : Word = opt_word.unwrap();

		if word.is_numeric() {
			if word.value.contains('.') {
				return Token::from_word(TokenKind::Float, word);
			}
			else {
				return Token::from_word(TokenKind::Integer, word);
			}
		}

		if Lexer::<'_>::RESERVED_KEYWORDS.contains(&word.value.as_str()) || Lexer::<'_>::RESERVED_SYMBOLS.contains(&word.value.as_str()) {
			return Token::from_word(Lexer::<'_>::get_kind(&word.value), word);
		}

		Token::from_word(TokenKind::Identifier, word)
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