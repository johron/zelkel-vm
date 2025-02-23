mod parser;
mod lexer;
mod evaluator;

fn main() {
    let code = String::from(r#"
@print:
    len
    rot
    psh 1
    psh 1
    sys
    ret
.entry:
    psh "t: "
    run @print
    alc *buffer, 128
    psh 128
    psh *buffer
    psh 0
    psh 0
    sys
    psh *buffer
    prt
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
