#![allow(clippy::upper_case_acronyms)]
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "regcmp", about = "比较两个正则表达式或自动机是否等价")]
struct Cli {
    /// 第一个输入（正则表达式或文件路径）
    input1: String,
    /// 第二个输入（正则表达式或文件路径）
    input2: String,
    /// 输出详细中间结果
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        let out = regcmp::compare_verbose(&cli.input1, &cli.input2);
        if out.starts_with("错误:") {
            eprintln!("{}", out.red());
            std::process::exit(1);
        }
        let lines: Vec<&str> = out.trim_end().lines().collect();
        if lines.len() > 1 {
            println!("{}", lines[..lines.len() - 1].join("\n"));
        }
        if let Some(&last) = lines.last() {
            if last == "等价" {
                println!("{}", last.green().bold());
            } else {
                println!("{}", last.red().bold());
            }
        }
    } else {
        match regcmp::compare(&cli.input1, &cli.input2) {
            Ok(true) => println!("{}", "等价".green().bold()),
            Ok(false) => println!("{}", "不等价".red().bold()),
            Err(e) => {
                eprintln!("{}", format!("错误: {}", e).red());
                std::process::exit(1);
            }
        }
    }
}
