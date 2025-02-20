mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
.entry:
    psh "Hello, world!"
    prt, ln
    psh 0
    jzr .end
.mid:
    psh "This is the middle"
    prt, ln
.end:
    psh 0
    ret
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
mod evaluator;
