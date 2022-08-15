pub mod lexer;

fn main() {
	let program = "52 - HELLO + tEsT / 552 * 3.5";
	
	let mut interpreter = lexer::Lexer::new(program);

	let mut token = interpreter.next_token();

	while token.kind != lexer::TokenKind::EOF {
		println!("{:?}", token);

		token = interpreter.next_token();
	}
	
	println!("{:?}", token);
}