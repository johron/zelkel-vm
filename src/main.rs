mod parser;
mod lexer;
mod evaluator;

fn main() {
    let code = String::from(r#"
@test:
    psh 1
    psh 3
    add
    ret
@entry:
    alc *answer, 16
    psh *answer
    len
    rot
    psh 0
    psh 0
    sys
    pop $_
    psh *answer
    typ str
    psh "yes"
    cmp
    jnz .run_test
    jmp .end
.run_test:
    run @test
.end:

    ret
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
