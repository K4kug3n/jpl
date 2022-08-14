#[derive(Debug, PartialEq)]
enum TokenKind {
	ADD,
	MINUS,
	NUMBER,
	IDENTIFIER,
	EOF
}

#[derive(Debug)]
struct Token {
	kind: TokenKind,
	value: String
}

struct Lexer<'a> {
	program_iterator: std::str::Chars<'a>,
	current_value: Option<char>,
}

impl Lexer<'_> {
	fn new(program: &str) -> Lexer {
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
				i.is_numeric()
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
			"+" => Token{ kind: TokenKind::ADD, value: String::from("+") },
			"-" => Token{ kind: TokenKind::MINUS, value: String::from("-") },
			_ => panic!("Unknow token")
		}
	}

	fn next_token(&mut self) -> Token {
		if self.current_value == None {
			return Token{ kind: TokenKind::EOF, value: String::new() };
		}

		let value : char = self.current_value.unwrap();

		if value.is_digit(10){
			return Token{ kind: TokenKind::NUMBER, value: self.parse_number() };
		}

		if value.is_alphabetic() {
			return Token{ kind: TokenKind::IDENTIFIER, value: self.parse_identifier() };
		}

		self.advance();

		Lexer::<'_>::to_token(value.to_string())
	}
}

fn main() {
	let program = "52 - HELLO + tEsT";
	
	let mut interpreter = Lexer::new(program);

	let mut token = interpreter.next_token();

	while token.kind != TokenKind::EOF {
		println!("{:?}", token);

		token = interpreter.next_token();
	}
	
	println!("{:?}", token);
}