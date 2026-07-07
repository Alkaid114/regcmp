use clap::Parser;
use colored::Colorize;

mod dfa;
mod nfa;
mod parser;
mod regex;

#[derive(Parser)]
#[command(name = "regcmp", about = "比较两个形式正则表达式是否等价")]
struct Cli {
    /// 第一个正则表达式
    regex1: String,
    /// 第二个正则表达式
    regex2: String,
    /// 输出详细中间结果
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let re1 = match parser::Parser::parse(&cli.regex1) {
        Ok(re) => re,
        Err(e) => {
            print_error("正则表达式1", &cli.regex1, &e);
            std::process::exit(1);
        }
    };

    let re2 = match parser::Parser::parse(&cli.regex2) {
        Ok(re) => re,
        Err(e) => {
            print_error("正则表达式2", &cli.regex2, &e);
            std::process::exit(1);
        }
    };

    if cli.verbose {
        println!("{}", "== 正则表达式1 ==".cyan().bold());
        println!("输入: {}", cli.regex1.yellow());
        println!("AST : {}", re1.to_string().yellow());
        println!();

        println!("{}", "== 正则表达式2 ==".cyan().bold());
        println!("输入: {}", cli.regex2.yellow());
        println!("AST : {}", re2.to_string().yellow());
        println!();
    }

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

    if cli.verbose {
        println!("{}", "== 字母表 ==".cyan().bold());
        println!("{:?}", alphabet);
        println!();

        println!("{}", "== NFA1 ==".cyan().bold());
        nfa1.dump();
        println!();

        println!("{}", "== NFA2 ==".cyan().bold());
        nfa2.dump();
        println!();
    }

    let dfa1 = dfa::DFA::from_nfa(&nfa1, &alphabet);
    let dfa2 = dfa::DFA::from_nfa(&nfa2, &alphabet);

    if cli.verbose {
        println!("{}", "== DFA1 ==".cyan().bold());
        dfa1.dump();
        println!();

        println!("{}", "== DFA2 ==".cyan().bold());
        dfa2.dump();
        println!();
    }

    let min1 = dfa1.minimize();
    let min2 = dfa2.minimize();

    if cli.verbose {
        println!("{}", "== 最小化DFA1 ==".cyan().bold());
        min1.dump();
        println!();

        println!("{}", "== 最小化DFA2 ==".cyan().bold());
        min2.dump();
        println!();
    }

    let equivalent = min1.is_equivalent_to(&min2);

    if cli.verbose {
        println!("{}", "== 结论 ==".cyan().bold());
    }

    if equivalent {
        println!("{}", "等价".green().bold());
    } else {
        println!("{}", "不等价".red().bold());
    }
}

fn print_error(label: &str, input: &str, err: &parser::ParseError) {
    let pos = err.pos.min(input.len());

    let line_start = input[..pos].rfind('\n').map_or(0, |i| i + 1);
    let line_end = input[pos..]
        .find('\n')
        .map_or(input.len(), |i| pos + i);
    let line_content = &input[line_start..line_end];
    let col = pos - line_start;
    let line_no = input[..pos].chars().filter(|&c| c == '\n').count() + 1;

    let pipe = "|".blue();
    let pad = " ".repeat(line_no.to_string().len());

    eprintln!(
        "{}: {}",
        "错误".red().bold(),
        format!("{} 解析失败: {}", label, err.message).red()
    );
    eprintln!("{}", format!("{pad} {pipe}").blue());
    eprintln!(
        "{}",
        format!("{line_no} {pipe} {line_content}").blue()
    );
    eprintln!(
        "{} {}",
        format!("{pad} {pipe}").blue(),
        format!("{}{}", " ".repeat(col), "^").red().bold()
    );
}
