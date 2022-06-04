use std::str::FromStr;

#[derive(Debug)]
struct Arguments {
    kadai: u32,
    sikenratio: u32,
}

// cargo run 課題点 試験の割合
// と入力する
fn main() {
    let args = parse_args();
    println!(
        "あなたは試験で{}点を取る必要があります．",
        cani(args.kadai, args.sikenratio)
    );
}

fn cani(kadai: u32, sikenratio: u32) -> u32 {
    ((60 - kadai) * 10) / sikenratio
}

fn parse_args() -> Arguments {
    let mut args = Vec::new();

    for arg in std::env::args().skip(1) {
        args.push(u32::from_str(&arg).expect("error parsing arguments"));
    }

    if args.len() != 2 {
        eprintln!(
            "Error: wrong number of argsments: expected 2, got {}",
            args.len()
        );
        std::process::exit(1);
    };

    Arguments {
        kadai: args[0],
        sikenratio: args[1],
    }
}
