pub mod lexer;
pub mod parser;

fn main() {
	let program = "52.2 * (552 + 3.5) / 12 - 3.141592";
	
	let mut lexer = lexer::Lexer::new(program);

	let mut parser = parser::Parser::new(&mut lexer);

	let ast = parser.ast();

	println!("{:?}", ast);
}