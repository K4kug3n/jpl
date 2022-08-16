pub mod lexer;
pub mod parser;

fn main() {
	let program = "52.2 - 552 + 3.5";
	
	let mut lexer = lexer::Lexer::new(program);

	let mut parser = parser::Parser::new(&mut lexer);

	let node = parser.ast();

	println!("{:?}", node);
}