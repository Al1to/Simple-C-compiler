use std::rc::Rc;
use core::{cell::RefCell, panic};

use crate::lex::lexer::Token;

#[derive(Debug)]
pub enum Type {
    IntT,
    FloatT,
    CharT,
    BoolT,
    VoidT,
}

#[derive(Debug)]
pub enum BinOp {
    Assignment,
    Add,
    Sub,
    Mul, 
    Div,
    Equal, NotEqual,
    Lesser, Bigger,
    LesserEqual, BiggerEqual,
    UnAdd, UnSub, UnMul, UnDiv,
}

#[derive(Debug)]
pub enum UnOp {
    Negation,
    Inc, Dec, 
}

#[derive(Debug)]
pub enum Node {
    FuncDef  { ret_t: Type, name: Box<Node>, args: Box<Node>, block: Box<Node> },
    FuncCall { name: Box<Node>, args: Box<Node> },
    If       { cond: Box<Node>, block: Box<Node>, next: Box<Node> },
    Else     { block: Box<Node> },
    While    { cond: Box<Node>, block: Box<Node> },
    VarDef   { var_t: Type, name: Box<Node> },
    Var      { name: Box<Node> },
    Block    { vec: Vec<Rc<RefCell<Node>>> },
    Expr     { e: Box<Node> },
    BinE     { exprl: Box<Node>, op: BinOp, exprr: Box<Node> },
    UnE      { op: UnOp, expr: Box<Node> },
    Id       { id: String },
    IntN     { n: i32 },
    FloatN   { n: f64 },
    CharN    { n: u8 },
    BoolTrue,
    BoolFalse,
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
            Token::IntT | Token::FloatT | Token::CharT | Token::BoolT | Token::VoidT => {
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
        Token::BoolT => Type::BoolT,
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
            if let Token::IntT | Token::FloatT | Token::CharT | Token::BoolT | Token::VoidT = tokens[*index] {
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
                    Token::CharT | Token::BoolT | Token::IntT | Token::FloatT => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, false))));
                    },
                    Token::If => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_if(index, tokens))));
                    },
                    Token::While => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_while(index, tokens))));
                    },
                    Token::Id { tok_lexeme: _ } | Token::Negation | Token::Inc | Token::Dec => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_expr(index, tokens))));
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

fn parse_func_call(index: &mut usize, tokens: &Vec<Token>) -> Node {
    let func_name = match &tokens[*index] {
        Token::Id { tok_lexeme } => tok_lexeme.clone(),
        _ => {
            panic!("Compiler message: Expected func name.");
        }
    };

    *index += 2;
    let func_args = match tokens[*index] {
        Token::RP => {
            Node::Block { vec: Vec::new() }
        },
        _ => {
            let mut block = Node::Block { vec: Vec::new() };
            block.get_vec_mut().push(Rc::new(RefCell::new(parse_expr(index, tokens))));
            loop {
                match &tokens[*index] {
                    Token::RP => {
                        break;
                    },
                    Token::Comma => {
                        *index += 1;
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_expr(index, tokens))));
                    },
                    _ => {
                        panic!("Compiler message: found {:?}.", &tokens[*index]);
                    }
                }
            }
            block
        }
    };

    *index += 1;
    Node::FuncCall { name: Box::new(Node::Id { id: func_name }), args: Box::new(func_args) }
}

fn parse_var(index: &mut usize, tokens: &Vec<Token>, is_arg: bool) -> Node {
    let var_type = match &tokens[*index] {
        Token::IntT => Type::IntT,
        Token::FloatT => Type::FloatT,
        Token::CharT => Type::CharT,
        Token::BoolT => Type::BoolT,
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
                    exprl: Box::new(Node::VarDef { var_t: var_type, name: Box::new(Node::Id { id: var_name }) }), 
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

    Node::VarDef { var_t: var_type, name: Box::new(Node::Id { id: var_name }) }
}

fn parse_if(index: &mut usize, tokens: &Vec<Token>) -> Node {
    *index += 1;
    let if_cond = match tokens[*index] {
        Token::LP => {
            *index += 1;
            parse_expr(index, tokens)
        },
        _ => {
            panic!("Compiler message: expected '('.");
        }
    };

    *index += 1;
    let if_block = match &tokens[*index] {
        Token::LC => {
            *index += 1;
            let mut block = Node::Block { vec: Vec::new() };
            loop {
                match &tokens[*index] {
                    Token::RC => {
                        break;
                    },
                    Token::CharT | Token::BoolT | Token::IntT | Token::FloatT => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, false))));
                    },
                    Token::If => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_if(index, tokens))));
                    },
                    Token::While => { // TODO:
    
                    },
                    Token::Id { tok_lexeme: _ } | Token::Negation | Token::Inc | Token::Dec => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_expr(index, tokens))));
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

    *index += 1;
    let if_next = match tokens[*index] {
        Token::Else => {
            if tokens[*index + 1] == Token::If {
                *index += 1;
                return parse_if(index, tokens);
            }

            *index += 2;
            let mut block = Node::Block { vec: Vec::new() };
            loop {
                match &tokens[*index] {
                    Token::RC => {
                        break;
                    },
                    Token::CharT | Token::BoolT | Token::IntT | Token::FloatT => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, false))));
                    },
                    Token::If => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_if(index, tokens))));
                    },
                    Token::While => { // TODO:
    
                    },
                    Token::Id { tok_lexeme: _ } | Token::Negation | Token::Inc | Token::Dec => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_expr(index, tokens))));
                    },
                    Token::Semicolon => {
                        *index += 1;
                    },
                    _ => {
                        panic!("Compiler message: found {:?}.", &tokens[*index]);
                    }
                }
            }

            *index += 1;
            Node::Else { block: Box::new(block) }
        },
        _ => {
            Node::None
        }
    };

    Node::If { cond: Box::new(if_cond), block: Box::new(if_block), next: Box::new(if_next) }
}

fn parse_while(index: &mut usize, tokens: &Vec<Token>) -> Node {
    *index += 1;
    let while_cond = match tokens[*index] {
        Token::LP => {
            *index += 1;
            parse_expr(index, tokens)
        },
        _ => {
            panic!("Compiler message: expected '('.");
        }
    };

    *index += 1;
    let while_block = match &tokens[*index] {
        Token::LC => {
            *index += 1;
            let mut block = Node::Block { vec: Vec::new() };
            loop {
                match &tokens[*index] {
                    Token::RC => {
                        break;
                    },
                    Token::CharT | Token::BoolT | Token::IntT | Token::FloatT => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_var(index, tokens, false))));
                    },
                    Token::If => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_if(index, tokens))));
                    },
                    Token::While => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_while(index, tokens))));
                    },
                    Token::Id { tok_lexeme: _ } | Token::Negation | Token::Inc | Token::Dec => {
                        block.get_vec_mut().push(Rc::new(RefCell::new(parse_expr(index, tokens))));
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

    Node::While { cond: Box::new(while_cond), block: Box::new(while_block) }
}

fn parse_expr(index: &mut usize, tokens: &Vec<Token>) -> Node {
    match tokens[*index] {
        Token::Id { tok_lexeme: _ } => {
            if tokens[*index + 1] == Token::LP {
                let func_call = parse_func_call(index, tokens);
                return parse_bin_func_call_expr(index, tokens, func_call);
            }
            return Node::Expr { e: Box::new(parse_bin_var_expr(index, tokens)) };
        },
        Token::Negation => {
            *index += 1;
            match tokens[*index] {
                Token::Id { tok_lexeme: _ } => {
                    if tokens[*index + 1] == Token::LP {
                        return Node::Expr { e: Box::new(Node::UnE { 
                            op: UnOp::Negation, 
                            expr: Box::new(parse_func_call(index, tokens)) }) 
                        };
                    }
                    return Node::Expr { e: Box::new(Node::UnE { 
                        op: UnOp::Negation, 
                        expr: Box::new(parse_bin_var_expr(index, tokens)) })
                    };
                },
                _ => {
                    panic!("Compiler message: found {:?}.", tokens[*index]);
                }
            };
        },
        Token::Inc => {
            *index += 1;
            match tokens[*index] {
                Token::Id { tok_lexeme: _ } => {
                    if tokens[*index + 1] == Token::LP {
                        return Node::Expr { e: Box::new(Node::UnE { 
                            op: UnOp::Inc, 
                            expr: Box::new(parse_func_call(index, tokens)) }) 
                        };
                    }
                    return Node::Expr { e: Box::new(Node::UnE { 
                        op: UnOp::Inc, 
                        expr: Box::new(parse_bin_var_expr(index, tokens)) })
                    };
                },
                _ => {
                    panic!("Compiler message: hz");
                }
            };
        },
        Token::Dec => {
            *index += 1;
            match tokens[*index] {
                Token::Id { tok_lexeme: _ } => {
                    if tokens[*index + 1] == Token::LP {
                        return Node::Expr { e: Box::new(Node::UnE { 
                            op: UnOp::Dec, 
                            expr: Box::new(parse_func_call(index, tokens)) }) 
                        };
                    }
                    return Node::Expr { e: Box::new(Node::UnE { 
                        op: UnOp::Dec, 
                        expr: Box::new(parse_bin_var_expr(index, tokens)) })
                    };
                },
                _ => {
                    panic!("Compiler message: hz");
                }
            };
        },
        Token::CharN { num } => {
            *index += 1;
            Node::Expr { e: Box::new(Node::CharN { n: num }) }
        },
        Token::IntN { num } => {
            *index += 1;
            if tokens[*index] != Token::Semicolon && tokens[*index] != Token::RP {
                return parse_bin_int_expr(index, tokens, num);
            }
            Node::Expr { e: Box::new(Node::IntN { n: num }) }
        },
        Token::FloatN { num } => {
            *index += 1;
            if tokens[*index] != Token::Semicolon && tokens[*index] != Token::RP {
                return parse_bin_float_expr(index, tokens, num);
            }
            Node::Expr { e: Box::new(Node::FloatN { n: num }) }
        },
        Token::BoolTrue => {
            *index += 1;
            Node::Expr { e: Box::new(Node::BoolTrue) }
        },
        Token::BoolFalse => {
            *index += 1;
            Node::Expr { e: Box::new(Node::BoolFalse) }
        },
        _ => {
            panic!(); // TODO:
        }
    }
}

fn parse_bin_func_call_expr(index: &mut usize, tokens: &Vec<Token>, func_call: Node) -> Node {
    match tokens[*index] {
        Token::Plus => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(func_call), 
                op: BinOp::Add, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Minus => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(func_call), 
                op: BinOp::Sub, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Star => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(func_call), 
                op: BinOp::Mul, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Slash => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(func_call), 
                op: BinOp::Div, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Semicolon => {
            *index += 1;
            return func_call;
        },
        Token::Comma | Token::RP => {
            return func_call;
        },
        _ => {
            panic!(); // TODO:
        }
    }
}

fn parse_bin_var_expr(index: &mut usize, tokens: &Vec<Token>) -> Node {
    let var_name = match &tokens[*index] {
        Token::Id { tok_lexeme } => {
            tok_lexeme
        }
        _ => {
            panic!("Compiler message: hz");
        }
    };

    *index += 1;
    match tokens[*index] {
        Token::Plus => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Add, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Minus => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Sub, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Star => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Mul, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Slash => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Div, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::UnAdd => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::UnAdd, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::UnSub => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::UnSub, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::UnMul => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::UnMul, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::UnDiv => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::UnDiv, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Assignment => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Assignment, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Equal => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Equal, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::NotEqual => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::NotEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Lesser => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Lesser, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Bigger => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::Bigger, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::LesserEqual => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::LesserEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::BiggerEqual => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) }), 
                op: BinOp::BiggerEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Negation => {
            *index += 1;
            return Node::UnE { op: UnOp::Negation, expr: Box::new(parse_expr(index, tokens)) }
        },
        Token::Inc => {
            *index += 1;
            return Node::UnE { op: UnOp::Inc, expr: Box::new(parse_expr(index, tokens)) }
        },
        Token::Dec => {
            *index += 1;
            return Node::UnE { op: UnOp::Dec, expr: Box::new(parse_expr(index, tokens)) }
        },
        Token::Semicolon => {
            *index += 1;
            return Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) };
        },
        Token::Comma | Token::RP => {
            return Node::Var { name: Box::new(Node::Id { id: var_name.to_string() }) };
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
        Token::Equal => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::IntN { n: num }), 
                op: BinOp::Equal, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::NotEqual => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::IntN { n: num }), 
                op: BinOp::NotEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Lesser => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::IntN { n: num }), 
                op: BinOp::Lesser, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Bigger => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::IntN { n: num }), 
                op: BinOp::Bigger, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::LesserEqual => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::IntN { n: num }), 
                op: BinOp::LesserEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::BiggerEqual => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::IntN { n: num }), 
                op: BinOp::BiggerEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Semicolon => {
            *index += 1;
            return Node::Expr { e: Box::new(Node::IntN { n: num }) };
        },
        Token::Comma | Token::RP => {
            return Node::Expr { e: Box::new(Node::IntN { n: num }) };
        },
        _ => {
            panic!("Compiler message: found {:?}.", tokens[*index]); // TODO:
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
        Token::Equal => {
            *index += 1;
            return Node::BinE { 
                exprl: Box::new(Node::FloatN { n: num }), 
                op: BinOp::Equal, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::NotEqual => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::FloatN { n: num }), 
                op: BinOp::NotEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Lesser => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::FloatN { n: num }), 
                op: BinOp::Lesser, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Bigger => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::FloatN { n: num }), 
                op: BinOp::Bigger, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::LesserEqual => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::FloatN { n: num }), 
                op: BinOp::LesserEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::BiggerEqual => {
            *index += 1;
            return Node::BinE {  
                exprl: Box::new(Node::FloatN { n: num }), 
                op: BinOp::BiggerEqual, 
                exprr: Box::new(parse_expr(index, tokens)) 
            };
        },
        Token::Semicolon => {
            *index += 1;
            return Node::Expr { e: Box::new(Node::FloatN { n: num }) };
        },
        Token::Comma | Token::RP => {
            return Node::Expr { e: Box::new(Node::FloatN { n: num }) };
        },
        _ => {
            panic!(); // TODO:
        }
    }
}
