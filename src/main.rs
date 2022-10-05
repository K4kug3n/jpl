//use std::env;
use std::fs;

use jpl::interpretor;
use jpl::lexer;
use jpl::parser;
use jpl::type_checker;

fn main() {
	// let args: Vec<String> = env::args().collect();

	// if args.len() != 2 {
	// 	println!("Need one file path as parameter");
	// 	return;
	// }
	// let file_path = &args[1];
	let contents = fs::read_to_string("exemples/basis.jpl")
	.expect("Could not read the file {}");
	
	let mut lexer = lexer::Lexer::new(&contents);

	let mut parser = parser::Parser::new(&mut lexer);

	let ast = parser.ast();

	if let Some(node) = ast {
		let mut checker = type_checker::TypeCheckerVisitor::new();
		checker.check(&node);

		let mut interpretor = interpretor::InterpretorVisitor::new();
		interpretor.interpret(node);
	}
}