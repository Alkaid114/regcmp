use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::nfa::NFA;

fn err_msg(no: usize, msg: impl Into<String>) -> String {
    format!("第{}行: {}", no, msg.into())
}

fn err_suggest(no: usize, msg: impl Into<String>, suggest: impl Into<String>) -> String {
    format!("第{}行: {}\n  --> 建议: {}", no, msg.into(), suggest.into())
}

fn validate_name(name: &str, no: usize) -> Result<(), String> {
    if name.is_empty() {
        return Err(err_msg(no, "状态名为空"));
    }
    for c in name.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' => {}
            _ => {
                return Err(err_msg(
                    no,
                    format!("状态名 '{}' 包含非法字符 '{}'", name, c),
                ));
            }
        }
    }
    Ok(())
}

fn validate_sym(sym: &str, no: usize) -> Result<Option<char>, String> {
    match sym {
        "$" => Ok(None),
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            if c == '#' || c == '$' {
                Err(err_msg(no, format!("'{}' 是保留字符，不能用作符号", c)))
            } else {
                Ok(Some(c))
            }
        }
        s => Err(err_msg(no, format!("符号 '{}' 无效，应为单个字符或 $", s))),
    }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<NFA, String> {
    let content = fs::read_to_string(path.as_ref()).map_err(|e| format!("无法读取文件: {}", e))?;
    parse_str(&content)
}

pub fn parse_str(content: &str) -> Result<NFA, String> {
    enum Line {
        Start(String),
        Accept(Vec<String>),
        Trans(String, String, String),
    }

    let mut lines: Vec<Line> = Vec::new();

    for (i, raw) in content.lines().enumerate() {
        let no = i + 1;
        let l = raw.trim();

        if l.is_empty() || l.starts_with('#') {
            continue;
        }

        if l.contains('\t') {
            return Err(err_suggest(no, "包含制表符", "请使用空格"));
        }

        if let Some(rest) = l.strip_prefix("start:") {
            let name = rest.trim().to_string();
            if name.is_empty() {
                return Err(err_msg(no, "start: 后缺少状态名"));
            }
            validate_name(&name, no)?;
            lines.push(Line::Start(name));
            continue;
        }
        if l.starts_with("start ") || l.starts_with("start\t") {
            return Err(err_suggest(
                no,
                "start 后缺少冒号",
                format!("start: {}", &l[6..]),
            ));
        }

        if let Some(rest) = l.strip_prefix("accept:") {
            let names: Vec<&str> = rest.split_whitespace().collect();
            if names.is_empty() {
                return Err(err_msg(no, "accept: 后缺少状态名"));
            }
            let owned: Vec<String> = names.iter().map(|s| s.to_string()).collect();
            for n in &owned {
                validate_name(n, no)?;
            }
            lines.push(Line::Accept(owned));
            continue;
        }
        if l.starts_with("accept ") || l.starts_with("accept\t") {
            return Err(err_suggest(
                no,
                "accept 后缺少冒号",
                format!("accept: {}", &l[7..]),
            ));
        }

        if l.contains("->") {
            let parts: Vec<&str> = l.split("->").map(|s| s.trim()).collect();
            return Err(err_suggest(
                no,
                "不支持 '->' 箭头语法",
                format!("请用空格分隔: {}", parts.join(" ")),
            ));
        }

        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(err_suggest(
                no,
                format!("需要 3 个字段，当前 {} 个", parts.len()),
                "每行格式: 源 符号 目标，例如: q0 a q1",
            ));
        }
        if parts.len() > 3 {
            return Err(err_suggest(
                no,
                format!("存在多余字段 '{}'", parts[3..].join(" ")),
                "转移行只能有 3 个字段，多余内容应写在单独行",
            ));
        }

        validate_name(parts[0], no)?;
        validate_name(parts[2], no)?;
        validate_sym(parts[1], no)?;

        lines.push(Line::Trans(
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        ));
    }

    let mut state_map: HashMap<String, usize> = HashMap::new();
    let mut transitions: Vec<Vec<(Option<char>, usize)>> = Vec::new();
    let mut next_id = 0usize;

    macro_rules! state_id {
        ($name:expr) => {{
            let n = $name;
            if let Some(&id) = state_map.get(n) {
                id
            } else {
                let id = next_id;
                next_id += 1;
                state_map.insert(n.to_string(), id);
                transitions.push(Vec::new());
                id
            }
        }};
    }

    let mut start: Option<usize> = None;
    let mut accept_names: Vec<String> = Vec::new();

    for line in &lines {
        match line {
            Line::Start(n) => start = Some(state_id!(n)),
            Line::Accept(ns) => {
                for n in ns {
                    state_id!(n);
                }
                accept_names = ns.clone();
            }
            Line::Trans(src, _, dst) => {
                state_id!(src);
                state_id!(dst);
            }
        }
    }

    for line in &lines {
        if let Line::Trans(src, sym, dst) = line {
            let s = state_id!(src);
            let d = state_id!(dst);
            let symbol = if sym == "$" {
                None
            } else {
                Some(sym.chars().next().unwrap())
            };
            transitions[s].push((symbol, d));
        }
    }

    let start = start.unwrap_or(0);

    let accept = if accept_names.is_empty() {
        next_id
    } else {
        let a = next_id;
        next_id += 1;
        transitions.push(Vec::new());
        for name in &accept_names {
            if let Some(&id) = state_map.get(name) {
                transitions[id].push((None, a));
            }
        }
        a
    };

    Ok(NFA {
        start,
        accept,
        transitions,
        state_count: next_id,
    })
}
