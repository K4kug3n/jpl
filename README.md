<div align="center">
  <div>
    <a href="https://github.com/K4kug3n/jpl/actions?query=workflow%3Alinux-build">
      <img src="https://github.com/K4kug3n/jpl/workflows/linux-build/badge.svg" alt="github-ci" />
    </a>
  </div>
  <div>
    <a href="https://github.com/K4kug3n/jpl/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/K4kug3n/jpl?style=plastic" alt="license" />
    </a>
  </div>

</div>

# Just a Programming Language

JPL is a toy language to practice programming language creation, made in Rust as learning project

## Current Grammar
Inspired by Compilers & Interpreters course from University of Geneva  

program ::= list_instr  

list-instr ::= [ instr list_instr ]

instr ::= [ 'let' ] VARIABLE '=' expression;  
instr ::= 'if' expression '{' [ list-instr ] '}' 
instr ::= 'fn' IDENTIFIER '(' [ IDENTIFIER [ ',' IDENTIFIER ] ] ')' '{' [ list-instr ] '}' 

expression ::= equality-expression  

equality-expression ::= additive-expression [ ( '==' | '!=' | '<=' | '>=' | '<' | '>' ) additive-expression ] *  

additive-expression ::= multiplicative-expression [ ( '+' | '-' | '||' ) multiplicative-expression ] *  

multiplicative-expression ::= primary [ ( '*' | '/' | '&&' ) primary ] *  

primary ::= '(' expression ')' | NUMBER | IDENTIFIER | BOOL | '!' primary | '-' primary