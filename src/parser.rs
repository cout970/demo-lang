use std::collections::VecDeque;

use crate::ast::{Expression, Operator, Program, Statement, TypeDef, TypeDefVariant, UnaryOperator, Variable};
use crate::source::Span;
use crate::tokenizer::{Token, Tokenizer, TokenSpan};

#[derive(Debug, Clone)]
pub enum ParseError {
    Expected { expected: Token, found: Token, span: TokenSpan },
    ExpectedId { found: Token, span: TokenSpan },
    UnexpectedToken(Token, TokenSpan),
    EOF,
}

pub struct Parser {
    tk: Tokenizer,
    lookahead: VecDeque<(Token, TokenSpan)>,
    eof: Token,
}

impl Parser {
    pub fn new(tk: Tokenizer) -> Self {
        Self { tk, lookahead: VecDeque::new(), eof: Token::Eof }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        parse_program(self)
    }

    fn current(&mut self) -> &Token {
        if self.lookahead.is_empty() {
            self.lookahead.push_back(self.tk.next());
        }

        self.lookahead.get(0)
            .map(|it| &it.0)
            .unwrap_or(&self.eof)
    }

    fn pop(&mut self) -> (Token, TokenSpan) {
        if self.lookahead.is_empty() {
            self.lookahead.push_back(self.tk.next());
        }

        self.lookahead.pop_front()
            .unwrap_or((Token::Eof, (Span { line: 0, column: 0 }, Span { line: 0, column: 0 })))
    }

    fn current_pos(&mut self) -> TokenSpan {
        if self.lookahead.is_empty() {
            self.lookahead.push_back(self.tk.next());
        }

        self.lookahead.get(0)
            .map(|it| it.1.clone())
            .unwrap_or((Span { line: 0, column: 0 }, Span { line: 0, column: 0 }))
    }

    fn at(&mut self, offset: i32) -> &Token {
        let index = offset as usize;

        while self.lookahead.len() <= index {
            self.lookahead.push_back(self.tk.next());
        }

        self.lookahead.get(index)
            .map(|it| &it.0)
            .unwrap_or(&self.eof)
    }

    fn next(&mut self) {
        if !self.lookahead.is_empty() {
            self.lookahead.pop_front();
        }
    }

    fn expect(&mut self, tk: Token) -> Result<(), ParseError> {
        if self.current() != &tk {
            return Err(ParseError::Expected {
                expected: tk,
                found: self.current().clone(),
                span: self.current_pos(),
            });
        }

        self.next();
        Ok(())
    }

    fn skip(&mut self, tk: Token) -> bool {
        if self.current() != &tk {
            return false;
        }

        self.next();
        true
    }

    fn expect_id(&mut self) -> Result<String, ParseError> {
        let (tk, span) = self.pop();

        if let Token::Identifier(name) = tk {
            Ok(name)
        } else {
            Err(ParseError::ExpectedId { found: tk, span })
        }
    }
}

pub fn parse_program(p: &mut Parser) -> Result<Program, ParseError> {
    let mut statements = vec![];

    while p.current() != &Token::Eof {
        statements.push(parse_statement(p)?);
        p.skip(Token::Semicolon);
    }

    Ok(Program { statements })
}

pub fn parse_statement(p: &mut Parser) -> Result<Statement, ParseError> {
    while p.current() == &Token::Semicolon {
        p.next();
    }

    if let Token::Identifier(_) = p.at(0) {
        if let Token::Assign = p.at(1) {
            return parse_variable(p).map(|i| Statement::Variable(i));
        }
    }

    if let Token::Typedef = p.at(0) {
        return parse_typedef(p).map(|i| Statement::TypeDef(i));
    }

    parse_expression(p).map(|i| Statement::Expression(i))
}

pub fn parse_variable(p: &mut Parser) -> Result<Variable, ParseError> {
    let name = p.expect_id()?;
    p.expect(Token::Assign)?;
    let value = parse_expression(p)?;

    Ok(Variable { name, value })
}

pub fn parse_typedef(p: &mut Parser) -> Result<TypeDef, ParseError> {
    p.expect(Token::Typedef)?;
    let name = p.expect_id()?;
    p.expect(Token::Assign)?;
    let mut variants = vec![];

    loop {
        let variant = parse_typedef_variant(p)?;
        variants.push(variant);

        if p.current() != &Token::Pipe {
            break;
        }
        p.expect(Token::Pipe)?;
    }

    Ok(TypeDef { name, variants })
}

pub fn parse_typedef_variant(p: &mut Parser) -> Result<TypeDefVariant, ParseError> {
    let name = p.expect_id()?;
    let mut properties = vec![];

    if p.skip(Token::LeftParen) {
        loop {
            let prop = p.expect_id()?;
            properties.push(prop);

            match p.current() {
                Token::Comma => {
                    p.next();
                    if p.skip(Token::RightParen) {
                        break;
                    }
                }
                Token::RightParen => {
                    p.next();
                    break;
                }
                _ => {
                    return Err(ParseError::Expected { expected: Token::RightParen, found: p.current().clone(), span: p.current_pos() });
                }
            }
        }
    }

    Ok(TypeDefVariant { name, properties })
}

pub fn parse_expression(p: &mut Parser) -> Result<Expression, ParseError> {
    parse_expression_6(p)
}

pub fn parse_expression_6(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_5(p)?;
    loop {
        let op = match p.current() {
            Token::And => Operator::And,
            Token::Or => Operator::Or,
            Token::Xor => Operator::Xor,
            _ => { break; }
        };

        p.next();
        let right = parse_expression_5(p)?;

        expr = Expression::Operator {
            operator: op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }

    Ok(expr)
}

pub fn parse_expression_5(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_4(p)?;
    loop {
        let op = match p.current() {
            Token::Equals => Operator::Equals,
            Token::NotEquals => Operator::NotEquals,
            _ => { break; }
        };

        p.next();
        let right = parse_expression_4(p)?;

        expr = Expression::Operator {
            operator: op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }

    Ok(expr)
}

pub fn parse_expression_4(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_3(p)?;
    loop {
        let op = match p.current() {
            Token::Less => Operator::Less,
            Token::Greater => Operator::Greater,
            Token::LessEquals => Operator::LessEquals,
            Token::GreaterEquals => Operator::GreaterEquals,
            _ => { break; }
        };

        p.next();
        let right = parse_expression_3(p)?;

        expr = Expression::Operator {
            operator: op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }

    Ok(expr)
}

pub fn parse_expression_3(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_2(p)?;
    loop {
        let op = match p.current() {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            _ => { break; }
        };

        p.next();
        let right = parse_expression_2(p)?;

        expr = Expression::Operator {
            operator: op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }

    Ok(expr)
}

pub fn parse_expression_2(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_1(p)?;
    loop {
        let op = match p.current() {
            Token::Times => Operator::Times,
            Token::Div => Operator::Div,
            Token::Percent => Operator::Rem,
            _ => { break; }
        };

        p.next();
        let right = parse_expression_1(p)?;

        expr = Expression::Operator {
            operator: op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }

    Ok(expr)
}

pub fn parse_expression_1(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_0(p)?;
    loop {
        let op = match p.current() {
            Token::Ampersand => Operator::BiteAnd,
            Token::Pipe => Operator::BiteOr,
            _ => { break; }
        };

        p.next();
        let right = parse_expression_0(p)?;

        expr = Expression::Operator {
            operator: op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }

    Ok(expr)
}

pub fn parse_expression_0(p: &mut Parser) -> Result<Expression, ParseError> {
    let mut expr = parse_expression_base(p)?;
    loop {
        if !p.skip(Token::Dot) {
            break;
        }

        let mut args = vec![expr];
        let name = p.expect_id()?;

        while p.current() != &Token::Dot && p.current() != &Token::Eof && expression_first(p) {
            args.push(parse_expression(p)?);
        }

        expr = Expression::FunCall { name, args };
    }

    Ok(expr)
}

pub fn parse_expression_base(p: &mut Parser) -> Result<Expression, ParseError> {
    let (token, span) = p.pop();

    let expr = match token {
        Token::Minus => {
            let expr = parse_expression(p)?;
            Expression::UnaryOperator { operator: UnaryOperator::Minus, expr: Box::new(expr) }
        }
        Token::Plus => {
            let expr = parse_expression(p)?;
            Expression::UnaryOperator { operator: UnaryOperator::Plus, expr: Box::new(expr) }
        }
        Token::Not => {
            let expr = parse_expression(p)?;
            Expression::UnaryOperator { operator: UnaryOperator::Not, expr: Box::new(expr) }
        }
        Token::IntLiteral(text) => {
            Expression::Int { value: text.parse::<i32>().unwrap() }
        }
        Token::FloatLiteral(text) => {
            Expression::Float { value: text.parse::<f32>().unwrap() }
        }
        Token::StringLiteral(text) => {
            Expression::String { value: text }
        }
        Token::Identifier(name) => {
            let mut args = vec![];

            while p.current() != &Token::Dot && p.current() != &Token::Eof && expression_first(p) {
                args.push(parse_expression(p)?);
                if !p.skip(Token::Comma) {
                    break;
                }
            }

            Expression::FunCall { name, args }
            // let expr = parse_expression(e)?;
        }
        Token::Return => {
            Expression::Return { value: Box::new(parse_expression(p)?) }
        }
        Token::LeftBrace => { // {
            // Lambda
            let mut args = vec![];
            let mut code = vec![];

            // let mut assume_args = true;

            let mut index = 0;
            loop {
                let next = p.at(index);
                index += 1;
                if let Token::Identifier(name) = next {
                    args.push(name.clone());

                    let sep = p.at(index);
                    index += 1;

                    if sep == &Token::Pipe {
                        for _ in 0..index {
                            p.next();
                        }
                        break;
                    } else if sep != &Token::Comma {
                        args.clear();
                        break;
                    }
                } else {
                    args.clear();
                    break;
                }
            }

            while p.current() != &Token::RightBrace {
                if p.current() == &Token::Eof { return Err(ParseError::EOF); }

                let stm = parse_statement(p)?;
                code.push(stm);

                if p.current() == &Token::Comma {
                    p.next();
                }
            }
            p.next();
            Expression::Lambda { args, code }
        }
        Token::LeftParen => { // (
            // Tuple
            let mut values = vec![];

            while p.current() != &Token::RightParen {
                if p.current() == &Token::Eof { return Err(ParseError::EOF); }

                let expr = parse_expression(p)?;
                values.push(expr);

                if p.current() == &Token::Comma {
                    p.next();
                }
            }
            p.next();
            if values.len() == 1 {
                values.into_iter().next().unwrap()
            } else {
                Expression::Tuple { values }
            }
        }
        Token::LeftBracket => { // [
            // List
            let mut items = vec![];

            while p.current() != &Token::RightBracket {
                if p.current() == &Token::Eof { return Err(ParseError::EOF); }

                let expr = parse_expression(p)?;
                items.push(expr);

                if p.current() == &Token::Comma {
                    p.next();
                }
            }
            p.next();
            Expression::List { items }
        }
        it => {
            return Err(ParseError::UnexpectedToken(it, span));
        }
    };

    Ok(expr)
}

fn expression_first(p: &mut Parser) -> bool {
    match p.current() {
        Token::IntLiteral(_) |
        Token::FloatLiteral(_) |
        Token::StringLiteral(_) |
        Token::Identifier(_) |
        Token::Minus |
        Token::Plus |
        Token::Not |
        Token::Return |
        Token::LeftBrace |
        Token::LeftParen |
        Token::LeftBracket => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use crate::source::{CodeSource, SourceReader};

    use super::*;

    fn parse(code: &'static str) -> Parser {
        let source = CodeSource::str(code);
        let reader = SourceReader::new(source);
        let tokenizer = Tokenizer::new(reader);

        Parser::new(tokenizer)
    }

    #[test]
    fn list() {
        let mut p = parse("[10, 56.7, \"abc\", 123, (1 2 3 4 5), (1)]");
        let stm = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", stm);
    }

    #[test]
    fn lambda_with_args() {
        let mut p = parse("{ a, b, c | a * b + c }");
        let exp = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn lambda_without_args() {
        let mut p = parse("{ a * b + c }");
        let exp = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn tuple() {
        let mut p = parse("(1, \"2\", 3.4)");
        let exp = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn variable() {
        let mut p = parse("pi = 3.14");
        let exp = parse_statement(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn function_void() {
        let mut p = parse("hello = { print \"hello\" }");
        let exp = parse_statement(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn function_1arg() {
        let mut p = parse("hello = { name | print \"hello \"; print name }");
        let exp = parse_statement(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn typedef() {
        let mut p = parse("typedef Bool = True | False");
        let exp = parse_statement(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn typedef2() {
        let mut p = parse("typedef Result = Ok(value) | Err(error)");
        let exp = parse_statement(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn typedef3() {
        let mut p = parse("typedef User = User(name, email, password_hash, age)");
        let exp = parse_statement(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn operator_precedence() {
        let mut p = parse("1 | 2 * 3 + 4 < 5 == 6 && 7");
        let exp = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn operator_precedence2() {
        let mut p = parse("7 && 6 == 5 < 4 + 3 * 2 | 1");
        let exp = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }

    #[test]
    fn dot_notation() {
        let mut p = parse("[1, 2, 3, 4].map { it * it }");
        let exp = parse_expression(&mut p).expect("ParseError");
        println!("{:#?}", exp);
    }
}