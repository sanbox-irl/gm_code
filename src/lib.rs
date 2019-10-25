pub mod config;
pub mod expressions;
pub mod lang_config;
pub mod lex_token;
pub mod parser;
pub mod scanner;
pub mod statements;

pub use config::{Config, PrintFlags};
use lang_config::LangConfig;
use parser::Parser;
use std::{error::Error, fs};

pub fn run_config(config: &Config, lang_config: &LangConfig) -> Result<(), Box<dyn Error>> {
    let log = config.print_flags.contains(PrintFlags::LOGS);
    // let log_scan = config.print_flags.contains(PrintFlags::SCANNER_LOGS);
    let overwrite = config.print_flags.contains(PrintFlags::OVERWRITE);

    for this_file in &config.files {
        let contents = fs::read_to_string(this_file)?;

        if contents.contains("// @gml_fmt ignore") {
            continue;
        }

        if log {
            println!("=========INPUT=========");
            println!("{}", contents);
        }

        let res = run(&contents, lang_config, log);
        if let Some(err) = res {
            println!("Could not parse file {:?}", this_file);
            println!("{}", err);
        }
    }

    Ok(())
}

pub fn run_test(input: &str) -> String {
    let res = run(input, &LangConfig::default(), false);
    if let Some(err) = res {
        println!("{}", err);
        return input.to_owned();
    }
    return res.unwrap();
}

fn run(source: &str, lang_config: &LangConfig, print_ast: bool) -> Option<String> {
    let source_size = source.len();
    let mut parser = Parser::new(source);
    parser.build_ast();
    parser.failure
}
