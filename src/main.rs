mod parser;
mod lexer;
mod evaluator;

fn main() {
    let code = String::from(r#"
.test:
    psh 5
    psh 5
    add
    jmp .here
.entry:
    psh 1
    prt, ln
    jmp .test
.here:
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
