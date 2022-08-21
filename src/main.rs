use jpl::interpretor;
use jpl::lexer;
use jpl::parser;
fn main() {
	let program = "(6 * 2) + (2 * 3)";
	
	let mut lexer = lexer::Lexer::new(program);

	let mut parser = parser::Parser::new(&mut lexer);

	let ast = parser.ast();

	let mut interpretor = interpretor::InterpretorVisitor::new();
	interpretor.interpret(ast);
}