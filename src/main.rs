use crate::lexer::DebugSymbol;

mod parser;
mod lexer;
mod evaluator;

struct Error {
    message: String,
    path: Option<String>,
    dsline: Option<usize>,
    dscol: Option<usize>,
    line: usize,
    col: usize,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{} near {}:{}:{} ({}:{})", self.message, path, self.dsline.unwrap(), self.dscol.unwrap(), self.line, self.col)
        } else {
            write!(f, "{} near {}:{}", self.message, self.line, self.col)
        }
    }
}

impl Error {
    fn new<S: Into<String>>(message: S, line: usize, col: usize, debug_symbol: &Option<DebugSymbol>) -> Self {
        if let Some(ds) = debug_symbol {
            Self {
                message: message.into(),
                path: Some(ds.clone().path),
                dsline: Some(ds.line),
                dscol: Some(ds.col),
                line,
                col,
            }
        } else {
            Self {
                message: message.into(),
                path: None,
                dsline: None,
                dscol: None,
                line,
                col,
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path: &String = &"test.zk".to_string();
    if args.len() >= 2 {
        path = &args[1];
    }
    let code = std::fs::read_to_string(path).expect("Failed to read the file");

    let tokens = lexer::lex(code).unwrap_or_else(|err| {
        eprintln!("Runtime error: Failed to lex: {:?}", err);
        std::process::exit(1);
    });
    let parsed = parser::parse(tokens).unwrap_or_else(|err| {
        eprintln!("Runtime error: Failed to parse: {:?}", err);
        std::process::exit(1);
    });
    let evaluated = evaluator::evaluate(parsed).unwrap_or_else(|err| {
        eprintln!("Runtime error: Failed to evaluate: {:?}", err);
        std::process::exit(1);
    });

    let stack = evaluated.0;
    let code = evaluated.1;

    println!("{:?}", stack);

    std::process::exit(code);
}

#[cfg(test)]
mod tests;
