use sass::Value;
use std::fmt;
use value::Quotes;
use variablescope::Scope;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SassString {
    parts: Vec<StringPart>,
    quotes: Quotes,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StringPart {
    Raw(String),
    Interpolation(Value),
}

impl SassString {
    pub fn new(parts: Vec<StringPart>, quotes: Quotes) -> Self {
        let mut p2 = vec![];
        let mut buf = String::new();
        for part in parts {
            match part {
                StringPart::Raw(s) => buf.push_str(&s),
                StringPart::Interpolation(v) => {
                    if !buf.is_empty() {
                        if buf.contains("\\#{") && !buf.contains("}") {
                            buf = buf.replace("\\#{", "#{");
                        }
                        p2.push(StringPart::Raw(buf.clone()));
                        buf.clear();
                    }
                    p2.push(StringPart::Interpolation(v));
                }
            }
        }
        if !buf.is_empty() {
            if buf.contains("\\#{") && !buf.contains("}") {
                buf = buf.replace("\\#{", "#{");
            }
            p2.push(StringPart::Raw(buf));
        }
        SassString { parts: p2, quotes }
    }
    pub fn evaluate(&self, scope: &Scope) -> (String, Quotes) {
        // Note This is an extremely peculiar special case;
        // A single-quoted string consisting only of an interpolation
        // becomes double-quoted.
        if self.quotes == Quotes::Single && self.parts.len() == 1 {
            if let &StringPart::Interpolation(ref v) = &self.parts[0] {
                return (format!("{}", v.evaluate(scope).unquote()),
                        Quotes::Double);
            }
        }
        let mut result = String::new();
        for part in &self.parts {
            match *part {
                StringPart::Interpolation(ref v) => {
                    result.push_str(&format!("{}", v.evaluate(scope).unquote()))
                }
                StringPart::Raw(ref s) => result.push_str(s),
            }
        }
        (result, self.quotes)
    }
    pub fn evaluate2(&self, scope: &Scope) -> SassString {
        let (result, quotes) = self.evaluate(scope);
        SassString {
            parts: vec![StringPart::Raw(result)],
            quotes: quotes,
        }
    }
    pub fn is_unquoted(&self) -> bool {
        self.quotes == Quotes::None
    }
    pub fn single_raw<'a>(&'a self) -> Option<&'a str> {
        if self.parts.len() == 1 {
            if let StringPart::Raw(ref s) = self.parts[0] {
                return Some(&s);
            }
        }
        None
    }
    pub fn append(&mut self, other: &Self) {
        self.parts.extend_from_slice(&other.parts);
    }
}

impl fmt::Display for SassString {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.quotes.fmt(out)?;
        for part in &self.parts {
            match *part {
                StringPart::Raw(ref s) => s.fmt(out)?,
                StringPart::Interpolation(ref v) => {
                    panic!("Interpolation should be evaluated: {:?}", v)
                }
            }
        }
        self.quotes.fmt(out)
    }
}

impl<'a> From<&'a str> for SassString {
    fn from(s: &'a str) -> Self {
        SassString {
            parts: vec![StringPart::Raw(s.to_string())],
            quotes: Quotes::None,
        }
    }
}