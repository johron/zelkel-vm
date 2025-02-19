mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
push "> "
print
input
push "\n"
add
print
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");
    let nodes = parser::parse(tokens);
    println!("{:#?}", nodes);
    let stack = evaluator::evaluate(nodes.unwrap()).expect("Failed to evaluate");
    println!("{:?}", stack);
}

#[cfg(test)]
mod tests;
mod evaluator;
