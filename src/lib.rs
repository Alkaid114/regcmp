#![allow(clippy::upper_case_acronyms)]

pub mod dfa;
pub mod nfa;
pub mod parser;
pub mod regex;

pub fn compare(regex1: &str, regex2: &str) -> Result<bool, String> {
    let re1 = parser::Parser::parse(regex1).map_err(|e| e.message)?;
    let re2 = parser::Parser::parse(regex2).map_err(|e| e.message)?;

    let nfa1 = nfa::NFA::from_regex(&re1);
    let nfa2 = nfa::NFA::from_regex(&re2);

    let mut alphabet = Vec::new();
    re1.collect_alphabet(&mut alphabet);
    let mut alpha2 = Vec::new();
    re2.collect_alphabet(&mut alpha2);
    for c in alpha2 {
        if !alphabet.contains(&c) {
            alphabet.push(c);
        }
    }
    alphabet.sort();

    let dfa1 = dfa::DFA::from_nfa(&nfa1, &alphabet);
    let dfa2 = dfa::DFA::from_nfa(&nfa2, &alphabet);

    let min1 = dfa1.minimize();
    let min2 = dfa2.minimize();

    Ok(min1.is_equivalent_to(&min2))
}
