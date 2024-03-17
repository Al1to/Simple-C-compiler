#[derive(Debug)]
pub enum Token {
    IntT, FloatT, CharT, VoidT,
    IntN { num: i32 }, FloatN { num: f64 }, CharN { num: u8 },
    If, Else,
    While,
    LP, RP,
    LC, RC,
    LB, RB,
    Semicolon, Comma, Assignment,
    Plus, Minus, Slash, Star, 
    Bigger, Lesser, Equal,
    LesserEqual, BiggerEqual,
    Inc, Dec, 
    UnAdd, UnSub, UnMul, UnDiv,
    Id { tok_lexeme: String },
}

pub fn lex(code: &String, tokens: &mut Vec<Token>) {
    let mut chars = code.chars().peekable();
    while let Some(curr_char) = chars.next() {
        if curr_char.is_whitespace() {
            continue;
        }

        match curr_char {
            '(' => tokens.push(Token::LP),
            ')' => tokens.push(Token::RP),
            '{' => tokens.push(Token::LC),
            '}' => tokens.push(Token::RC),
            '[' => tokens.push(Token::LB),
            ']' => tokens.push(Token::RB),
            ';' => tokens.push(Token::Semicolon),
            ',' => tokens.push(Token::Comma),
            '=' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::Equal);
                    },
                    _ => {
                        tokens.push(Token::Assignment);
                    }
                };
            },
            '>' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::BiggerEqual);
                    },
                    _ => {
                        tokens.push(Token::Bigger);
                    }
                };
            },
            '<' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::LesserEqual);
                    },
                    _ => {
                        tokens.push(Token::Lesser);
                    }
                };
            },
            '!' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::LesserEqual);
                    },
                    Some(char) if !char.is_whitespace() && !char.is_alphanumeric() => {
                        panic!("Compiler message: What's that '!{}' operator?", char);
                    }
                    _ => {
                        panic!("Compiler message: Wha? End of input after '!'.");
                    }
                };
            },
            '+' => {
                match chars.peek() {
                    Some(&'+') => {
                        chars.next();
                        tokens.push(Token::Inc);
                    },
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::UnAdd);
                    },
                    Some(char) if !char.is_whitespace() && !char.is_alphanumeric() => {
                        panic!("Compiler message: What's that '+{}' operator?", char);
                    }
                    _ => {
                        tokens.push(Token::Plus);
                    }
                };
            },
            '-' => {
                match chars.peek() {
                    Some(&'+') => {
                        chars.next();
                        tokens.push(Token::Dec);
                    },
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::UnSub);
                    },
                    Some(char) if !char.is_whitespace() && !char.is_alphanumeric() => {
                        panic!("Compiler message: What's that '-{}' operator?", char);
                    }
                    _ => {
                        tokens.push(Token::Minus);
                    }
                };
            },
            '/' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::UnDiv);
                    },
                    Some(char) if !char.is_whitespace() && !char.is_alphanumeric() => {
                        panic!("Compiler message: What's that '/{}' operator?", char);
                    }
                    _ => {
                        tokens.push(Token::Slash);
                    }
                };
            },
            '*' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::UnMul);
                    },
                    Some(char) if !char.is_whitespace() && !char.is_alphanumeric() => {
                        panic!("Compiler message: What's that '*{}' operator?", char);
                    }
                    _ => {
                        tokens.push(Token::Star);
                    }
                };
            },
            
            _ if curr_char.is_alphabetic() => {
                let mut lexeme = String::new();
                lexeme.push(curr_char);

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        lexeme.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match lexeme.as_str() {
                    "int" => tokens.push(Token::IntT),
                    "float" => tokens.push(Token::FloatT),
                    "char" => tokens.push(Token::CharT),
                    "void" => tokens.push(Token::VoidT),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "while" => tokens.push(Token::While),
                    _ => tokens.push(Token::Id { tok_lexeme: lexeme }),
                };
            }

            _ if curr_char.is_numeric() => {
                let mut point_count: u8 = 0;
                let mut lexeme = String::new();
                lexeme.push(curr_char);

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_numeric() {
                        lexeme.push(next_char);
                        chars.next();
                    } else if next_char == '.' {
                        point_count += 1;
                        lexeme.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
            
                if point_count > 1 {
                    panic!("Compiler message: Wtf? {} has more than one decimal point.", lexeme);
                }
            
                if point_count == 0 {
                    match lexeme.parse::<i32>() {
                        Ok(n) => {
                            tokens.push(Token::IntN { num: n });
                        }
                        Err(_) => {
                            panic!("Compiler message: Are you an idiot? You have letters in a decimal int number {}.", lexeme);
                        }
                    };
                } else {
                    match lexeme.parse::<f64>() {
                        Ok(n) => {
                            tokens.push(Token::FloatN { num: n });
                        }
                        Err(_) => {
                            panic!("Compiler message: Are you an idiot? You have letters in a decimal float number {}.", lexeme);
                        }
                    };
                }
            }

            _ if curr_char == '\'' => {
                let char: String = chars.by_ref().take_while(|&c| c.is_ascii() && c != '\'').collect();
                if char.len() != 1 {
                    panic!("Compiler message: What the hell? Do you even know C syntax? Your char length is greater than 1: '{}'.", char);
                }
                tokens.push(Token::CharN { num: *char.as_bytes().get(0).unwrap() });
            }

            _ => {
                panic!("Compiler message: What's that '{}' symbol?", curr_char);
            }
        };
    }
}
