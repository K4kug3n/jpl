enum TokenKind {
	ADD,
	MINUS,
	NUMBER,
	EOF
}

struct Token {
	kind: TokenKind,
	value: String
}

struct Lexer<'a> {
	program_iterator: std::str::Chars<'a>
}

impl Lexer<'_> {
	fn new(program: &str) -> Lexer {
		Lexer {
			program_iterator: program.chars()
		}
	}
}

fn main() {
	let program = "52-4";
	
	let mut interpreter = Lexer::new(program);
}