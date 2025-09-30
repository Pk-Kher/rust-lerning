use std::{env, process};

use minigrep::{Config, run};

fn main() {
    // let args: Vec<String> = .collect();
    let data=env::args();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = run(config) {
        eprintln!("application problem: {}", err);
        process::exit(1);
    }
    // dbg!()
}
// pub fn search<'a>(query:&str, content:&str) ->Vec<&'a str>  {
//     let data=content.spli
//     content;
//     vec![]
// }
