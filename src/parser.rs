use egui::TextBuffer;
use logos::Logos;
use std::cmp::PartialEq;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("^")]
    Caret,
    #[token("%")]
    Modulus,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("x")]
    VarX,
    #[token(",")]
    Comma,
    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),
    #[regex("[a-zA-Z]+\\(", |lex| lex.slice().to_owned(), priority = 3)]
    FuncCall(String),
}

impl Token {
    fn is_operator(&self) -> bool {
        match self {
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Caret
            | Token::Modulus => true,
            _ => false,
        }
    }
}

fn prec(token: &Token) -> i64 {
    match token {
        Token::Plus => 1,
        Token::Minus => 1,
        Token::Star => 2,
        Token::Slash => 2,
        Token::Modulus => 2,
        Token::Caret => 3,
        _ => -1,
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Associativity {
    Left,
    Right,
}

fn associativity(token: &Token) -> Associativity {
    match token {
        Token::Caret => Associativity::Right,
        _ => Associativity::Left,
    }
}

fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut result = vec![];
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token.clone() {
            Token::Int(_) | Token::VarX => {
                result.push(token);
            }
            Token::FuncCall(_) => {
                stack.push(token)
            }
            Token::LeftParen => {
                stack.push(token);
            }
            Token::RightParen => {
                while !stack.is_empty() && *stack.last().unwrap() != Token::LeftParen && *stack.last().unwrap() != Token::Comma {
                    result.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            _ => {
                while !stack.is_empty()
                    && (prec(&token) < prec(stack.last().unwrap())
                        || prec(&token) == prec(stack.last().unwrap())
                            && associativity(&token) == Associativity::Left)
                {
                    result.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
        }
    }

    while !stack.is_empty() {
        result.push(stack.pop().unwrap());
    }

    result
}

fn parse(expr: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut lexer = Token::lexer(expr);
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => tokens.push(token),
            _ => panic!("unexpected token `{}`", lexer.slice()),
        }
    }

    tokens
}

fn evaluate_postfix(tokens: Vec<Token>) -> i64 {
    let mut stack: Vec<i64> = vec![];

    for token in tokens {
        let mut plus: Option<i64> = None;
        if let Token::Int(int_val) = token {
            stack.push(int_val);
        } else if !stack.is_empty() {
            if token == Token::Plus {
                let value = stack.pop().unwrap();
                plus = Some(stack.pop().unwrap() + value);
            } else if token == Token::Minus {
                let value = stack.pop().unwrap();
                plus = Some(stack.pop().unwrap() - value);
            } else if token == Token::Star {
                let value = stack.pop().unwrap();
                plus = Some(stack.pop().unwrap() * value);
            } else if token == Token::Slash {
                let value = stack.pop().unwrap();
                plus = Some(stack.pop().unwrap() / value);
            } else if token == Token::Modulus {
                let value = stack.pop().unwrap();
                plus = Some(stack.pop().unwrap() % value);
            } else if token == Token::Caret {
                let value = stack.pop().unwrap().try_into().unwrap();
                plus = Some(stack.pop().unwrap().pow(value));
            }
            if let Some(val) = plus {
                stack.push(val);
            } else {
                unimplemented!("{token:?}");
            }
        }
    }

    stack.pop().unwrap()
}

fn evaluate_with_var(expr: &str, x: i64) -> i64 {
    let tokens = parse(expr)
        .iter()
        .map(|token| match token {
            Token::VarX => Token::Int(x),
            _ => token.clone(),
        })
        .collect();
    let postfix = infix_to_postfix(tokens);
    evaluate_postfix(postfix)
}

#[cfg(test)]
mod tests {
    use crate::parser::evaluate_with_var;

    #[test]
    fn test_parser() {
        let expr = "2^2*2";
        println!("{:?}", evaluate_with_var(expr, 10));
    }
}
