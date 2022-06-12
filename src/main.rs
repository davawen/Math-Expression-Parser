use core::fmt;
use std::{
    error,
    collections::HashMap
};
use thiserror::Error;

mod inspect_err;

use inspect_err::InspectErr;

type ResultDyn<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy)]
enum OpType {
    Plus,
    Sub,
    Mul,
    Div
}

impl OpType {
    /// Return operator precendence, where a higher precedence means computation happens earlier
    fn precedence(&self) -> i32 {
        use OpType::*;

        match *self {
            Plus => 1,
            Sub => 1,
            Mul => 2,
            Div => 2
        }
    }
}

#[derive(Debug)]
struct Op {
    op: OpType,
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

type FunType = fn(f64) -> f64;

struct Function {
    arg: Box<Expr>,
    fun: FunType
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("arg", &self.arg)
            .finish()
    }
}

impl Op {
    fn new(op: OpType, lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Op {
            op, lhs, rhs
        }
    }
}

#[derive(Debug)]
enum Expr {
    Value(f64),
    Variable(String),
    Operation(Op),
    Function(Function)
}

impl Expr {
    fn calc(&self, variables: &HashMap<String, f64>) -> f64 {
        match self {
            Expr::Value(v) => *v,
            Expr::Variable(v) => variables[v],
            Expr::Operation(op) => {
                let lhs = op.lhs.calc(variables);
                let rhs = op.rhs.calc(variables);

                use OpType::*;

                match op.op {
                    Plus => lhs + rhs,
                    Sub => lhs - rhs,
                    Mul => lhs * rhs,
                    Div => lhs / rhs
                }
            },
            Expr::Function(fun) => {
                (fun.fun)( fun.arg.calc(variables) )
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Op(OpType),
    Identifier(String),
    LeftParen,
    RightParen
}

fn tokenize(str: &str) -> ResultDyn<Vec<Token>> {
    // 9+5

    let mut tokens = Vec::new();

    let mut chars = str.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            let mut radix: u32 = 10;

            let mut sum: f64;
            
            // Use prefix to indicate base
            if c == '0' {
                match chars.peek().unwrap() {
                    'x' | 'X' => radix = 16,
                    'o' | 'O' => radix = 8,
                    'b' | 'B' => radix = 2,
                    _ => ()
                }

                if radix != 10 {
                    // chars.advance_by(1) Nightly feature
                    chars.next().unwrap_or_default();
                }

                sum = 0.0;
            }
            else {
                sum = c.to_digit(radix).unwrap().try_into()?;
            }
            


            while let Some(n) = chars.peek() {
                if n.is_digit(radix) {
                    let n = chars.next().unwrap();

                    sum *= f64::from(radix);
                    sum += f64::from(n.to_digit(radix).unwrap());
                }
                else {
                    break;
                }
            }

            tokens.push(Token::Number(sum));
        }
        else {
            use OpType::*;

            match c {
                '+' => tokens.push( Token::Op( Plus ) ),
                '-' => tokens.push( Token::Op( Sub ) ),
                '*' => tokens.push( Token::Op( Mul ) ),
                '/' => tokens.push( Token::Op( Div ) ),
                '(' => tokens.push( Token::LeftParen ),
                ')' => tokens.push( Token::RightParen ),
                c if c.is_ascii_alphabetic() => {
                    let mut id = String::new();
                    id.push(c);

                    while let Some(c) = chars.peek() {
                        if c.is_ascii_alphabetic() {
                            id.push( chars.next().unwrap() );
                        }
                        else {
                            break;
                        }
                    }

                    tokens.push( Token::Identifier(id) );
                },
                _ => ()
            }
        }
        
    }

    Ok(tokens)
}

#[derive(Error, Debug, Clone)]
enum MalformedExpressionError {
    #[error("Uknown function `{0}`")]
    UknownFunction(String),
    #[error("Malformed expression, expected {expected}, found token {found:?}")]
    InvalidToken {
        expected: &'static str,
        found: Option<Token>
    },
    #[error("Couldn't find matching token to {start:?}, found {end:?}")]
    NoMatchingToken {
        start: Token,
        end: Option<Token>
    },
    #[error("Couldn't find a valid action at tokens {0:?}")]
    AmbiguousOperation(Vec<Token>)
}

trait AdvanceToMatchingParen 
where Self: std::marker::Sized {
    fn advance_to_matching_paren(self) -> Result<Self, MalformedExpressionError>;
}

macro_rules! advanceable_to_matching_paren {
    ($($ty: ty), +) => {
        $(
            impl AdvanceToMatchingParen for $ty {
                /// Constructs an iterator that returns the matching parenthesise
                fn advance_to_matching_paren(mut self) -> Result<Self, MalformedExpressionError> {
                    let mut scope = 0;

                    let mut last_token: Option<&Token> = None;

                    for t in &mut self {
                        match t {
                            Token::LeftParen => scope += 1,
                            Token::RightParen => scope -= 1,
                            _ => ()
                        }

                        last_token = Some(t);

                        if scope < 0 { break; }
                    }

                    if scope >= 0 {
                        Err(MalformedExpressionError::NoMatchingToken { start: Token::LeftParen, end: last_token.cloned() })
                    }
                    else {
                        Ok(self)
                    }
                }
            }
        )+
    }
}

advanceable_to_matching_paren!(std::slice::Iter<'_, Token>, std::iter::Skip<std::slice::Iter<'_, Token>>);

impl AdvanceToMatchingParen for std::iter::Enumerate<std::slice::Iter<'_, Token>> {
    fn advance_to_matching_paren(mut self) -> Result<Self, MalformedExpressionError> {
        let mut scope = 0;

        let mut last_token: Option<&Token> = None;

        for (_, t) in &mut self {
            match t {
                Token::LeftParen => scope += 1,
                Token::RightParen => scope -= 1,
                _ => ()
            }

            last_token = Some(t);

            if scope < 0 { break; }
        }

        if scope >= 0 {
            Err(MalformedExpressionError::NoMatchingToken { start: Token::LeftParen, end: last_token.cloned() })
        }
        else {
            Ok(self)
        }
    }
}

fn parse(tokens: &[Token], functions: &HashMap<String, FunType>) -> Result<Expr, MalformedExpressionError> {
    println!("{:?}", tokens);

    if tokens.len() == 1 {
        match tokens.first() {
            Some(Token::Number(num)) => Ok(Expr::Value(*num)),
            Some(Token::Identifier(v)) => Ok(Expr::Variable(v.clone())),
            _ => Err(MalformedExpressionError::InvalidToken { 
                expected: "Token::Number | Token::Identifier", 
                found: tokens.first().cloned()
            })
        }
    }
    else {
        // If slice is wrapped in parenthesises, remove them
        // First check if it starts with one
        if let Some(Token::LeftParen) = tokens.first() {

            // Then check if the matching parenthesis is at the end of the slice
            if tokens.iter().skip(1).advance_to_matching_paren()?.next().is_none() {
                return parse( &tokens[1..tokens.len()-1], functions );
            }
        }

        // Find operator with lowest precedence, and split expression here
        let ( mut idx, mut precedence ) = ( 0_usize, i32::MAX );

        let mut it = tokens.iter().enumerate();

        while let Some(t) = it.next() {
            match t.1 {
                Token::Op(op) => {
                    if op.precedence() <= precedence {
                        idx = t.0;
                        precedence = op.precedence();
                    }
                },
                // By definition, parenthesises have a higher precedence, so we'll skip them
                Token::LeftParen => it = it.advance_to_matching_paren()?,
                _ => ()
            }
        }

        // If no operator was found, search for functions
        if precedence == i32::MAX {
            let mut it = tokens.iter();
            
            if let ( Some(Token::Identifier(id)), Some(Token::LeftParen), Some(Token::RightParen) ) = ( it.next(), it.next(), it.last() ) {
                if let Some(fun) = functions.get(id) {
                    Ok(Expr::Function(Function {
                        arg: Box::new( parse( &tokens[2..tokens.len()-1], functions )? ),
                        fun: *fun
                    }))
                }
                else {
                    Err(MalformedExpressionError::UknownFunction(id.clone()))
                }
            }
            else {
                Err(MalformedExpressionError::AmbiguousOperation(tokens.to_vec()))
            }
        }
        else if let Token::Op(op) = tokens[idx] {
            Ok(Expr::Operation(Op::new(
                op,
                Box::new(parse(&tokens[..idx], functions)?),
                Box::new(parse(&tokens[(idx+1)..], functions)?)
            )))
        }
        else { unreachable!() }
    }
}

fn main() -> ResultDyn<()> {

    let input = std::env::args().skip(1).reduce(|a, x|{ a + " " + &x }).unwrap();

    println!("> Input\n{}", input);

    let tokens = tokenize(&input)?;

    println!("> Tokens\n{:#?}", tokens);

    let functions: HashMap<String, FunType> = HashMap::from([
        ("cos".into(), f64::cos as FunType),
        ("sin".into(), f64::sin),
        ("tan".into(), f64::tan),
        ("add".into(), |x|{ x + 1.0 })
    ]);

    let mut variables: HashMap<String, f64> = HashMap::from([
        ("x".into(), 0.0)
    ]);

    #[allow(unstable_name_collisions)]
    let parsed = parse(&tokens, &functions)
        .inspect_err(|e|{ println!("{}", e) })
        .unwrap();

    println!("> AST\n{:#?}\n> Value", parsed);

    for i in 0..=10 {
        *variables.get_mut("x").unwrap() = f64::from(i);
        println!("{}", parsed.calc(&variables));
    }

    // match parsed {
    //     Ok(parsed) => println!("> AST\n{:#?}\n> Value\n{}", parsed, parsed.calc()),
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // }


    Ok(())
}
