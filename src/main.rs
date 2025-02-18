mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"push 5"#);
    let tokens = lexer::lex(code).expect("Failed to lex");
    println!("{:?}", tokens);
    let nodes = parser::parse(tokens);
    println!("{:?}", nodes);
}

#[cfg(test)]
mod tests;
