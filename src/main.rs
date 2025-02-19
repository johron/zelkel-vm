mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
.entry:
    psh "Hello, world!\n"
    prt
    jmp label1
    psh "This will not be printed\n"
    prt
.label1:
    psh 5
    psh "\n"
    add
    prt
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");

    let parsed = parser::parse(tokens).expect("Failed to parse");
    let instrs = parsed.0;
    let labels = parsed.1;

    let stack = evaluator::evaluate(instrs, labels).expect("Failed to evaluate");
    println!("{:?}", stack);
}

#[cfg(test)]
mod tests;
mod evaluator;
