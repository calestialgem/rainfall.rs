use std::env;

mod otc;

fn main() {
    println!("Rainfall: Thrice C Transpiler");
    println!("Running with arguments:");
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("[{i}] `{arg}`");
    }
    println!();

    if args.len() != 2 {
        println!("Provide a single Thrice file!");
        return;
    }

    let src = otc::Source::of(args[1].clone());
    println!("Contents:\n{}", src.contents);
}
