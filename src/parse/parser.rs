use std::rc::Rc;
use core::{cell::RefCell, panic};

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
    Assignment,
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
    If       { cond: Box<Node>, block: Box<Node>, next: Box<Node> },
    ElseIf   { cond: Box<Node>, block: Box<Node>, next: Box<Node> },
    Else     { block: Box<Node> },
    While    { cond: Box<Node>, block: Box<Node> },
    Var      { var_t: Type, name: Box<Node> },
    Block    { vec: Vec<Rc<RefCell<Node>>> },
    Expr     { e: Box<Node> },
    BinE     { exprl: Box<Node>, op: BinOp, exprr: Box<Node> },
    UnE      { op: UnOp, expr: Box<Node> },
    Id       { id: String },
    IntN     { n: i32 },
    FloatN   { n: f64 },
    CharN    { n: u8 },
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
                root.get_vec_mut().push(Rc::new(RefCell::new(parse_func_def(&mut index, &tokens))));
            }
            _ => {} // TODO:
        };
        index += 1;
    }

    ast.push(root);
}

fn parse_func_def(index: &mut usize, tokens: &Vec<Token>) -> Node {
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
                args_block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, true))));
            }
            while let Token::Comma = tokens[*index] {
                *index += 1;
                args_block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, true))));
            }
            if let Token::RP = tokens[*index] {
                *index += 1;
                args_block
            } else {
                panic!("Compiler message: Expected ')'.");
            }
        }
        _ => {
            panic!("Compiler message: Expected '('.");
        }
    };

    let func_block = match &tokens[*index] {
        Token::LC => {
            *index += 1;
            let mut block = Node::Block { vec: Vec::new() };
            loop {
                match &tokens[*index] {
                    Token::RC => {
                        break;
                    },
                    Token::CharT | Token::IntT | Token::FloatT => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, false))));
                    },
                    Token::If => {
    
                    },
                    Token::Id { tok_lexeme: _ } => {
                        // parse_func_call
                    },
                    Token::Semicolon => {
                        *index += 1;
                    },
                    _ => {
                        panic!("Compiler message: found {:?}.", &tokens[*index]);
                    }
                }
            }
            block
        },
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

fn parse_var(index: &mut usize, tokens: &Vec<Token>, is_arg: bool) -> Node {
    let var_type = match &tokens[*index] {
        Token::IntT => Type::IntT,
        Token::FloatT => Type::FloatT,
        Token::CharT => Type::CharT,
        Token::VoidT => {
            panic!("Compiler message: You can't give void type to a var.");
        },
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

    if !is_arg {
        match tokens[*index] {
            Token::Assignment => {
                *index += 1;
                return Node::Expr { e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::Var { var_t: var_type, name: Box::new(Node::Id { id: var_name }) }), 
                    op: BinOp::Assignment, 
                    exprr: Box::new(parse_expr(index, tokens)) 
                })};
            },
            Token::Semicolon => {
                *index += 1;
            },
            _ => {} // TODO:
        }
    }

    Node::Var { var_t: var_type, name: Box::new(Node::Id { id: var_name }) }
}

fn parse_expr(index: &mut usize, tokens: &Vec<Token>) -> Node {
    match tokens[*index] {
        Token::CharN { num } => {
            *index += 1;
            Node::Expr { e: Box::new(Node::CharN { n: num }) }
        },
        Token::IntN { num } => {
            *index += 1;
            if tokens[*index] != Token::Semicolon {
                return parse_bin_int_expr(index, tokens, num);
            }
            Node::Expr { e: Box::new(Node::IntN { n: num }) }
        },
        Token::FloatN { num } => {
            *index += 1;
            if tokens[*index] != Token::Semicolon {
                return parse_bin_float_expr(index, tokens, num);
            }
            Node::Expr { e: Box::new(Node::FloatN { n: num }) }
        },
        _ => {
            panic!(); // TODO:
        }
    }
}

fn parse_bin_int_expr(index: &mut usize, tokens: &Vec<Token>, num: i32) -> Node {
    match tokens[*index] {
        Token::Plus => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::IntN { n: num }), 
                    op: BinOp::Add, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        Token::Minus => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::IntN { n: num }), 
                    op: BinOp::Sub, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        Token::Star => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::IntN { n: num }), 
                    op: BinOp::Mul, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        Token::Slash => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::IntN { n: num }), 
                    op: BinOp::Div, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        _ => {
            panic!(); // TODO:
        }
    }
}

fn parse_bin_float_expr(index: &mut usize, tokens: &Vec<Token>, num: f64) -> Node {
    match tokens[*index] {
        Token::Plus => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::FloatN { n: num }), 
                    op: BinOp::Add, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        Token::Minus => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::FloatN { n: num }), 
                    op: BinOp::Sub, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        Token::Star => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::FloatN { n: num }), 
                    op: BinOp::Mul, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        Token::Slash => {
            *index += 1;
            return Node::Expr { 
                e: Box::new(Node::BinE { 
                    exprl: Box::new(Node::FloatN { n: num }), 
                    op: BinOp::Div, 
                    exprr: Box::new(parse_expr(index, tokens))
                }) 
            };
        },
        _ => {
            panic!(); // TODO:
        }
    }
}