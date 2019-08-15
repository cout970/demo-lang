use crate::source::{SourceReader, Span};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    FloatingLiteral(String),
    IntegerLiteral(String),
    StringLiteral(String),
    // Keywords
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    LeftAssign,
    RightAssign,
    LeftShift,
    RightShift,
    Ellipsis,
    Tilde,
    QuestionMark,
    Semicolon,
    Equals,
    Assign,
    Colon,
    Comma,
    NotEquals,
    Not,
    At,
    Hash,
    Dollar,
    Percent,
    Xor,
    Ampersand,
    And,
    Or,
    Times,
    Div,
    Minus,
    MinusMinus,
    Plus,
    PlusPlus,
    Pointer,
    Pipe,
    PercentAssign,
    XorAssign,
    AndAssign,
    DivAssign,
    TimesAssign,
    MinusAssign,
    PlusAssign,
    OrAssign,
    Dot,
    // End of file
    Eof,
    Error(char, Span),
}

pub type TokenSpan = (Span, Span);

struct Tokenizer {
    read: SourceReader,
}

impl Tokenizer {
    pub fn new(reader: SourceReader) -> Self {
        Tokenizer { read: reader }
    }

    pub fn next_tk(&mut self) -> Token {
        self.next().0
    }

    fn produce(&mut self, tk: Token) -> Token {
        self.read.shift();
        tk
    }

    pub fn next(&mut self) -> (Token, TokenSpan) {
        self.trim_spaces();
        self.trim_comments();
        let start = self.read.span();
        let ty = match self.read.current() {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier(),
            b'0'..=b'9' => self.read_number(),
            b'.' => {
                if let b'0'..=b'9' = self.read.next() {
                    self.read_number()
                } else if self.read.next() == b'.' && self.read.next_next() == b'.' {
                    self.read.shift_multiple(2);
                    self.produce(Token::Ellipsis)
                } else {
                    self.produce(Token::Dot)
                }
            }
            b'\"' => {
                self.read_string()
            }
            b'(' => self.produce(Token::LeftParen),
            b')' => self.produce(Token::RightParen),
            b'{' => self.produce(Token::LeftBrace),
            b'}' => self.produce(Token::RightBrace),
            b'[' => self.produce(Token::LeftBracket),
            b']' => self.produce(Token::RightBracket),
            b'~' => self.produce(Token::Tilde),
            b'?' => self.produce(Token::QuestionMark),
            b'<' => {
                if self.read.next() == b'<' && self.read.next_next() == b'=' {
                    self.read.shift_multiple(2);
                    self.produce(Token::LeftAssign)
                } else if self.read.next() == b'<' {
                    self.read.shift();
                    self.produce(Token::LeftShift)
                } else if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::LessEquals)
                } else {
                    self.produce(Token::Less)
                }
            }
            b'>' => {
                if self.read.next() == b'>' && self.read.next_next() == b'=' {
                    self.read.shift_multiple(2);
                    self.produce(Token::RightAssign)
                } else if self.read.next() == b'>' {
                    self.read.shift();
                    self.produce(Token::RightShift)
                } else if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::GreaterEquals)
                } else {
                    self.produce(Token::Greater)
                }
            }
            b';' => self.produce(Token::Semicolon),
            b':' => self.produce(Token::Colon),
            b',' => self.produce(Token::Comma),
            b'=' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::Equals)
                } else {
                    self.produce(Token::Assign)
                }
            }
            b'!' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::NotEquals)
                } else {
                    self.produce(Token::Not)
                }
            }
            b'@' => self.produce(Token::At),
            b'#' => self.produce(Token::Hash),
            b'$' => self.produce(Token::Dollar),
            b'%' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::PercentAssign)
                } else {
                    self.produce(Token::Percent)
                }
            }
            b'^' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::XorAssign)
                } else {
                    self.produce(Token::Xor)
                }
            }
            b'&' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::AndAssign)
                } else if self.read.next() == b'&' {
                    self.read.shift();
                    self.produce(Token::And)
                } else {
                    self.produce(Token::Ampersand)
                }
            }
            b'|' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::OrAssign)
                } else if self.read.next() == b'|' {
                    self.read.shift();
                    self.produce(Token::Or)
                } else {
                    self.produce(Token::Pipe)
                }
            }
            b'*' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::TimesAssign)
                } else {
                    self.produce(Token::Times)
                }
            }
            b'/' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::DivAssign)
                } else {
                    self.produce(Token::Div)
                }
            }
            b'-' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::MinusAssign)
                } else if self.read.next() == b'-' {
                    self.read.shift();
                    self.produce(Token::MinusMinus)
                } else if self.read.next() == b'>' {
                    self.read.shift();
                    self.produce(Token::Pointer)
                } else {
                    self.produce(Token::Minus)
                }
            }
            b'+' => {
                if self.read.next() == b'=' {
                    self.read.shift();
                    self.produce(Token::PlusAssign)
                } else if self.read.next() == b'+' {
                    self.read.shift();
                    self.produce(Token::PlusPlus)
                } else {
                    self.produce(Token::Plus)
                }
            }
            b'\0' => self.produce(Token::Eof),
            _ => self.produce(Token::Error(self.read.current() as char, self.read.span())),
        };
        let end = self.read.span();

        (ty, (start, end))
    }

    fn trim_spaces(&mut self) {
        while self.read.current().is_ascii_whitespace() {
            self.read.shift();
        }
    }

    fn trim_comments(&mut self) {
        if self.read.current() != b'/' { return; }

        if self.read.next() == b'/' {
            self.read.shift_multiple(2);
            while self.read.current() != b'\n' && self.read.current() != 0 {
                self.read.shift();
            }

            self.trim_spaces();
            self.trim_comments();
        } else if self.read.next() == b'*' {
            self.read.shift_multiple(2);

            loop {
                if self.read.current() == 0 { break; }
                if self.read.current() == b'*' && self.read.next() == b'/' {
                    // Skip the */
                    self.read.shift_multiple(2);
                    break;
                }
                self.read.shift();
            }

            self.trim_spaces();
            self.trim_comments();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut id = String::new();

        loop {
            let c = self.read.current() as char;
            if let 'a'..='z' | 'A'..='Z' | '_' = c {
                id.push(c);
                self.read.shift();
            } else {
                break;
            }
        }

        Self::identifier_to_token(id)
    }

    fn identifier_to_token(id: String) -> Token {
        match id.as_str() {
            "auto" => Token::Auto,
            "break" => Token::Break,
            "case" => Token::Case,
            "char" => Token::Char,
            "const" => Token::Const,
            "continue" => Token::Continue,
            "default" => Token::Default,
            "do" => Token::Do,
            "double" => Token::Double,
            "else" => Token::Else,
            "enum" => Token::Enum,
            "extern" => Token::Extern,
            "float" => Token::Float,
            "for" => Token::For,
            "goto" => Token::Goto,
            "if" => Token::If,
            "int" => Token::Int,
            "long" => Token::Long,
            "register" => Token::Register,
            "return" => Token::Return,
            "short" => Token::Short,
            "signed" => Token::Signed,
            "sizeof" => Token::Sizeof,
            "static" => Token::Static,
            "struct" => Token::Struct,
            "switch" => Token::Switch,
            "typedef" => Token::Typedef,
            "union" => Token::Union,
            "unsigned" => Token::Unsigned,
            "void" => Token::Void,
            "volatile" => Token::Volatile,
            "while" => Token::While,
            _ => Token::Identifier(id)
        }
    }

    fn read_number(&mut self) -> Token {
        let mut id = String::new();

        if self.read.current() == b'0' {
            self.read.shift();
            if let b'0'..=b'9' = self.read.current() {
                // octal number or decimal starting at 0
                id.push('0');
                self.read_digits(&mut id);

                if self.read.current() == b'.' {
                    id.push('.');
                    self.read.shift();
                    self.read_digits(&mut id);
                    self.read_exponent(&mut id);

                    if let b'f' | b'F' | b'l' | b'L' = self.read.current() {
                        self.read.shift();
                    }
                    return Token::FloatingLiteral(id);
                } else {
                    while let b'u' | b'U' | b'l' | b'L' = self.read.current() {
                        self.read.shift();
                    }
                    return Token::IntegerLiteral(id);
                }
            } else if self.read.current() == b'x' || self.read.current() == b'X' {
                // hex number
                id.push('0');
                id.push('x');
                self.read.shift();

                loop {
                    let c = self.read.current();
                    if let b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' = c {
                        id.push((c as char).to_ascii_uppercase());
                        self.read.shift();
                    } else {
                        break;
                    }
                }
                while let b'u' | b'U' | b'l' | b'L' = self.read.current() {
                    self.read.shift();
                }
                return Token::IntegerLiteral(id);
            } else if self.read.current() == b'.' {
                // decimal number
                id.push('0');
                id.push('.');
                self.read.shift();
                self.read_digits(&mut id);
                self.read_exponent(&mut id);

                if let b'f' | b'F' | b'l' | b'L' = self.read.current() {
                    self.read.shift();
                }
                return Token::FloatingLiteral(id);
            } else {
                // just zero
                while let b'u' | b'U' | b'l' | b'L' = self.read.current() {
                    self.read.shift();
                }
                return Token::IntegerLiteral("0".to_string());
            }
        } else if self.read.current() == b'.' {
            id.push('0');
            id.push('.');
            self.read.shift();
            self.read_digits(&mut id);
            self.read_exponent(&mut id);

            if let b'f' | b'F' | b'l' | b'L' = self.read.current() {
                self.read.shift();
            }
            return Token::FloatingLiteral(id);
        } else {
            self.read_digits(&mut id);

            if self.read.current() == b'.' {
                id.push('.');
                self.read.shift();
                self.read_digits(&mut id);
                self.read_exponent(&mut id);

                if let b'f' | b'F' | b'l' | b'L' = self.read.current() {
                    self.read.shift();
                }
                return Token::FloatingLiteral(id);
            } else if self.read.current() == b'e' || self.read.current() == b'E' {
                id.push('.');
                id.push('0');
                self.read_exponent(&mut id);

                if let b'f' | b'F' | b'l' | b'L' = self.read.current() {
                    self.read.shift();
                }
                return Token::FloatingLiteral(id);
            } else {
                while let b'u' | b'U' | b'l' | b'L' = self.read.current() {
                    self.read.shift();
                }
                return Token::IntegerLiteral(id);
            }
        }
    }

    fn read_digits(&mut self, id: &mut String) -> u32 {
        let mut count = 0;
        loop {
            let c = self.read.current();
            if !c.is_ascii_digit() { break; }
            id.push(c as char);
            self.read.shift();
            count += 1;
        }

        count
    }

    fn read_exponent(&mut self, id: &mut String) {
        if self.read.current() != b'e' && self.read.current() != b'E' {
            return;
        }

        if id.as_bytes()[id.len() - 1] == b'.' {
            id.push('0');
        }

        id.push('e');
        self.read.shift();

        if self.read.current() == b'+' || self.read.current() == b'-' {
            id.push(self.read.current() as char);
            self.read.shift();
        } else {
            id.push('+');
        }

        self.read_digits(id);
    }

    fn read_string(&mut self) -> Token {
        let mut content = String::new();
        // First "
        self.read.shift();

        loop {
            match self.read.current() {
                b'"' => {
                    self.read.shift();
                    break;
                }
                b'\\' => {
                    self.read.shift();
                    let value = match self.read.current() {
                        b'0' => 0,
                        b'n' => b'\n',
                        b't' => b'\t',
                        b'r' => b'\r',
                        c => c
                    };
                    content.push(value as char);
                }
                c => content.push(c as char)
            }
            self.read.shift();
        }

        Token::StringLiteral(content)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::CodeSource;

    use super::*;

    #[test]
    fn test_next_token() {
        let source = CodeSource::str("
            //
            // Created by cout970 on 12/8/19.
            //

            int main(/* Test */int i) {
               return 0;
            }

            ");

        let reader = SourceReader::new(source);
        let mut tokenizer = Tokenizer::new(reader);

        assert_eq!(Token::Int, tokenizer.next_tk());   // int
        assert_eq!(Token::Identifier("main".to_string()), tokenizer.next_tk());   // main
        assert_eq!(Token::LeftParen, tokenizer.next_tk());    // (
        assert_eq!(Token::Int, tokenizer.next_tk());   // int
        assert_eq!(Token::Identifier("i".to_string()), tokenizer.next_tk());   // i
        assert_eq!(Token::RightParen, tokenizer.next_tk());   // )
        assert_eq!(Token::LeftBrace, tokenizer.next_tk());    // {
        assert_eq!(Token::Return, tokenizer.next_tk());   // return
        assert_eq!(Token::IntegerLiteral("0".to_string()), tokenizer.next_tk());       // 0
        assert_eq!(Token::Semicolon, tokenizer.next_tk());    // ;
        assert_eq!(Token::RightBrace, tokenizer.next_tk());   // }
        assert_eq!(Token::Eof, tokenizer.next_tk());          // EOF
    }

    #[test]
    fn test_constants() {
        let source = CodeSource::str("\
        0x0123456789abcdef\n     0x0123456789ABCDEF
        0XABC\n                  0x012345L\n        0x012345l\n      0x012345U
        0x012345u\n              0x012345uL\n       0x012345ul\n     0x012345Ul
        0x012345ul\n             01234567\n         01234567ul\n     01234567LU
        123456e+123f\n           123456e10\n        123456E+123f\n   123456e-123l
        123456e-123\n            0123456.1325\n     .123\n           .123f
        .123e123l\n              123.e12\n          123.e12F
        ");

        let reader = SourceReader::new(source);
        let mut tokenizer = Tokenizer::new(reader);

        assert_eq!(Token::IntegerLiteral("0x0123456789ABCDEF".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x0123456789ABCDEF".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0xABC".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("0x012345".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("01234567".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("01234567".to_string()), tokenizer.next_tk());
        assert_eq!(Token::IntegerLiteral("01234567".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123456.0e+123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123456.0e+10".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123456.0e+123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123456.0e-123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123456.0e-123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("0123456.1325".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("0.123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("0.123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("0.123e+123".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123.0e+12".to_string()), tokenizer.next_tk());
        assert_eq!(Token::FloatingLiteral("123.0e+12".to_string()), tokenizer.next_tk());
    }

    #[test]
    fn test_string() {
        let source = CodeSource::str("\"Hello world\"\n\" \\t Test \\n \\\\ \"");
        let reader = SourceReader::new(source);
        let mut tokenizer = Tokenizer::new(reader);
        assert_eq!(Token::StringLiteral("Hello world".to_string()), tokenizer.next_tk());
        assert_eq!(Token::StringLiteral(" \t Test \n \\ ".to_string()), tokenizer.next_tk());
    }

    #[test]
    fn test_especial_tokens() {
        let source = CodeSource::str("\
            ... >>= <<= += -= *= /= %= &= ^= |= >> << ++ -- -> && || <= >= == != ;
            ({|<%) (}|%>) , : = ( ) ([|<:) (]|:>) . & ! ~ - + * / % < > ^ | ?");

        let reader = SourceReader::new(source);
        let mut tokenizer = Tokenizer::new(reader);
        loop {
            let tk = tokenizer.next_tk();
            println!("{:?}", tk);
            if tk == Token::Eof { break; }
        }
    }

    #[test]
    fn test() {
        let source = CodeSource::str(".123e123l");
        let reader = SourceReader::new(source);
        let mut tokenizer = Tokenizer::new(reader);
        assert_eq!(Token::FloatingLiteral(".123e+123".to_string()), tokenizer.next_tk());
    }

//    #[test]
//    fn test_next_token_text() {
//        let mut tok = Tokenizer::new(SourceCodeIterator::new("void main (int i) { return 0; }"));
//
//        let (_, span) = tok.next();
//        assert_eq!("void".to_string(), tok.get_text(span)); // void
//        let (_, span) = tok.next();
//        assert_eq!("main".to_string(), tok.get_text(span)); // main
//        let (_, span) = tok.next();
//        assert_eq!("(".to_string(), tok.get_text(span)); // (
//        let (_, span) = tok.next();
//        assert_eq!("int".to_string(), tok.get_text(span)); // int
//        let (_, span) = tok.next();
//        assert_eq!("i".to_string(), tok.get_text(span)); // i
//        let (_, span) = tok.next();
//        assert_eq!(")".to_string(), tok.get_text(span)); // )
//        let (_, span) = tok.next();
//        assert_eq!("{".to_string(), tok.get_text(span)); // {
//        let (_, span) = tok.next();
//        assert_eq!("return".to_string(), tok.get_text(span)); // return
//        let (_, span) = tok.next();
//        assert_eq!("0".to_string(), tok.get_text(span)); // 0
//        let (_, span) = tok.next();
//        assert_eq!(";".to_string(), tok.get_text(span)); // ;
//        let (_, span) = tok.next();
//        assert_eq!("}".to_string(), tok.get_text(span)); // }
//
//        assert_eq!(TkEof, tok.next().0); //
//    }
}