#![allow(clippy::upper_case_acronyms)]

pub mod dfa;
pub mod fsm;
pub mod nfa;
pub mod parser;
pub mod regex;

use std::path::Path;

fn merge_alphabet(a: &[char], b: &[char]) -> Vec<char> {
    let mut set: Vec<char> = a.to_vec();
    for &c in b {
        if !set.contains(&c) {
            set.push(c);
        }
    }
    set.sort();
    set
}

fn compare_nfas(nfa1: &nfa::NFA, nfa2: &nfa::NFA, alphabet: &[char]) -> bool {
    let dfa1 = dfa::DFA::from_nfa(nfa1, alphabet);
    let dfa2 = dfa::DFA::from_nfa(nfa2, alphabet);
    let min1 = dfa1.minimize();
    let min2 = dfa2.minimize();
    min1.is_equivalent_to(&min2)
}

fn parse_input(input: &str) -> Result<nfa::NFA, String> {
    if Path::new(input).is_file() {
        fsm::parse_file(input)
    } else {
        let re = parser::Parser::parse(input).map_err(|e| e.message)?;
        Ok(nfa::NFA::from_regex(&re))
    }
}

fn input_label(input: &str) -> &str {
    if Path::new(input).is_file() {
        "文件"
    } else {
        "正则表达式"
    }
}

pub fn compare(input1: &str, input2: &str) -> Result<bool, String> {
    let nfa1 = parse_input(input1)?;
    let nfa2 = parse_input(input2)?;
    let alphabet = merge_alphabet(&nfa1.collect_alphabet(), &nfa2.collect_alphabet());
    Ok(compare_nfas(&nfa1, &nfa2, &alphabet))
}

pub fn compare_verbose(input1: &str, input2: &str) -> String {
    let nfa1 = match parse_input(input1) {
        Ok(nfa) => nfa,
        Err(e) => return format!("错误: {}1 解析失败: {}", input_label(input1), e),
    };
    let nfa2 = match parse_input(input2) {
        Ok(nfa) => nfa,
        Err(e) => return format!("错误: {}2 解析失败: {}", input_label(input2), e),
    };

    let alphabet = merge_alphabet(&nfa1.collect_alphabet(), &nfa2.collect_alphabet());
    let mut out = String::new();

    let label1 = input_label(input1);
    let label2 = input_label(input2);

    out.push_str(&format!("== {}1 ==\n输入: {}\n\n", label1, input1));
    out.push_str(&format!("== {}2 ==\n输入: {}\n\n", label2, input2));
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
