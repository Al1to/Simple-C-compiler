use std::rc::Rc;
use core::cell::RefCell;

use crate::lex::lexer::Token;

#[derive(Debug)]
pub enum Type {
    IntT,
    FloatT,
    CharT,
    VoidT,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul, 
    Div,
    Equal, 
    Lesser, Bigger,
    LesserEqual, BiggerEqual,
}

#[derive(Debug)]
pub enum UnOp {
    Negation,
    Inc, Dec, 
    UnAdd, UnSub, UnMul, UnDiv,
}

#[derive(Debug)]
pub enum Node {
    FuncDef  { ret_t: Type, name: Box<Node>, args: Box<Node>, block: Box<Node> },
    FuncCall { name: Box<Node>, args: Box<Node> },
    If       { cond: Box<Node>, block: Box<Node> },
    ElseIf   { cond: Box<Node>, block: Box<Node> },
    Else     { block: Box<Node> },
    While    { cond: Box<Node>, block: Box<Node> },
    Var      { var_t: Type, name: Box<Node> },
    // Block    { vec: Vec<Node> },
    Block    { vec: Vec<Rc<RefCell<Node>>> },
    BinExpr  { exprl: Box<Node>, op: BinOp, exprr: Box<Node> },
    UnExpr   { op: UnOp, expr: Box<Node> },
    Id       { id: String },
    None,
}

impl Node {
    fn get_vec_mut(&mut self) -> &mut Vec<Rc<RefCell<Node>>> {
        if let Node::Block { vec } = self {
            return vec;
        }
        panic!("Compiler message: hz");
    }
}

pub fn parse(tokens: &Vec<Token>, ast: &mut Vec<Node>) {
    let mut root = Node::Block { vec: Vec::new() };

    let mut index = 0;
    while index < tokens.len() {
        match tokens[index] {
            Token::IntT | Token::FloatT | Token::CharT | Token::VoidT => {
                root.get_vec_mut().push(Rc::new(RefCell::new(parse_funcdef(&mut index, &tokens))));
            }
            _ => {} // TODO:
        };
        index += 1;
    }

    ast.push(root);
}

fn parse_funcdef(index: &mut usize, tokens: &Vec<Token>) -> Node {
    let func_ret_type = match tokens[*index] {
        Token::IntT => Type::IntT,
        Token::FloatT => Type::FloatT,
        Token::CharT => Type::CharT,
        Token::VoidT => Type::VoidT,
        _ => {
            panic!("Compiler message: Expected func type.");
        }
    };

    *index += 1;
    let func_name = match &tokens[*index] {
        Token::Id { tok_lexeme } => tok_lexeme.clone(),
        _ => {
            panic!("Compiler message: Expected func name.");
        }
    };

    *index += 1;
    let func_args = match &tokens[*index] {
        Token::LP => {
            *index += 1;
            let mut args_block = Node::Block { vec: Vec::new() };
            if let Token::IntT | Token::FloatT | Token::CharT | Token::VoidT = tokens[*index] {
                args_block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens))));
            }
            while let Token::Comma = tokens[*index] {
                *index += 1;
                args_block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens))));
            }
            args_block
        }
        _ => {
            panic!("Compiler message: Expected '('.");
        }
    };

    let func_block = match &tokens[*index] {
        Token::RP => Node::Block { vec: vec![] },
        _ => {
            panic!("Compiler message: Expected ')'.");
        }
    };

    Node::FuncDef {
        ret_t: func_ret_type,
        name: Box::new(Node::Id { id: func_name }),
        args: Box::new(func_args),
        block: Box::new(func_block),
    }
}

fn parse_var(index: &mut usize, tokens: &Vec<Token>) -> Node {
    let var_type = match &tokens[*index] {
        Token::IntT => Type::IntT,
        Token::FloatT => Type::FloatT,
        Token::CharT => Type::CharT,
        Token::VoidT => Type::VoidT,
        _ => {
            panic!("Compiler message: Expected var type.");
        }
    };

    *index += 1;
    let var_name = match &tokens[*index] {
        Token::Id { tok_lexeme } => {
            *index += 1;
            tok_lexeme.clone()
        }
        _ => {
            panic!("Compiler message: Expected var name.");
        }
    };

    Node::Var { var_t: var_type, name: Box::new(Node::Id { id: var_name }) }
}