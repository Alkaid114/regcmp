use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Regex {
    EmptySet,
    EmptyStr,
    Char(char),
    Union(Box<Regex>, Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
    Star(Box<Regex>),
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Regex::EmptySet => write!(f, "#"),
            Regex::EmptyStr => write!(f, "$"),
            Regex::Char(c) => write!(f, "{}", c),
            Regex::Union(l, r) => write!(f, "({}|{})", l, r),
            Regex::Concat(l, r) => write!(f, "({}{})", l, r),
            Regex::Star(r) => write!(f, "({}*)", r),
        }
    }
}

impl Regex {
    pub fn collect_alphabet(&self, set: &mut Vec<char>) {
        match self {
            Regex::Char(c) if !set.contains(c) => {
                set.push(*c);
            }
            Regex::Union(l, r) | Regex::Concat(l, r) => {
                l.collect_alphabet(set);
                r.collect_alphabet(set);
            }
            Regex::Star(r) => r.collect_alphabet(set),
            _ => {}
        }
    }
}
