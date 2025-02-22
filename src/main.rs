mod parser;
mod lexer;
mod evaluator;

fn main() {
    let code = String::from(r#"
@add:
    psh 1
    psh 2
    add
    ret

.entry:
    psh "Hello, world!"
    prt, ln
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");

    let parsed = parser::parse(tokens).expect("Failed to parse");
    let instrs = parsed.0;
    let labels = parsed.1;

    let evaluated = evaluator::evaluate(instrs, labels).expect("Failed to evaluate");
    let stack = evaluated.0;
    let code = evaluated.1;

    println!("{:?}", stack);

    std::process::exit(code);
}

#[cfg(test)]
mod tests;
