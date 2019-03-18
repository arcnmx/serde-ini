use std::{io, fmt, error, str};
use result::prelude::*;
use void::Void;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Item {
    Empty,
    Section {
        name: String
    },
    Value {
        key: String,
        value: String,
    },
    Comment {
        text: String
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SyntaxError {
    SectionNotClosed,
    SectionName,
    MissingEquals,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyntaxError::SectionNotClosed => write!(f, "section missing ']'"),
            SyntaxError::SectionName => write!(f, "section name contains ']'"),
            SyntaxError::MissingEquals => write!(f, "variable assignment missing '='"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Error<E> {
    Inner(E),
    Syntax(SyntaxError),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Inner(e)
    }
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Inner(ref e) => fmt::Display::fmt(e, f),
            Error::Syntax(s) => write!(f, "INI syntax error: {}", s),
        }
    }
}

impl<E: error::Error> error::Error for Error<E> {
    fn description(&self) -> &str {
        match *self {
            Error::Inner(ref e) => e.description(),
            Error::Syntax(..) => "INI syntax error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Inner(ref e) => Some(e),
            _ => None,
        }
    }
}

pub struct Parser<T> {
    input: T,
}

impl<T> Parser<T> {
    pub fn new(input: T) -> Self {
        Parser {
            input: input,
        }
    }

    pub fn into_inner(self) -> T {
        self.input
    }
}

impl<'a> Parser<OkIter<str::Lines<'a>>> {
    pub fn from_str(s: &'a str) -> Self {
        Self::new(OkIter(s.lines()))
    }
}

impl<R: io::BufRead> Parser<io::Lines<R>> {
    pub fn from_bufread(r: R) -> Self {
        Self::new(r.lines())
    }
}

impl<R: io::Read> Parser<io::Lines<io::BufReader<R>>> {
    pub fn from_read(r: R) -> Self {
        Self::from_bufread(io::BufReader::new(r))
    }
}

impl<T> Parser<T> {
    fn parse_next<E, S: AsRef<str>>(line: Option<S>) -> Result<Option<Item>, Error<E>> {
        let line = match line {
            Some(line) => line,
            None => return Ok(None),
        };
        let line = line.as_ref();

        if line.starts_with('[') {
            if line.ends_with(']') {
                let line = &line[1..line.len() - 1];
                if line.contains(']') {
                    Err(Error::Syntax(SyntaxError::SectionName))
                } else {
                    Ok(Some(Item::Section {
                        name: line.into(),
                    }))
                }
            } else {
                Err(Error::Syntax(SyntaxError::SectionNotClosed))
            }
        } else if line.starts_with(';') || line.starts_with('#') {
            Ok(Some(Item::Comment {
                text: line.into(),
            }))
        } else {
            let mut line = line.splitn(2, '=');
            if let Some(key) = line.next() {
                if let Some(value) = line.next() {
                    Ok(Some(Item::Value {
                        key: key.trim().into(),
                        value: value.trim().into(),
                    }))
                } else if key.is_empty() {
                    Ok(Some(Item::Empty))
                } else {
                    Err(Error::Syntax(SyntaxError::MissingEquals))
                }
            } else {
                unreachable!()
            }
        }
    }
}

impl<E, S: AsRef<str>, T: Iterator<Item=Result<S, E>>> Iterator for Parser<T> {
    type Item = Result<Item, Error<E>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next_invert().map_err(Error::Inner).and_then(|l| Self::parse_next(l)).invert()
    }
}

pub struct OkIter<I>(pub I);

impl<T, I: Iterator<Item=T>> Iterator for OkIter<I> {
    type Item = Result<T, Void>;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0).next().map(Ok)
    }
}
