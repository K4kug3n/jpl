use jpl::interpretor;
use jpl::lexer;
use jpl::parser;

fn main() {
	let program = "
		let Test = (6 * 2) + (2 * 3) + 3;
		let test = 32.5;
		Test = 2;
		let bool_test = true;
	";
	
	let mut lexer = lexer::Lexer::new(program);

	let mut parser = parser::Parser::new(&mut lexer);

	let ast = parser.ast();

	if let Some(node) = ast {
		let mut interpretor = interpretor::InterpretorVisitor::new();
		interpretor.interpret(node);
	}
}