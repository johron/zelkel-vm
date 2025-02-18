mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
push 32
push 24
mod
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");
    let nodes = parser::parse(tokens);
    let stack = evaluator::evaluate(nodes.unwrap()).expect("Failed to evaluate");
    println!("{:?}", stack);
}

#[cfg(test)]
mod tests;
mod evaluator;
