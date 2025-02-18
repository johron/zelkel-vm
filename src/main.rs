mod parser;
mod lexer;

fn main() {
    let code = String::from(r#"
push 5
push 3
add
"#);
    let result = lexer::lex(code);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests;
