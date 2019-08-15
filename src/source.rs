use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, stdin};

pub enum CodeSource {
    File { path: String, offset: usize },
    Str { code: &'static str, offset: usize },
    Stdin,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span {
    line: u32,
    column: u32,
}

const BUFFER_SIZE: usize = 64;
const LOOKAHEAD_AMOUNT: usize = 3;

pub struct SourceReader {
    source: CodeSource,
    lookahead: VecDeque<u8>,
    buffer: [u8; BUFFER_SIZE],
    count: usize,
    pos: usize,
    span: Span,
    eof: bool,
}

impl SourceReader {
    pub fn new(source: CodeSource) -> Self {
        let mut this = Self {
            source,
            lookahead: VecDeque::with_capacity(LOOKAHEAD_AMOUNT),
            buffer: [0; BUFFER_SIZE],
            count: 0,
            pos: 0,
            span: Span { line: 1, column: 1 },
            eof: false,
        };
        this.fill_lookahead();
        this
    }

    fn fill_buffer(&mut self) {
        if self.eof { return; }
        let buff: &mut [u8] = &mut self.buffer;

        let line_size = match &mut self.source {
            CodeSource::File { path, offset } => {
                let mut f = File::open(path).unwrap();
                f.seek(SeekFrom::Start(*offset as _)).unwrap();
                let line_size = f.read(buff).unwrap();
                *offset += line_size;
                line_size
            }
            CodeSource::Str { code, offset } => {
                let remaining: &[u8] = &code.as_bytes()[*offset..];
                let mut line_size = 0;
                for (dst, src) in buff.iter_mut().zip(remaining) {
                    *dst = *src;
                    line_size += 1;
                }
                *offset += line_size;
                line_size
            }
            CodeSource::Stdin => {
                stdin().read(buff).unwrap()
            }
        };

        self.count = line_size;

        if line_size == 0 {
            self.eof = true;
        }
    }

    pub fn shift_multiple(&mut self, amount: usize) {
        if self.eof { return; }

        for _ in 0..amount {
            self.shift();
        }
    }

    fn fill_lookahead(&mut self) {
        while self.lookahead.len() < LOOKAHEAD_AMOUNT {
            if self.pos >= self.count {
                self.fill_buffer();
                self.pos = 0;
            }

            if !self.eof {
                self.lookahead.push_back(self.buffer[self.pos]);
                self.pos += 1;
            } else {
                self.lookahead.push_back(0);
            }
        }
    }

    pub fn shift(&mut self) {
        if !self.eof {
            if self.current() == b'\n' {
                self.span.line += 1;
                self.span.column = 1;
            } else {
                self.span.column += 1;
            }
        }

        self.lookahead.pop_front();
        self.fill_lookahead();
    }

    pub fn current(&self) -> u8 {
        self.lookahead[0]
    }

    pub fn next(&self) -> u8 {
        self.lookahead[1]
    }

    pub fn next_next(&self) -> u8 {
        self.lookahead[2]
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl CodeSource {
    pub fn stdin() -> Self {
        CodeSource::Stdin
    }

    pub fn file(path: &str) -> Self {
        CodeSource::File { path: path.to_string(), offset: 0 }
    }

    pub fn str(code: &'static str) -> Self {
        CodeSource::Str { code, offset: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_read() {
        let source = CodeSource::file("test.c");

        let mut reader = SourceReader::new(source);

        loop {
            let c = reader.current();
            println!("{:?} => {:?}", c as char, reader.span());
            if c == 0 { break; }
            reader.shift();
        }
    }

    #[test]
    fn test_str_read() {
        let source = CodeSource::str("\
            123456789012345678901234567890123456789012345678901234567890123456789012345\n\
            6789012345678901234567890123456789012345678901234567890123456789012345678901234567\n\
            8901234567890123456789012345678901234567890123456789012345678901234567890123456789\n"
        );

        let mut reader = SourceReader::new(source);

        loop {
            let c = reader.current();
            println!("{:?} => {:?}", c as char, reader.span());
            if c == 0 { break; }
            reader.shift();
        }
    }
}