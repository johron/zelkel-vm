mod parser;
mod lexer;
mod evaluator;

struct Error {
    message: String,
    line: usize,
    col: usize,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} near {}:{}", self.message, self.line, self.col)
    }
}

impl Error {
    fn new<S: Into<String>>(message: S, line: usize, col: usize) -> Self {
        Self {
            message: message.into(),
            line,
            col,
        }
    }
}

fn main() {
    let code = String::from(r#"
@entry:
    psh "Hello, world!\n"
    len
    rot
    psh 1
    dup
    sys
    pop $
"#);
    let tokens = lexer::lex(code).unwrap_or_else(|err| {
        eprintln!("Failed to lex: {:?}", err);
        std::process::exit(1);
    });
    let parsed = parser::parse(tokens).unwrap_or_else(|err| {
        eprintln!("Failed to parse: {:?}", err);
        std::process::exit(1);
    });
    let evaluated = evaluator::evaluate(parsed).unwrap_or_else(|err| {
        eprintln!("Failed to evaluate: {:?}", err);
        std::process::exit(1);
    });

    let stack = evaluated.0;
    let code = evaluated.1;

    println!("{:?}", stack);

    std::process::exit(code);
}

#[cfg(test)]
mod tests;
