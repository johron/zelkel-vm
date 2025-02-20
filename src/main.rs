mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
.entry:
    psh "Enter two numbers equalling 10\n"
    prt
    psh "Number 1: "
    prt
    inp
    typ float
    psh "Number 2: "
    prt
    inp
    typ float
    add
    dup
    psh 10.0
    cmp
    jnz equal
    psh "Not equal"
    prt
    prt
    jmp end
.equal:
    psh "Equal"
    prt
    prt
.end:
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
