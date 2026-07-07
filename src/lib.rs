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

pub fn compare_verbose(regex1: &str, regex2: &str) -> String {
    let re1 = match parser::Parser::parse(regex1) {
        Ok(re) => re,
        Err(e) => return format!("错误: 正则表达式1 解析失败: {}", e.message),
    };
    let re2 = match parser::Parser::parse(regex2) {
        Ok(re) => re,
        Err(e) => return format!("错误: 正则表达式2 解析失败: {}", e.message),
    };

    let mut out = String::new();

    out.push_str(&format!(
        "== 正则表达式1 ==\n输入: {}\nAST : {}\n\n",
        regex1, re1
    ));

    out.push_str(&format!(
        "== 正则表达式2 ==\n输入: {}\nAST : {}\n\n",
        regex2, re2
    ));

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

    out.push_str(&format!("== 字母表 ==\n{:?}\n\n", alphabet));

    out.push_str(&format!("== NFA1 ==\n{}\n\n", nfa1.display_string()));
    out.push_str(&format!("== NFA2 ==\n{}\n\n", nfa2.display_string()));

    let dfa1 = dfa::DFA::from_nfa(&nfa1, &alphabet);
    let dfa2 = dfa::DFA::from_nfa(&nfa2, &alphabet);

    out.push_str(&format!("== DFA1 ==\n{}\n\n", dfa1.display_string()));
    out.push_str(&format!("== DFA2 ==\n{}\n\n", dfa2.display_string()));

    let min1 = dfa1.minimize();
    let min2 = dfa2.minimize();

    out.push_str(&format!("== 最小化DFA1 ==\n{}\n\n", min1.display_string()));
    out.push_str(&format!("== 最小化DFA2 ==\n{}\n\n", min2.display_string()));

    out.push_str("== 结论 ==\n");
    if min1.is_equivalent_to(&min2) {
        out.push_str("等价\n");
    } else {
        out.push_str("不等价\n");
    }

    out
}
