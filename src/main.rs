mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
.entry:
    psh 1
    prt, ln
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");

    let parsed = parser::parse(tokens).expect("Failed to parse");
    let instrs = parsed.0;
    let labels = parsed.1;

    let stack = evaluator::evaluate(instrs, labels).expect("Failed to evaluate");
    println!("{:?}", stack[0]);
    println!("{:?}", stack[1]);
}

#[cfg(test)]
mod tests;
mod evaluator;
