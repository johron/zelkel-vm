use crate::lexer;

pub enum InstructionKind {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pop,
}

pub struct Instruction {
    pub kind: InstructionKind,
    pub params: Vec<lexer::Token>,
}

pub enum NodeKind {
    Instruction(Instruction),
    Block(Vec<Node>),
}

pub struct Node {
    pub kind: NodeKind,
}

pub fn parse(tokens: Vec<lexer::Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    nodes
}