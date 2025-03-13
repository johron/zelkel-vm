mod parser;
mod lexer;
mod evaluator;

fn main() {
    let code = String::from(r#"
@print:
    len
    rot
    psh 0
    dup
    sys
@entry:
    psh ":"
    run @print
    alc *buffer, 128
    psh *buffer
    len
    rot
    psh 0
    dup
    sys
    pop $_
    psh *buffer
    typ str
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");

    let parsed = parser::parse(tokens).expect("Failed to parse");

    let evaluated = evaluator::evaluate(parsed).expect("Failed to evaluate");
    let stack = evaluated.0;
    let code = evaluated.1;

    println!("{:?}", stack);

    std::process::exit(code);
}

#[cfg(test)]
mod tests;
