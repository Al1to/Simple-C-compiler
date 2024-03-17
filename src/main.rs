// use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io;

mod lex {
    pub mod lexer;
}

mod parse {
    pub mod parser;
}

// mod byte_code_gen {
//     pub mod byte_code_generator;
// }

use crate::lex::lexer::lex;
use crate::lex::lexer::Token;
use crate::parse::parser::parse;
use crate::parse::parser::Node;
// use crate::byte_code_gen::byte_code_generator::gen;

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     return Ok(());
    // } else {
        // let in_path = &args[1];
        let in_path = &String::from("./test.c");
        let mut in_file = File::open(in_path)?;
        let mut code = String::new(); 
        in_file.read_to_string(&mut code)?;

        // let out_path = &args[2];
        // let mut out_file = File::create(out_path)?;
        // out_file.write_all(gen(parse(lex(&code))))?;
        let mut tokens: Vec<Token> = vec![]; 
        lex(&code, &mut tokens);
        println!("{:?}", tokens);

        let mut ast: Vec<Node> = vec![];
        parse(&tokens, &mut ast);
        println!("{:?}", ast);
    // }

    Ok(())
}
