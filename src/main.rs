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
    psh "input: "
    run @print
    alc *buffer, 128
    psh *buffer
    len
    rot
    psh 0
    psh 0
    sys
    psh *buffer
    prt
"#); // kanskje det er noe her som gjør at navnet *buffer blir lagt til greier. når man nevner en identifier som er buffer endres navnet, ny buffer blir lagd, huske i en hashmap greie
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
