#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Identifier(String),
    Integer(i32),
    Float(f32),
    String(String),
    Punctuation(char),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: &'static str,
    pub value: TokenValue,
}

fn until<F>(chars: &Vec<char>, start: usize, check: F) -> (String, usize)
where
    F: Fn(char) -> bool
{
    let mut cur = start;
    let mut value = String::new();
    while cur < chars.len() && check(chars[cur]) {
        value.push(chars[cur]);
        cur += 1;
    }

    (value, cur)
}

fn could_be(c: char, s: &str) -> bool {
    s.chars().any(|x| x == c)
}

pub fn lex(input: String) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = vec![];
    let mut cur = 0;

    while cur < chars.len() {
        let c = chars[cur];
        if c.is_alphabetic() {
            let value = until(&chars, cur, |c| c.is_alphanumeric());
            tokens.push(Token { kind: "identifier", value: TokenValue::Identifier(value.0) });
            cur = value.1;
        } else if c == '.' && cur + 1 < chars.len() && chars[cur + 1].is_alphabetic() {
            tokens.push(Token { kind: "punctuation", value: TokenValue::Punctuation('.') });
            cur += 1;
        } else if c.is_digit(10) || c == '.' {
            let value = until(&chars, cur, |c| c.is_digit(10) || c == '.');
            if value.0.contains('.') {
                let float_value: f32 = value.0.parse().map_err(|_| format!("Invalid float: '{}'", value.0))?;
                tokens.push(Token { kind: "float", value: TokenValue::Float(float_value) });
            } else {
                let integer_value: i32 = value.0.parse().map_err(|_| format!("Invalid integer: '{}'", value.0))?;
                tokens.push(Token { kind: "integer", value: TokenValue::Integer(integer_value) });
            }
            cur = value.1;
        } else if c == '"' {
            let value = until(&chars, cur + 1, |c| c != '"');
            if cur + value.0.len() + 1 >= chars.len() || chars[cur + value.0.len() + 1] != '"' {
                return Err("Unclosed string literal".to_string());
            }
            tokens.push(Token { kind: "string", value: TokenValue::String(value.0) });
            cur = value.1 + 1;
        } else if could_be(c, ":,") {
            tokens.push(Token { kind: "punctuation", value: TokenValue::Punctuation(c) });
            cur += 1;
        } else if c.is_whitespace() {
            cur += 1;
        } else {
            Err(format!("Unexpected character: '{}'", c))?;
        }
    }

    Ok(tokens)
}