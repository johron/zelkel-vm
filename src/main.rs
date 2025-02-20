mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
.entry:
    psh ": "
    prt
    inp
    jnz label1
    psh "You didn't enter anything\n"
    prt
    jmp end
.label1:
    psh "You entered something\n"
    prt
.end:
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");

    let parsed = parser::parse(tokens).expect("Failed to parse");
    let instrs = parsed.0;
    let labels = parsed.1;

    let stack = evaluator::evaluate(instrs, labels).expect("Failed to evaluate");
    //println!("{:?}", stack);
}

#[cfg(test)]
mod tests;
mod evaluator;
