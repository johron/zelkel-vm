use super::*;

#[test]
fn lex_push_int() {
    let result = lexer::lex("push 5".to_string());
    assert_eq!(result, Ok(vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("push".to_string()) },
        lexer::Token { kind: "integer", value: lexer::TokenValue::Integer(5) },
    ]));
}

#[test]
fn lex_push_unknown_char_error() {
    let result = lexer::lex("push ?;".to_string());
    assert_eq!(result, Err("Unexpected character: '?'".to_string()));
}

#[test]
fn lex_push_string() {
    let result = lexer::lex("push \"hello\"".to_string());
    assert_eq!(result, Ok(vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("push".to_string()) },
        lexer::Token { kind: "string", value: lexer::TokenValue::String("hello".to_string()) },
    ]));
}

#[test]
fn lex_push_float() {
    let result = lexer::lex("push 3.14".to_string());
    assert_eq!(result, Ok(vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("push".to_string()) },
        lexer::Token { kind: "float", value: lexer::TokenValue::Float(3.14) },
    ]));
}

#[test]
fn parse_push_int() {
    let tokens = vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("push".to_string()) },
        lexer::Token { kind: "integer", value: lexer::TokenValue::Integer(5) },
    ];
    let result = parser::parse(tokens);
    assert_eq!(result, Ok(vec![
        parser::Node {
            kind: parser::NodeKind::Instruction(parser::Instruction {
                kind: parser::InstructionKind::Push(),
                params: vec![parser::ValueType::Integer(5)],
            }),
        },
    ]));
}

#[test]
fn parse_push_float() {
    let tokens = vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("push".to_string()) },
        lexer::Token { kind: "float", value: lexer::TokenValue::Float(3.14) },
    ];
    let result = parser::parse(tokens);
    assert_eq!(result, Ok(vec![
        parser::Node {
            kind: parser::NodeKind::Instruction(parser::Instruction {
                kind: parser::InstructionKind::Push(),
                params: vec![parser::ValueType::Float(3.14)],
            }),
        },
    ]));
}

#[test]
fn parse_push_string() {
    let tokens = vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("push".to_string()) },
        lexer::Token { kind: "string", value: lexer::TokenValue::String("hello".to_string()) },
    ];
    let result = parser::parse(tokens);
    assert_eq!(result, Ok(vec![
        parser::Node {
            kind: parser::NodeKind::Instruction(parser::Instruction {
                kind: parser::InstructionKind::Push(),
                params: vec![parser::ValueType::String("hello".to_string())],
            }),
        },
    ]));
}

#[test]
fn parse_add() {
    let tokens = vec![
        lexer::Token { kind: "identifier", value: lexer::TokenValue::Identifier("add".to_string()) },
    ];
    let result = parser::parse(tokens);
    assert_eq!(result, Ok(vec![
        parser::Node {
            kind: parser::NodeKind::Instruction(parser::Instruction {
                kind: parser::InstructionKind::Add,
                params: vec![],
            }),
        },
    ]));
}