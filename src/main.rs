// SPDX-FileCopyrightText: 2022 Cem Geçgel <gecgelcem@outlook.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use rainfall::Error;
use std::env;

/// Entry to the Rainfall compiler.
fn main() {
    // Get the arguments after the first one, which would be the path to the executable.
    let args: Vec<String> = env::args().collect();
    if let Err(error) = run(&args[1..]) {
        println!("{}", error);
    }
}

/// Run the compiler with the given arguments.
fn run(args: &[String]) -> Result<(), Error> {
    // Check arguments.
    if args.len() > 0 {
        return Err(Error::ArgumentError(String::from("Too many arguments!")));
    }
    Ok(())
}
