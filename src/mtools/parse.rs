use std::{env, fmt::Error};

pub fn parser() -> Result<(String, String), Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => Ok((String::from(&args[1]), String::from(&args[2]))),
        _ => Err(Error),
    }
}
