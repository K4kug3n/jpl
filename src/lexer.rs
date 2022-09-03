use crate::text_iterator::{TextIterator, Symbol};
use crate::operator::{Operator};

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

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
	Operator(Operator),
	Integer,
	Float,
	Bool,
	Identifier,
	LParenthesis,
	RParenthesis,
	LBracket,
	RBracket,
	Coma,
	Assign,
	Let,
	If,
	Fn,
	Semilicon,
	Eof
}

#[derive(Debug, Clone)]
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
	const RESERVED_KEYWORDS : [&'static str; 25] = 
	["+", "-", "*", "/", "(", ")", "{", "}", ",", "=", ";", "&&", "||", "==", "!=", ">=", "<=", ">", "<", "!", "let", "true", "false", "if", "fn"];

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

	fn is_identifier_symbol(value: char) -> bool {
		value.is_alphanumeric() || value == '_'
	}

	fn is_number_symbol(value: char) -> bool {
		value.is_numeric() || value == '.'
	}

	fn next(&mut self) -> Option<Symbol> {
		let current_symbol = self.next_symbol;
		self.next_symbol = self.program_iterator.next();

		current_symbol
	}

	fn identifier(&mut self, word: &mut Word) {
		while let Some(symbol) = self.next_symbol {
			if !Lexer::<'_>::is_identifier_symbol(symbol.value) {
				break;
			}

			word.value.push(symbol.value);
			self.next();
		}
	}

	fn number(&mut self, word: &mut Word) {
		while let Some(symbol) = self.next_symbol {
			if !Lexer::<'_>::is_number_symbol(symbol.value) {
				break;
			}

			word.value.push(symbol.value);
			self.next();
		}
	}

	fn advance(&mut self) -> Option<Word> {
		let mut current_symbol = self.next();

		while current_symbol != None && Lexer::<'_>::is_blank_space(current_symbol.unwrap().value) {
			current_symbol = self.next();
		}

		if current_symbol == None {
			return None;
		}
		
		if let Some(next_symbol) = self.next_symbol { // Handle 2 caractere operator
			let mut potential_double_op = Word::from_symbol(current_symbol.unwrap());
			potential_double_op.value.push(next_symbol.value);

			if Lexer::<'_>::RESERVED_KEYWORDS.contains(&potential_double_op.value.as_str()) {
				self.next();
				return Some(potential_double_op);
			}
		}
		if Lexer::<'_>::RESERVED_KEYWORDS.contains(&current_symbol.unwrap().value.to_string().as_str()) {
			return Some(Word::from_symbol(current_symbol.unwrap()));
		}

		let mut word = Word::from_symbol(current_symbol.unwrap());
		if Lexer::<'_>::is_number_symbol(current_symbol.unwrap().value) {
			self.number(&mut word);
		}
		else {
			self.identifier(&mut word);
		}

		Some(word)
	}

	fn get_kind(value: &str) -> TokenKind {
		match value {
			"+" => TokenKind::Operator(Operator::Add),
			"-" => TokenKind::Operator(Operator::Minus),
			"*" => TokenKind::Operator(Operator::Product),
			"/" => TokenKind::Operator(Operator::Divide),
			"&&" => TokenKind::Operator(Operator::LogicalAnd),
			"||" => TokenKind::Operator(Operator::LogicalOr),
			"(" => TokenKind::LParenthesis,
			")" => TokenKind::RParenthesis,
			"{" => TokenKind::LBracket,
			"}" => TokenKind::RBracket,
			"," => TokenKind::Coma,
			"=" => TokenKind::Assign,
			";" => TokenKind::Semilicon,
			"let" => TokenKind::Let,
			"if" => TokenKind::If,
			"fn" => TokenKind::Fn,
			"true" => TokenKind::Bool,
			"false" => TokenKind::Bool,
			"==" => TokenKind::Operator(Operator::Equal),
			"!=" => TokenKind::Operator(Operator::NotEqual),
			">=" => TokenKind::Operator(Operator::GreaterOrEq),
			"<=" => TokenKind::Operator(Operator::LowerOrEq),
			">" => TokenKind::Operator(Operator::Greater),
			"<" => TokenKind::Operator(Operator::Lower),
			"!" => TokenKind::Operator(Operator::Not),
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

		if Lexer::<'_>::RESERVED_KEYWORDS.contains(&word.value.as_str()) {
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
	fn op_token() {
		expect_token_kind("+", TokenKind::Operator(Operator::Add));
		expect_token_kind("-", TokenKind::Operator(Operator::Minus));
		expect_token_kind("*", TokenKind::Operator(Operator::Product));
		expect_token_kind("/", TokenKind::Operator(Operator::Divide));
		expect_token_kind("&&", TokenKind::Operator(Operator::LogicalAnd));
		expect_token_kind("||", TokenKind::Operator(Operator::LogicalOr));
		expect_token_kind(">=", TokenKind::Operator(Operator::GreaterOrEq));
		expect_token_kind("<=", TokenKind::Operator(Operator::LowerOrEq));
		expect_token_kind("==", TokenKind::Operator(Operator::Equal));
		expect_token_kind("!=", TokenKind::Operator(Operator::NotEqual));
		expect_token_kind(">", TokenKind::Operator(Operator::Greater));
		expect_token_kind("<", TokenKind::Operator(Operator::Lower));
		expect_token_kind("!", TokenKind::Operator(Operator::Not));
		expect_token_kind("=", TokenKind::Assign);
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
	fn pair_token() {
		expect_token_kind("(", TokenKind::LParenthesis);
		expect_token_kind(")", TokenKind::RParenthesis);
		expect_token_kind("{", TokenKind::LBracket);
		expect_token_kind("}", TokenKind::RBracket);
	}

	#[test]
	fn semilicon_token() {
		expect_token_kind(";", TokenKind::Semilicon);
	}

	#[test]
	fn keyword_token() {
		expect_token_kind("true", TokenKind::Bool);
		expect_token_kind("false", TokenKind::Bool);
		expect_token_kind("let", TokenKind::Let);
		expect_token_kind("if", TokenKind::If);
	}

	#[test]
	fn eof_token() {
		expect_token_kind("", TokenKind::Eof);
	}

	#[test]
	fn number_expression_lexing() {
		let mut lexer = Lexer::new("let test=(2.5*3 ) + 2;");

		assert_eq!(lexer.next_token().kind, TokenKind::Let);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Assign);
		assert_eq!(lexer.next_token().kind, TokenKind::LParenthesis);
		assert_eq!(lexer.next_token().kind, TokenKind::Float);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::Product));
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::RParenthesis);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::Add));
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::Semilicon);
	}

	#[test]
	fn bool_expression_lexing() {
		let mut lexer = Lexer::new("let test=true&&false||identifier;");

		assert_eq!(lexer.next_token().kind, TokenKind::Let);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Assign);
		assert_eq!(lexer.next_token().kind, TokenKind::Bool);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::LogicalAnd));
		assert_eq!(lexer.next_token().kind, TokenKind::Bool);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::LogicalOr));
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Semilicon);
	}

	#[test]
	fn condition_expression_lexing() {
		let mut lexer = Lexer::new("let test=2.3==identifier&&3>=4;");

		assert_eq!(lexer.next_token().kind, TokenKind::Let);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Assign);
		assert_eq!(lexer.next_token().kind, TokenKind::Float);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::Equal));
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::LogicalAnd));
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::GreaterOrEq));
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::Semilicon);
	}

	#[test]
	fn if_statement_lexing() {
		let mut lexer = Lexer::new("if id == 2 { }");

		assert_eq!(lexer.next_token().kind, TokenKind::If);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Operator(Operator::Equal));
		assert_eq!(lexer.next_token().kind, TokenKind::Integer);
		assert_eq!(lexer.next_token().kind, TokenKind::LBracket);
		assert_eq!(lexer.next_token().kind, TokenKind::RBracket);
	}

	#[test]
	fn function_declaration_lexing() {
		let mut lexer = Lexer::new("fn foo(arg1, arg2) { }");

		assert_eq!(lexer.next_token().kind, TokenKind::Fn);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::LParenthesis);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::Coma);
		assert_eq!(lexer.next_token().kind, TokenKind::Identifier);
		assert_eq!(lexer.next_token().kind, TokenKind::RParenthesis);
		assert_eq!(lexer.next_token().kind, TokenKind::LBracket);
		assert_eq!(lexer.next_token().kind, TokenKind::RBracket);
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