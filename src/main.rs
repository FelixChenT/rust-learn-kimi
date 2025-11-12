mod lessons;

use std::env;

fn print_help() {
    eprintln!("Usage:");
    eprintln!("  cargo run -- list");
    eprintln!("  cargo run -- <lesson>");
    eprintln!("");
    eprintln!("Examples:");
    eprintln!("  cargo run -- list           # 列出所有 lessons");
    eprintln!("  cargo run -- 01_hello_world # 运行指定 lesson");
    eprintln!("  cargo run -- 1              # 通过编号运行 lesson");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_help();
        return;
    }

    match args[0].as_str() {
        "list" => lessons::list(),
        sel => {
            if let Err(e) = lessons::run_selected(sel) {
                eprintln!("Error: {}", e);
                print_help();
                std::process::exit(1);
            }
        }
    }
}
