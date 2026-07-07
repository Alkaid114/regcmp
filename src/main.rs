use clap::Parser;

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
            eprintln!("解析正则表达式1失败: {}", e);
            std::process::exit(1);
        }
    };

    let re2 = match parser::Parser::parse(&cli.regex2) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("解析正则表达式2失败: {}", e);
            std::process::exit(1);
        }
    };

    if cli.verbose {
        println!("=== 正则表达式1 ===");
        println!("输入: {}", cli.regex1);
        println!("AST: {}", re1);
        println!();

        println!("=== 正则表达式2 ===");
        println!("输入: {}", cli.regex2);
        println!("AST: {}", re2);
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
        println!("=== 字母表 ===");
        println!("{:?}", alphabet);
        println!();

        println!("=== NFA1 ===");
        nfa1.dump();
        println!();

        println!("=== NFA2 ===");
        nfa2.dump();
        println!();
    }

    let dfa1 = dfa::DFA::from_nfa(&nfa1, &alphabet);
    let dfa2 = dfa::DFA::from_nfa(&nfa2, &alphabet);

    if cli.verbose {
        println!("=== DFA1 ===");
        dfa1.dump();
        println!();

        println!("=== DFA2 ===");
        dfa2.dump();
        println!();
    }

    let min1 = dfa1.minimize();
    let min2 = dfa2.minimize();

    if cli.verbose {
        println!("=== 最小化DFA1 ===");
        min1.dump();
        println!();

        println!("=== 最小化DFA2 ===");
        min2.dump();
        println!();
    }

    let equivalent = min1.is_equivalent_to(&min2);

    if cli.verbose {
        println!("=== 结论 ===");
    }

    if equivalent {
        println!("等价");
    } else {
        println!("不等价");
    }
}
